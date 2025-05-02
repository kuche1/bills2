mod cmdline;
mod parse_bill_toml;

use colored::*; // cargo add colored

enum Progress {
    Good,
    Bad,
}

macro_rules! print_columns_str {
    ($day:expr, $exp_day:expr, $mpd_adapt:expr, $mpd_adapt_progress:expr, $mpd_def:expr, $median:expr, $apl_med:expr) => {
        println!(
            " {} {} {} {} {} {}",
            $day.truecolor(200, 200, 200),   // .red(),
            $mpd_def.truecolor(150, 150, 0), // .yellow()
            $exp_day.truecolor(150, 20, 20), // .green(),
            match $mpd_adapt_progress {
                // .truecolor(100, 100, 255) //.blue()
                Progress::Good => $mpd_adapt.truecolor(100, 200, 100),
                Progress::Bad => $mpd_adapt.truecolor(200, 100, 100),
            },
            $median.truecolor(100, 100, 255),
            $apl_med
        );
    };
}

macro_rules! print_columns_num {
    ($day:expr, $exp_day:expr, $mpd_adapt:expr, $mpd_adapt_progress:expr, $mpd_def:expr, $median:expr, $apl_med:expr) => {
        print_columns_str!(
            format!("|{:2}|", $day),
            format!("|{:6.2}|", $exp_day),
            format!("|{:7.2}|", $mpd_adapt),
            $mpd_adapt_progress,
            format!("|{:4.2}|", $mpd_def),
            format!("|{:6.2}|", $median),
            format!("|{:7.2}|", $apl_med)
        );
    };
}

fn calc_median(expenditures: &[f32]) -> f32 {
    let mut ordered = expenditures.to_vec(); // I hate this, but whatever, we're working with just floats
    ordered.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let items_to_remove = ordered.len() / 4; // we'll remove the highest 1/4 and the lowest 1/4
    // println!("dbg: {}", ordered.len());
    ordered.drain(ordered.len() - items_to_remove..);
    // println!("dbg: {}", ordered.len());
    ordered.drain(..items_to_remove);
    // println!("dbg: {}", ordered.len());

    ordered.iter().sum::<f32>() / ordered.len() as f32
}

fn main() -> Result<(), String> {
    let (input_toml, year, month) = cmdline::parse()?;

    let (income, expenditures_monthly, expenditures_regular, days_in_month) =
        parse_bill_toml::main(input_toml, year, month)?;
    let days_in_month = days_in_month as f32;

    // TODO4? put in new module

    println!("MPD -> money-per-day");
    println!("EXP -> expenditures");
    println!("MAV -> median-average");

    print_columns_str!(
        "day",
        "EXP",
        "MPD-adaptive",
        Progress::Good,
        "MPD-default",
        "EXP-MAV",
        "MPDD-apl-EXPMAV"
    );

    // TODO0 names are getting complex, could be cool if we could color code each word

    let income = income - expenditures_monthly;

    let money_per_day_default = income / days_in_month;

    let mut money_left = income;

    let mut last_money_per_day_adaptive: f32 = 0.0;

    for (day, expenditure_day) in expenditures_regular.iter().enumerate() {
        let day = day + 1;
        let day_f32 = day as f32;

        let days_left = days_in_month - day_f32 + 1.0;

        let money_per_day_adaptive = money_left / days_left;
        let money_per_day_adaptive = money_per_day_adaptive - expenditure_day;

        money_left -= expenditure_day;

        let median = calc_median(&expenditures_regular[..day]);

        print_columns_num!(
            day,
            expenditure_day,
            money_per_day_adaptive,
            if last_money_per_day_adaptive > money_per_day_adaptive {
                Progress::Bad
            } else {
                Progress::Good
            },
            money_per_day_default,
            median,
            money_per_day_default - median
        );

        last_money_per_day_adaptive = money_per_day_adaptive;
    }

    Ok(())
}

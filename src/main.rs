mod cmdline;
mod parse_bill_toml;

use colored::*; // cargo add colored

enum Progress {
    Good,
    Bad,
}

macro_rules! print_columns_str {
    ($col_day:expr, $col_exp_day:expr, $col_mpd_adapt:expr, $col_mpd_adapt_progress:expr, $col_mpd_def:expr) => {
        println!(
            "| {} | {} | {} | {} |",
            $col_day.truecolor(200, 200, 200),   // .red(),
            $col_mpd_def.truecolor(150, 150, 0), // .yellow()
            $col_exp_day.truecolor(150, 20, 20), // .green(),
            match $col_mpd_adapt_progress {
                // .truecolor(100, 100, 255) //.blue()
                Progress::Good => $col_mpd_adapt.truecolor(100, 200, 100),
                Progress::Bad => $col_mpd_adapt.truecolor(200, 100, 100),
            }
        );
    };
}

macro_rules! print_columns_num {
    ($col_day:expr, $col_exp_day:expr, $col_mpd_adapt:expr, $col_mpd_adapt_progress:expr, $col_mpd_def:expr) => {
        print_columns_str!(
            format!("{:2}", $col_day),
            format!("{:6.2}", $col_exp_day),
            format!("{:7.2}", $col_mpd_adapt),
            $col_mpd_adapt_progress,
            format!("{:4.2}", $col_mpd_def)
        );
    };
}

fn main() -> Result<(), String> {
    let (input_toml, year, month) = cmdline::parse()?;

    let (income, expenditures_monthly, expenditures_regular, days_in_month) =
        parse_bill_toml::main(input_toml, year, month)?;
    let days_in_month = days_in_month as f32;

    // TODO4? put in new module

    println!("MPD -> money-per-day");

    print_columns_str!(
        "day",
        "expenditures",
        "MPD-adaptive",
        Progress::Good,
        "MPD-default"
    );

    // TODO0 also calculate median for each day

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

        print_columns_num!(
            day,
            expenditure_day,
            money_per_day_adaptive,
            if last_money_per_day_adaptive > money_per_day_adaptive {
                Progress::Bad
            } else {
                Progress::Good
            },
            money_per_day_default
        );

        last_money_per_day_adaptive = money_per_day_adaptive;
    }

    Ok(())
}

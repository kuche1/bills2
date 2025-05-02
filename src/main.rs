mod cmdline;
mod parse_bill_toml;

use colored::*; // cargo add colored

// enum Progress {
//     Good,
//     Bad,
// }

// macro_rules! print_columns_str {
//     ($day:expr, $exp_day:expr, $mpd_adapt:expr, $mpd_adapt_progress:expr, $mpd_def:expr, $median:expr, $apl_med:expr) => {
//         println!(
//             " {} {} {} {} {} {}",
//             $day.truecolor(200, 200, 200),
//             $mpd_def.truecolor(150, 150, 0), // .yellow()
//             $exp_day.truecolor(150, 20, 20), // .green(),
//             match $mpd_adapt_progress {
//                 // .truecolor(100, 100, 255) //.blue()
//                 Progress::Good => $mpd_adapt.truecolor(100, 200, 100),
//                 Progress::Bad => $mpd_adapt.truecolor(200, 100, 100),
//             },
//             $median.truecolor(100, 100, 210),
//             $apl_med.truecolor(255, 255, 0)
//         );
//     };
// }

// macro_rules! print_columns_num {
//     ($day:expr, $exp_day:expr, $mpd_adapt:expr, $mpd_adapt_progress:expr, $mpd_def:expr, $median:expr, $apl_med:expr) => {
//         print_columns_str!(
//             format!("|{:2}|", $day),
//             format!("|{:6.2}|", $exp_day),
//             format!("|{:7.2}|", $mpd_adapt),
//             $mpd_adapt_progress,
//             format!("|{:4.2}|", $mpd_def),
//             format!("|{:6.2}|", $median),
//             format!("|{:7.2}|", $apl_med)
//         );
//     };
// }

fn col_day(str: &str) -> String {
    format!("{}", str.truecolor(200, 200, 200)) // .red()
}

fn col_mpds(str: &str) -> String {
    format!("{}", str.truecolor(150, 150, 0))
}

fn col_mpda_good(str: &str) -> String {
    format!("{}", str.truecolor(100, 200, 100))
}

fn col_mpda_bad(str: &str) -> String {
    format!("{}", str.truecolor(200, 100, 100))
}

fn col_exp(str: &str) -> String {
    format!("{}", str.truecolor(150, 20, 20))
}

fn col_ema(str: &str) -> String {
    format!("{}", str.truecolor(100, 100, 210))
}

fn col_mpds_apl_ema(str: &str) -> String {
    format!("{}", str.truecolor(200, 150, 150))
}

fn calc_median(expenditures: &[f32]) -> f32 {
    let mut ordered = expenditures.to_vec(); // I hate this, but whatever, we're working with just floats
    ordered.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let items_to_remove = ordered.len() / 4; // we'll remove the highest 1/4 and the lowest 1/4
    ordered.drain(ordered.len() - items_to_remove..);
    ordered.drain(..items_to_remove);

    ordered.iter().sum::<f32>() / ordered.len() as f32
}

fn main() -> Result<(), String> {
    let (input_toml, year, month) = cmdline::parse()?;

    let (income, expenditures_monthly, expenditures_regular, days_in_month) =
        parse_bill_toml::main(input_toml, year, month)?;
    let days_in_month = days_in_month as f32;

    // TODO4 this shit needs to be contained in a new module

    println!("{}", col_day("day"));
    println!("{}", col_mpds("money-per-day-static"));
    println!("{}", col_exp("expenditures"));
    println!(
        "{} {}",
        col_mpda_good("money-per-day-adaptive good"),
        col_mpda_bad("bad")
    );
    println!("{}", col_ema("expenditures-median-average"));
    println!(
        "{}{}{}",
        col_mpds("MPDS"),
        col_mpds_apl_ema("-applied-to-"),
        col_ema("EMA")
    );
    println!();

    // TODO0 names are getting complex, could be cool if we could color code each word

    let income = income - expenditures_monthly;

    let money_per_day_static = income / days_in_month;

    let mut money_left = income;

    let mut last_money_per_day_adaptive: f32 = 0.0;

    for (day, expenditure_day) in expenditures_regular.iter().enumerate() {
        let day = day + 1;
        let day_f32 = day as f32;

        let days_left = days_in_month - day_f32 + 1.0;

        let money_per_day_adaptive = money_left / days_left;
        let money_per_day_adaptive = money_per_day_adaptive - expenditure_day;

        money_left -= expenditure_day;

        let expenditure_median = calc_median(&expenditures_regular[..day]);

        let mpds_minus_median = money_per_day_static - expenditure_median;

        let col_mpda = if (last_money_per_day_adaptive > money_per_day_adaptive)
            || (money_per_day_adaptive < 0.0)
            || (money_per_day_adaptive < money_per_day_static - expenditure_day)
        {
            col_mpda_bad
        } else {
            col_mpda_good
        };

        println!(
            "{} {} {} {} {} {}",
            col_day(&format!("|{day:2}|")),
            col_mpds(&format!("|{money_per_day_static:5.2}|")),
            col_exp(&format!("|{expenditure_day:6.2}|")),
            col_mpda(&format!("|{money_per_day_adaptive:7.2}|")), // TODO1 would be cool if we added more than just 2 options for color
            col_ema(&format!("|{expenditure_median:6.2}|")),
            col_mpds_apl_ema(&format!("|{mpds_minus_median:7.2}|")),
        );

        last_money_per_day_adaptive = money_per_day_adaptive;
    }

    Ok(())
}

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

fn col_money_today_precalc_monthly(str: &str) -> String {
    format!("{}", str.truecolor(160, 160, 20))
}

fn col_mpda_good(str: &str) -> String {
    format!("{}", str.truecolor(100, 200, 100))
}

fn col_mpda_bad(str: &str) -> String {
    format!("{}", str.truecolor(200, 100, 100))
}

fn col_expenditures(str: &str) -> String {
    format!("{}", str.truecolor(165, 20, 20))
}

fn col_ema(str: &str) -> String {
    format!("{}", str.truecolor(200, 80, 20))
}

fn col_money_today_default(str: &str) -> String {
    format!("{}", str.truecolor(120, 120, 220))
}

fn col_money_at_end_of_month_based_on_median(str: &str) -> String {
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

    println!(
        "{}",
        col_day("x day ------------------------------------> which day of the month it is")
    );
    println!(
        "{}{}{}",
        col_money_today_precalc_monthly("x money-"),
        col_day("today"),
        col_money_today_precalc_monthly(
            "-precalc-monthly ------------> how much money are we supposed to have each day, on average"
        )
    );
    println!(
        "{}{}{}{}{}",
        col_money_today_default("x money-"),
        col_day("this-morning"),
        col_money_today_default("-"),
        col_expenditures("pre-applied"),
        col_money_today_default(
            " ---------> how much money did we have this morning (before spending anything)"
        ),
    );
    println!(
        "{}{}{}",
        col_expenditures("x expenditures-"),
        col_day("today"),
        col_expenditures(" ---------------------> how much money did we spend today"),
    );
    println!(
        "{}{}{}",
        col_ema("x expenditures-median-average-"),
        col_day("so-far"),
        col_ema(" -----> until this day, what is our median average spending per day"),
    );
    println!(
        "{}{}{}{} {} {} {}",
        col_mpda_good("x money-"),
        col_money_today_default("today"),
        col_mpda_good("-"),
        col_expenditures("applied"),
        col_mpda_good("good"),
        col_mpda_bad("bad"),
        col_mpda_good("-----------> how much money are we left with today"),
    );
    println!(
        "{}{}{}{}{}",
        col_money_at_end_of_month_based_on_median("x money"),
        col_day("-at-end-of-month-"),
        col_money_at_end_of_month_based_on_median("based-on-"),
        col_ema("median"),
        col_money_at_end_of_month_based_on_median(
            " --> how much money would we have at the end of the month, based on the median average"
        ),
    );
    println!();

    let income = income - expenditures_monthly;

    let money_per_day_static = income / days_in_month;

    let mut money_left = income;

    // let mut last_money_per_day_adaptive: f32 = 0.0;

    for (day, expenditure_day) in expenditures_regular.iter().enumerate() {
        let day = day + 1;
        let day_f32 = day as f32;

        let days_left = days_in_month - day_f32 + 1.0; // including today

        let money_today_default = money_left / days_left;

        let money_per_day_adaptive = money_today_default - expenditure_day;

        money_left -= expenditure_day;

        let expenditure_median = calc_median(&expenditures_regular[..day]);

        let money_at_end_of_month_based_on_median =
            money_left - expenditure_median * (days_left - 1.0);

        let col_mpda = if false //(last_money_per_day_adaptive > money_per_day_adaptive)
            || (money_per_day_adaptive < 0.0)
            || (money_today_default < money_per_day_static)
        {
            col_mpda_bad
        } else {
            col_mpda_good
        };

        println!(
            "{} {} {} {} {} {} {}",
            col_day(&format!("|{day:2}|")),
            col_money_today_precalc_monthly(&format!("|{money_per_day_static:5.2}|")),
            col_money_today_default(&format!("|{money_today_default:7.2}|")),
            col_expenditures(&format!("|{expenditure_day:6.2}|")),
            col_ema(&format!("|{expenditure_median:6.2}|")),
            col_mpda(&format!("|{money_per_day_adaptive:7.2}|")),
            col_money_at_end_of_month_based_on_median(&format!(
                "|{money_at_end_of_month_based_on_median:8.2}|"
            )),
        );

        // last_money_per_day_adaptive = money_per_day_adaptive;
    }

    Ok(())
}

mod cmdline;
mod parse_bill_toml;

fn main() -> Result<(), String> {
    let (input_toml, year, month) = cmdline::parse()?;

    let (income, expenditures_monthly, expenditures_regular, days_in_month) =
        parse_bill_toml::main(input_toml, year, month)?;
    let days_in_month = days_in_month as f32;

    // TODO4? put in new module

    let income = income - expenditures_monthly;

    let money_per_day_default = income / days_in_month;

    let mut money_left = income;

    // TODO2 colored print
    println!("day | expenditures | money-per-day-adaptive | money-per-day-default");
    for (day, expenditure_day) in expenditures_regular.iter().enumerate() {
        let day = day + 1;
        let day_f32 = day as f32;

        let days_left = days_in_month - day_f32 + 1.0;

        let money_per_day_adaptive = money_left / days_left;
        let money_per_day_adaptive = money_per_day_adaptive - expenditure_day;

        money_left -= expenditure_day;

        println!(
            "{day:2} | {expenditure_day:6.2} | {money_per_day_adaptive:7.2} | {money_per_day_default:4.2}"
        );
    }

    Ok(())
}

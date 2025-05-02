mod cmdline;
mod parse_bill_data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO3 try and use something more appropriate (like a regular string)
    let (input_toml, year, month) = cmdline::parse()?;

    let (income, expenditures_monthly, expenditures_regular) =
        parse_bill_data::main(input_toml, year, month)?;

    dbg!(income);
    dbg!(expenditures_monthly);
    dbg!(expenditures_regular);

    Ok(())
}

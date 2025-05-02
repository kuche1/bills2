mod cmdline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (input_toml, year, month) = cmdline::parse()?;

    Ok(())
}

mod cmdline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cmdline::parse()?;

    Ok(())
}

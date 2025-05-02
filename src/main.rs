use clap::Parser; // cargo add clap --features derive
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Toml file containing bills data
    #[arg(short, long)]
    bills_toml: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO0 parse cmdline in a separate file

    let args = Args::parse();

    let input_toml = &args.bills_toml;

    let filename = Path::new(input_toml).file_name().unwrap();
    let filename = filename.to_str().unwrap(); // `to_str` fail if invalid utf-8 ; `to_string_lossy` replaces invalid utf-8 characters

    let year_and_month = filename
        .strip_suffix(".toml")
        .ok_or_else(|| format!("filename needs to end with `.toml`"))?;

    let year_and_month: Vec<&str> = year_and_month.split(".").collect();

    let (year, month) = match year_and_month.as_slice() {
        [year, month] => (year, month),
        _ => return Err("filename needs to follow the `<year>.<month>.toml` format".into()),
    };

    let year: u32 = match year.parse() {
        Ok(v) => v,
        Err(e) => return Err(format!("`{}` is not a valid year", year).into()),
    };

    let month: u32 = match month.parse() {
        Ok(v) => v,
        Err(e) => return Err(format!("`{}` is not a valid month", month).into()),
    };

    if (month < 1) || (month > 12) {
        return Err(format!("`{}` is not a valid month (1 to 12)", month).into());
    }

    Ok(())
}

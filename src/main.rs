use chrono; // cargo add chrono
use chrono::Datelike;
use std::fs;
use toml::Table; // cargo add toml

mod cmdline;

///////////// vvvvv stupid fucking shit (this should have been included in the library) // https://github.com/chronotope/chrono/issues/69
trait NaiveDateExt {
    fn days_in_month(&self) -> u32;
    fn is_leap_year(&self) -> bool;
}

impl NaiveDateExt for chrono::NaiveDate {
    fn days_in_month(&self) -> u32 {
        let month = self.month();
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => panic!("Invalid month: {}", month),
        }
    }

    fn is_leap_year(&self) -> bool {
        let year = self.year();
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }
}
///////////// ^^^^^

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (input_toml, year, month) = cmdline::parse()?;

    // TODO0 put this whole reading thing into it's own module
    // TODO0 maybe even put a new file for getting today

    let date = chrono::NaiveDate::from_ymd_opt(year.try_into().unwrap(), month, 1).unwrap();
    let days_in_month = date.days_in_month();

    let bills_data =
        fs::read_to_string(&input_toml).map_err(|_| format!("can't open file `{}`", input_toml))?;

    let bills_data = bills_data.parse::<Table>().unwrap(); // I think there is a way to "desearialise" the file using lib features

    dbg!(bills_data);

    Ok(())
}

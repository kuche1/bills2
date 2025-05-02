// TODO9 maybe put a new module for getting today

use chrono; // cargo add chrono
use chrono::Datelike;
use std::fs;
use toml::Table; // cargo add toml
use toml::Value;

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

fn toml_sum(value: Value) -> Result<f32, String> {
    match value {
        Value::String(_) | Value::Boolean(_) | Value::Datetime(_) => {
            return Err(format!("unsupported value: `{value}`"));
        }

        Value::Integer(v) => return Ok(v as f32),

        Value::Float(v) => return Ok(v as f32),

        Value::Array(v) => {
            let mut sum: f32 = 0.0;
            for value in v {
                sum += toml_sum(value)?;
            }
            return Ok(sum);
        }

        Value::Table(v) => {
            let mut sum: f32 = 0.0;
            for (_key, value) in v {
                sum += toml_sum(value)?;
            }
            return Ok(sum);
        }
    }
}

pub fn main(input_toml: String, year: u32, month: u32) -> Result<(f32, f32, Vec<f32>), String> {
    let date = chrono::NaiveDate::from_ymd_opt(year.try_into().unwrap(), month, 1).unwrap();
    let days_in_month = date.days_in_month();
    let days_in_month_usize: usize = days_in_month.try_into().unwrap();

    let bills_data =
        fs::read_to_string(&input_toml).map_err(|_| format!("can't open file `{}`", input_toml))?;

    let bills_data = bills_data.parse::<Table>().unwrap(); // I think there is a way to "desearialise" the file using lib features

    let mut income: f32 = 0.0;
    let mut expenditures_monthly: f32 = 0.0;
    let mut expenditures_regular = vec![0.0_f32; days_in_month_usize];

    for (bill_key, bill_value) in bills_data {
        match bill_key.as_str() {
            "INCOME" => {
                income += toml_sum(bill_value)?;
            }

            "EXPENDITURES-MONTHLY" => {
                expenditures_monthly += toml_sum(bill_value)?;
            }

            "EXPENDITURES-REGULAR" => match bill_value {
                Value::String(_)
                | Value::Boolean(_)
                | Value::Datetime(_)
                | Value::Integer(_)
                | Value::Float(_)
                | Value::Array(_) => {
                    return Err(format!("unsupported value: `{bill_value}`"));
                }

                Value::Table(value) => {
                    for (day, money) in value {
                        let day: usize = day
                            .parse()
                            .expect(&format!("`{}` is not a valid month day", day)); // TODO2 use actual error

                        let idx = day.checked_add_signed(-1).unwrap(); // TODO2 use actual error

                        let money = toml_sum(money)?;

                        match expenditures_regular.get_mut(idx) {
                            None => return Err(format!("invalid day of month: `{}`", day)),
                            Some(item) => *item += money,
                        }
                    }
                }
            },

            _ => {
                return Err(format!("unknown key: `{}`", bill_key));
            }
        }
    }

    Ok((income, expenditures_monthly, expenditures_regular))
}

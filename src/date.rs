use chrono::prelude::*;

//* Get the month and friday of the current w&eek. */
pub struct End_Date {
    pub Month: String,
    pub Day: u32,
    pub Year: u32,
}

impl End_Date {
    fn new(&self, month: String, day: u32, year: u32) -> End_Date {
        End_Date {
            Month: (month),
            Day: (day),
            Year: (year),
        }
    }
}
#[allow(unused_assignments)]
pub fn friday_of_week() -> End_Date {
    let date = Utc::now();

    let mut month = date.month();
    let week_day = date.weekday().to_string();
    let mut day = date.day();
    let year = date.year();

    match week_day.as_str() {
        "Mon" => day += 4,
        "Tue" => day += 3,
        "Wed" => day += 2,
        "Thu" => day += 1,
        "Fri" => (),
        "Sat" => day += 6,
        "Sun" => day += 5,
        _ => todo!(),
    }
    if day > 31 {
        day = day - 31;
        month += 1;
    }
    End_Date {
        Month: month.to_string(),
        Day: day,
        Year: year as u32,
    }
}

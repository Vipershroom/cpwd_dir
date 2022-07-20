use chrono::prelude::*;

//* Get the month and friday of the current week. */
pub struct EndDate {
    pub month: String,
    pub day: u32,
    pub year: i32,
}

impl EndDate {
    fn new(month: String, day: u32, year: i32) -> EndDate {
        EndDate {
            month: (month),
            day: (day),
            year: (year),
        }
    }

    pub fn format_week(&self) -> String {
        format!("WE_{}_{}_{}", self.month, self.day, self.year)
    }
}
#[allow(unused_assignments)]
pub fn friday_of_week() -> EndDate {
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
        _ => unreachable!(),
    }
    if day > 31 {
        day = day - 31;
        month += 1;
    }
    EndDate::new(month.to_string(), day, year)
}

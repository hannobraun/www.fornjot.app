use chrono::{Date, NaiveDate, Utc};

#[derive(clap::Parser)]
pub struct Args {
    pub last_release_date: NaiveDate,
}

impl Args {
    pub fn last_release_date(&self) -> Date<Utc> {
        Date::from_utc(self.last_release_date, Utc)
    }
}

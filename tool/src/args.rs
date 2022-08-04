use chrono::NaiveDate;

#[derive(clap::Parser)]
pub struct Args {
    pub last_release_date: NaiveDate,
}

use chrono::{Date, NaiveDate, Utc};

#[derive(clap::Parser)]
pub enum Args {
    PrintPullRequests(PrintPullRequests),
    CreateReleaseAnnouncement,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}

#[derive(clap::Parser)]
pub struct PrintPullRequests {
    pub last_release_date: NaiveDate,
}

impl PrintPullRequests {
    pub fn last_release_date(&self) -> Date<Utc> {
        Date::from_utc(self.last_release_date, Utc)
    }
}

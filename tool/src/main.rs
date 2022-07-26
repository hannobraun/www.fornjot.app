use std::collections::BTreeMap;

use chrono::{Date, NaiveDate, Utc};
use clap::Parser;
use octocrab::params::{pulls::Sort, Direction, State};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let last_release_date = Date::from_utc(args.last_release_date, Utc);

    let mut pull_requests = BTreeMap::new();
    let mut page = 1u32;

    'outer: loop {
        println!("Fetching page {}...", page);
        let pull_request_page = octocrab::instance()
            .pulls("hannobraun", "Fornjot")
            .list()
            .state(State::Closed)
            .sort(Sort::Created)
            .direction(Direction::Descending)
            .per_page(100) // this is the maximum number of results per page
            .page(page)
            .send()
            .await?;

        for pull_request in pull_request_page.items {
            if let Some(created_at) = pull_request.created_at {
                if created_at.date() < last_release_date {
                    // Results are sorted by date of creation, descending. Since
                    // we've reached on that was created before the last
                    // release, that means we're done.
                    break 'outer;
                }
            }

            if let Some(merged_at) = pull_request.merged_at {
                if merged_at.date() >= last_release_date {
                    pull_requests.insert(pull_request.number, pull_request);
                }
            }
        }

        if pull_request_page.next.is_some() {
            page += 1;
        } else {
            break;
        }
    }

    for (_, pull_request) in pull_requests {
        let url = pull_request.html_url.expect("Pull request is missing URL");
        println!("{}", url);
    }

    Ok(())
}

#[derive(Parser)]
pub struct Args {
    pub last_release_date: NaiveDate,
}

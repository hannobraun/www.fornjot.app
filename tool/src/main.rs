mod args;

use std::collections::BTreeMap;

use chrono::{Date, Utc};
use clap::Parser;
use octocrab::params::{pulls::Sort, Direction, State};

use self::args::Args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    print_pull_requests_since_last_release(args.last_release_date()).await?;

    Ok(())
}

async fn print_pull_requests_since_last_release(
    last_release_date: Date<Utc>,
) -> anyhow::Result<()> {
    let mut pull_requests = BTreeMap::new();
    let mut page = 1u32;

    loop {
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

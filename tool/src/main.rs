use octocrab::params::{pulls::Sort, Direction, State};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pull_requests = octocrab::instance()
        .pulls("hannobraun", "Fornjot")
        .list()
        .state(State::Closed)
        .sort(Sort::Created)
        .direction(Direction::Ascending)
        .per_page(100) // this is the maximum number of results per page
        .page(0u32)
        .send()
        .await?;

    for pull_request in pull_requests.items {
        println!("{}", pull_request.number);
    }

    Ok(())
}

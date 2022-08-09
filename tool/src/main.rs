mod args;
mod generate;
mod pull_requests;

use anyhow::Context;

use self::{
    args::Args, generate::create_release_announcement,
    pull_requests::print_pull_requests_since_last_release,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match Args::parse() {
        Args::PrintPullRequests(args) => {
            print_pull_requests_since_last_release(args.last_release_date())
                .await
                .context("Failed to print pull requests since last release")?;
        }
        Args::CreateReleaseAnnouncement(args) => {
            create_release_announcement(args.version)
                .await
                .context("Failed to create release announcement")?;
        }
    }

    Ok(())
}

mod args;
mod generate;
mod pull_requests;
mod run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run::run().await
}

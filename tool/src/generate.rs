use anyhow::Context;
use chrono::{Datelike, Utc};
use tokio::fs::File;

pub async fn create_release_announcement() -> anyhow::Result<()> {
    let now = Utc::now();

    let year = now.year();
    let week = now.iso_week().week();

    let path = format!("content/blog/weekly-release/{year}-w{week}/index.md");
    File::create(&path)
        .await
        .with_context(|| format!("Failed to create file `{path}`"))?;

    Ok(())
}

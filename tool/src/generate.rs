use std::path::PathBuf;

use anyhow::Context;
use chrono::{Datelike, Utc};
use tokio::fs::{self, File};

pub async fn create_release_announcement() -> anyhow::Result<()> {
    let now = Utc::now();

    let year = now.year();
    let week = now.iso_week().week();

    let dir =
        PathBuf::from(format!("content/blog/weekly-release/{year}-w{week}"));
    let file = dir.join("index.md");

    fs::create_dir_all(&dir).await.with_context(|| {
        format!("Failed to create directory `{}`", dir.display())
    })?;
    File::create(&file).await.with_context(|| {
        format!("Failed to create file `{}`", file.display())
    })?;

    Ok(())
}

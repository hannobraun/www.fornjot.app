use std::{fmt::Write, path::PathBuf};

use anyhow::Context;
use chrono::{Datelike, Utc};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

pub async fn create_release_announcement(
    version: String,
) -> anyhow::Result<()> {
    let now = Utc::now();

    let year = now.year();
    let week = now.iso_week().week();

    let mut file = create_file(year, week).await?;

    let mut buf = String::new();
    write!(
        buf,
        "\
+++
title = \"Weekly Release - 2022-W{week}\"

[extra]
version = \"{version}\"
+++

**TASK: Write introduction.**


### Sponsors

Fornjot is supported by [@webtrax-oz](https://github.com/webtrax-oz), [@lthiery](https://github.com/lthiery), [@Yatekii](https://github.com/Yatekii), [@martindederer](https://github.com/martindederer), [@hobofan](https://github.com/hobofan), [@ahdinosaur](https://github.com/ahdinosaur), [@thawkins](https://github.com/thawkins), [@bollian](https://github.com/bollian), [@rozgo](https://github.com/rozgo), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

If you want Fornjot to be stable and sustainable long-term, please consider [supporting me](https://github.com/sponsors/hannobraun) too.


### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

**TASK: Add end-user improvements.**


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

**TASK: Add ecosystem improvements.**


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

**TASK: Add internal improvements.**


### Issue of the Week

**TASK: Write.**


### Outlook

**TASK: Write.**
\
    "
    )?;

    file.write_all(buf.as_bytes()).await?;

    Ok(())
}

async fn create_file(year: i32, week: u32) -> anyhow::Result<File> {
    let dir =
        PathBuf::from(format!("content/blog/weekly-release/{year}-w{week}"));
    let file = dir.join("index.md");

    fs::create_dir_all(&dir).await.with_context(|| {
        format!("Failed to create directory `{}`", dir.display())
    })?;
    let file = File::create(&file).await.with_context(|| {
        format!("Failed to create file `{}`", file.display())
    })?;

    Ok(file)
}

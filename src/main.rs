#![cfg_attr(debug_assertions, allow(unused))]

use scraper::Html;
use tracing::instrument;
use tracing::metadata::LevelFilter;
use tracing::info as log;
use tracing_subscriber::fmt::format::FmtSpan;

use wrapped_error::DebugResultExt;
use serde::Deserialize;
use scraper::Selector;

use std::format as f;
use bstr::{BStr, ByteSlice, ByteVec};

mod wrapped_error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), miette::Report> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::INFO)
            .with_target(false)
            .with_level(false)
            .with_span_events(FmtSpan::FULL)
            .with_file(true)
            .with_line_number(true)
            .without_time()
            .finish(),
    )
    .wrap()?;

    let ryl_story_id: u32 = 22518;
    let archive_datetime: u64 = 2022_03_24_02_32_33;

    let fic_url = f!["https://www.royalroad.com/fiction/{ryl_story_id}"];
    let archived_fic_url = f!["https://web.archive.org/web/{archive_datetime}/{fic_url}"];

    let html = reqwest::get(fic_url).await.wrap()?.text().await.wrap()?;

    let document = Html::parse_document(&html);

    // let next = Selector::parse("link[rel=next]").wrap()?;

    let chapters = Selector::parse("table#chapters tbody tr").wrap()?;
    for chapter in document.select(&chapters) {
        for text in chapter.text() {
            let s = BStr::new(text.as_bytes().trim());
            if !s.is_empty() {
                log!("{s}");
            }
        }
    }

    Ok(())
}

#[derive(Deserialize)]
struct ChapterMeta {
    id: u32,
    volume_id: Option<u32>,
    title: String,
    slug: String,
    date: String,
    order: u32,
    visible: u8,
    url: String,
}

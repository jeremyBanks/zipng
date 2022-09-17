#![cfg_attr(debug_assertions, allow(unused))]
#![warn(unused_crate_dependencies)]

use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::borrow::Cow::Owned;
use std::collections::BTreeSet;
use std::env;
use std::fmt::Debug;
use std::format as f;
use std::hash::BuildHasher;
use std::hash::BuildHasherDefault;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use bstr::BStr;
use bstr::ByteSlice;
use bstr::ByteVec;
use color_eyre::install;
use eyre::Report as ErrorReport;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use pulldown_cmark;
use scraper::Html;
use scraper::Selector;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use static_assertions::assert_obj_safe;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use tokio::time::interval;
use tokio::time::Interval;
use tokio::time::MissedTickBehavior;
use tracing::debug;
use tracing::info;
use tracing::instrument;
use tracing::metadata::LevelFilter;
use tracing::trace;
use tracing::warn;
use tracing_error::ErrorLayer;
use tracing_error::SpanTrace;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;
use twox_hash::Xxh3Hash64;
use std::collections::HashSet;
use time::error::InvalidFormatDescription;
use time::format_description;
use time::OffsetDateTime;
use time::PrimitiveDateTime;


use crate::throttle::throttle;
use crate::throttle::Throttle;


use crate::load::load;
use crate::load::Load;
use crate::wrapped_error::DebugResultExt;

mod load;
mod throttle;
mod wrapped_error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ErrorReport> {
    if cfg!(debug_assertions) {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", f!("warn,{}=trace", env!("CARGO_CRATE_NAME")));
        }
    } else {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", f!("error,{}=warn", env!("CARGO_CRATE_NAME")));
        }
    }

    color_eyre::install().wrap()?;

    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .finish()
            .with(ErrorLayer::default()),
    )
    .wrap()?;

    let ryl_fic_ids = [49033, 22518, 25137, 21220];

    for ryl_fic_id in ryl_fic_ids {
        royalroad::spine(ryl_fic_id).await.wrap()?;
    }

    for ryl_fic_id in ryl_fic_ids {
        royalroad::fic(ryl_fic_id).await.wrap()?;
    }

    Ok(())
}

fn digest(bytes: &[u8]) -> String {
    let mut hasher = <BuildHasherDefault<Xxh3Hash64>>::default().build_hasher();
    bytes.hash(&mut hasher);
    let digest = hasher.finish();
    f!("x{digest:016X}")
}
mod web {
    use super::*;

    static THROTTLE: Lazy<Throttle> = Lazy::new(|| throttle("web", 256));

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Page {
        pub url: Arc<str>,
        pub url_final: Arc<str>,
        pub content_type: Option<String>,
        pub body: String,
    }

    #[instrument(skip_all)]
    pub async fn get(url: impl AsRef<str>) -> Result<Page, ErrorReport> {
        let url = url.as_ref().to_string();
        let digest = digest(url.as_bytes());
        load!("target/web/{digest}", async move || {
            THROTTLE.tick().await;

            info!("Fetching {url}");
            let request = reqwest::get(url.to_string());
            let response = request.await.wrap()?.error_for_status()?;
            let content_type =
                if let Some(header) = response.headers().get(http::header::CONTENT_TYPE) {
                    Some(header.to_str().wrap()?.to_string())
                } else {
                    None
                };
            let url_final = response.url().to_string();
            let body = response.bytes().await.wrap()?.to_vec();
            let body = String::from_utf8_lossy(&body).to_string();
            Page {
                body,
                content_type,
                url: url.into(),
                url_final: url_final.into(),
            }
        })
    }
}

mod ia {

    use super::*;

    static THROTTLE: Lazy<Throttle> = Lazy::new(|| throttle("archive.org", 16 * 1024));

    static FORMAT: Lazy<Result<Vec<format_description::FormatItem>, InvalidFormatDescription>> =
        Lazy::new(|| format_description::parse("[year][month][day][hour][minute][second]"));

    pub async fn get(url: &str, timestamp: i64) -> Result<web::Page, ErrorReport> {
        let datetime = OffsetDateTime::from_unix_timestamp(timestamp)?.format(FORMAT.as_ref()?)?;
        THROTTLE.tick().await;
        web::get(f!("https://web.archive.org/web/{datetime}id_/{url}")).await
    }
}

mod royalroad {
    use super::*;

    static SITE_ID: &str = "RYL";
    static THROTTLE: Lazy<Throttle> = Lazy::new(|| throttle(SITE_ID, 8 * 1024));

    #[instrument(level = "debug")]
    pub async fn spine_at(
        id: u64,
        slug: Option<String>,
        archive_timestamp: Option<i64>,
    ) -> Result<Spine, ErrorReport> {
        let t = archive_timestamp
            .map(|t| f!["-{t}"])
            .unwrap_or_else(String::new);
        load!("data/spines/{SITE_ID}{id:07}{t}", async move || {
            THROTTLE.tick().await;

            let slug = slug.unwrap_or_else(String::new);
            let url = f!["https://www.royalroad.com/fiction/{id}/{slug}"];

            let page = if let Some(archive_timestamp) = archive_timestamp {
                ia::get(&url, archive_timestamp).await?
            } else {
                web::get(url).await?
            };

            let slug = page
                .url_final
                .split("://")
                .last()
                .unwrap()
                .split("/fiction/")
                .skip(1)
                .next()
                .unwrap()
                .split("/")
                .skip(1)
                .next()
                .unwrap()
                .to_string()
                .into();

            let html = page.body;

            let document = Html::parse_document(html.as_ref());

            let mut chapters = BTreeSet::new();

            let title = document
                .select(&Selector::parse("title").unwrap())
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .split("|")
                .next()
                .unwrap()
                .trim()
                .to_owned()
                .into();

            static FORMAT_1: Lazy<
                Result<Vec<format_description::FormatItem>, InvalidFormatDescription>,
            > = Lazy::new(|| {
                format_description::parse(
                    "[weekday], [day] [month repr:long] [year] [hour]:[minute]",
                )
            });
            static FORMAT_2: Lazy<
                Result<Vec<format_description::FormatItem>, InvalidFormatDescription>,
            > = Lazy::new(|| {
                format_description::parse(
                    "[weekday], [month repr:long] [day padding:none], [year] [hour repr:12 \
                     padding:none]:[minute] [period]",
                )
            });

            for chapter in document.select(&Selector::parse("table#chapters tbody tr").wrap()?) {
                let html = chapter.html();

                let chapter_link = chapter
                    .select(&Selector::parse("a").wrap()?)
                    .next()
                    .unwrap();
                let chapter_time = chapter
                    .select(&Selector::parse("time").wrap()?)
                    .next()
                    .unwrap();

                let timestamp: i64 = chapter_time
                    .value()
                    .attr("unixtime")
                    .map(|s| s.parse().unwrap())
                    .unwrap_or_else(|| {
                        let s = chapter_time
                            .value()
                            .attr("title")
                            .expect("no title found in chapter link, either!");
                        PrimitiveDateTime::parse(s, FORMAT_1.as_ref().unwrap())
                            .or(PrimitiveDateTime::parse(s, FORMAT_2.as_ref().unwrap()))
                            .expect(&f!["{s:?}"])
                            .assume_utc()
                            .unix_timestamp()
                    });

                let title = chapter_link
                    .text()
                    .next()
                    .unwrap()
                    .trim()
                    .to_string()
                    .into();
                let href = chapter_link.value().attr("href").unwrap();

                let mut id_slug = href
                    .split("://")
                    .last()
                    .unwrap()
                    .split("/chapter/")
                    .last()
                    .unwrap()
                    .split("/");
                let id = id_slug.next().unwrap().parse().wrap()?;
                let slug = id_slug.next().unwrap().to_string().into();

                chapters.insert(SpineChapter {
                    id,
                    timestamp,
                    title,
                    slug,
                });
            }

            Spine {
                id,
                title,
                slug,
                chapters,
            }
        })
    }

    pub async fn spine(id: u64) -> Result<Spine, ErrorReport> {
        spine_at(id, None, None).await
    }

    #[instrument]
    pub async fn fic(id: u64) -> Result<Fic, ErrorReport> {
        let spine = spine(id).await?;

        let archive_timestamp = spine.chapters.iter().map(|c| c.timestamp).min();

        let archived = spine_at(id, spine.slug.to_string().into(), archive_timestamp).await?;

        let mut chapters = BTreeSet::new();

        for chapter in &spine.chapters {
            let chapter = fic_chapter(&spine, &chapter).await?;
            chapters.insert(chapter);
        }

        for chapter in &archived.chapters {
            let chapter = fic_chapter_at(&spine, &chapter, archive_timestamp).await?;
            chapters.insert(chapter);
        }

        Ok(Fic {
            id,
            title: spine.title,
            slug: spine.slug,
            chapters,
        })
    }

    pub async fn fic_chapter(
        spine: &Spine,
        chapter: &SpineChapter,
    ) -> Result<FicChapter, ErrorReport> {
        fic_chapter_at(spine, chapter, None).await
    }

    #[instrument(skip_all)]
    pub async fn fic_chapter_at(
        spine: &Spine,
        chapter: &SpineChapter,
        archive_timestamp: Option<i64>,
    ) -> Result<FicChapter, ErrorReport> {
        let spine = spine.clone();
        let chapter = chapter.clone();
        let fic_id = spine.id;
        let fic_slug = spine.slug.clone();
        let chapter_id = chapter.id;
        let chapter_slug = chapter.slug.clone();

        let t = archive_timestamp
            .map(|t| f!["-{t}"])
            .unwrap_or_else(String::new);
        load!(
            "target/chapters/{SITE_ID}{chapter_id}{t}",
            async move || {
                THROTTLE.tick().await;

                let url = f!["https://www.royalroad.com/fiction/{fic_id}/{fic_slug}/chapter/{chapter_id}/{chapter_slug}"];

                let html = if let Some(timestamp) = archive_timestamp {
                    ia::get(&url, timestamp).await?
                } else {
                    web::get(url).await?
                }
                .body;

                let document = Html::parse_document(html.as_ref());

                let html_original = document
                    .select(&Selector::parse("div.chapter-inner").wrap()?)
                    .next()
                    .expect("missing expected div.chapter-inner in document")
                    .html();

                let html = ammonia::Builder::new()
                    .rm_tags(HashSet::<&str>::from_iter(["img", "span"]))
                    .url_schemes(HashSet::<&str>::from_iter([
                        "http", "https", "mailto", "magnet",
                    ]))
                    .url_relative(ammonia::UrlRelative::Deny)
                    .clean(&html_original)
                    .to_string()
                    .into();

                FicChapter {
                    id: chapter.id,
                    title: chapter.title.clone(),
                    timestamp: chapter.timestamp,
                    slug: chapter.slug.clone(),
                    html,
                }
            }
        )
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Fic {
        id: u64,
        title: Arc<str>,
        slug: Arc<str>,
        chapters: BTreeSet<FicChapter>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FicChapter {
        id: u64,
        timestamp: i64,
        title: Arc<str>,
        slug: Arc<str>,
        html: Arc<str>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Spine {
        id: u64,
        title: Arc<str>,
        slug: Arc<str>,
        chapters: BTreeSet<SpineChapter>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SpineChapter {
        id: u64,
        timestamp: i64,
        title: Arc<str>,
        slug: Arc<str>,
    }
}

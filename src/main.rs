#![cfg_attr(debug_assertions, allow(unused))]
#![warn(unused_crate_dependencies)]

use std::borrow::Cow;
use std::borrow::Cow::Borrowed;
use std::borrow::Cow::Owned;
use std::collections::BTreeSet;
use std::collections::HashSet;
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
use scraper::Html;
use scraper::Selector;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use static_assertions::assert_obj_safe;
use time::error::InvalidFormatDescription;
use time::format_description;
use time::OffsetDateTime;
use time::PrimitiveDateTime;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::join;
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::time::interval;
use tokio::time::Interval;
use tokio::time::MissedTickBehavior;
use tokio::try_join;
use tracing::debug;
use tracing::error;
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

use crate::load::load;
use crate::load::Load;
use crate::throttle::throttle;
use crate::throttle::Throttle;
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
            .with_span_events(FmtSpan::CLOSE)
            .finish()
            .with(ErrorLayer::default()),
    )
    .wrap()?;

    let mol = royalroad::fic(21220);
    let ant = royalroad::fic(22518);
    let wtc = royalroad::fic(25137);
    let mox = royalroad::fic(49033);

    let (mol, ant, wtc, mox) = join!(mol, ant, wtc, mox);
    let (mol, ant, wtc, mox) = (mol?, ant?, wtc?, mox?);

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

    static THROTTLE: Lazy<Throttle> = Lazy::new(|| throttle("web", 16 * 1024));

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Page {
        pub url: String,
        pub url_final: String,
        pub content_type: Option<String>,
        pub body: String,
    }

    #[instrument(level = "trace", skip_all)]
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

mod royalroad {
    use std::io::Write;

    use super::*;

    static THROTTLE: Lazy<Throttle> = Lazy::new(|| throttle("RoyalRoad", 128));

    #[instrument(level = "trace")]
    pub async fn spine(id: u64) -> Result<Spine, ErrorReport> {
        let id10 = fic_id10(id);
        let spine = load!("target/spines/{id10}", async move || {
            THROTTLE.tick().await;

            let url = f!["https://www.royalroad.com/fiction/{id}"];

            let page = web::get(url).await?;

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

                chapters.insert(SpineChapter {
                    id,
                    id10: chapter_id10(id),
                    timestamp,
                    title,
                });
            }

            Spine {
                id,
                id10: fic_id10(id),
                title,
                chapters,
            }
        })?;

        Ok(spine)
    }

    #[instrument(level = "trace")]
    pub async fn fic(id: u64) -> Result<Fic, ErrorReport> {
        let id10 = fic_id10(id);

        let fic = load!("target/fics/{id10}", async move || {
            let spine = spine(id).await?;

            let mut chapters = BTreeSet::new();

            for chapter in &spine.chapters {
                let chapter = fic_chapter(&spine, &chapter).await?;
                chapters.insert(chapter);
            }

            let fic = Fic {
                id,
                id10: fic_id10(id),
                title: spine.title,
                chapters,
            };

            info!("Loaded fic #{id} {title:?}", title = &fic.title);
            info!(chapter_count = fic.chapters.len());

            // let span = tracing::info_span!("JSON+Brotli...").entered();
            // let mut fic_brotli_json = Vec::new();
            // serde_json::to_writer_pretty(
            //     brotli::CompressorWriter::new(&mut fic_brotli_json, 0, 11, 24),
            //     &fic,
            // )?;
            // drop(span);
            // info!(json_brotli = fic_brotli_json.len());

            // let span = tracing::info_span!("JSON+zstd...").entered();
            // let mut fic_zstd_json = Vec::new();
            // serde_json::to_writer_pretty(
            //     zstd::Encoder::new(&mut fic_zstd_json, 22)?.auto_finish(),
            //     &fic,
            // )?;
            // drop(span);
            // info!(json_zstd = fic_zstd_json.len());

            fic
        })?;

        let ff = fic.clone();
        let _rich: Result<RichSpine, _> = load!("data/spines/{id10}", async move || {
            let mut chapters = BTreeSet::new();

            for chapter in ff.chapters {
                let starts_with = chapter
                    .html
                    .to_string()
                    .split_ascii_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .chars()
                    .take(96)
                    .collect::<String>()
                    .rsplit_once(" ")
                    .unwrap()
                    .0
                    .to_string();

                let chapter = RichSpineChapter {
                    id10: chapter.id10.clone(),
                    timestamp: chapter.timestamp,
                    title: chapter.title.clone(),
                    length: chapter.html.len() as _,
                    starts_with,
                };
                chapters.insert(chapter);
            }

            RichSpine {
                id10,
                author: "TODO".to_string(),
                title: ff.title,
                length: chapters.iter().map(|c| c.length).sum(),
                chapters,
            }
        });

        Ok(fic)
    }

    pub async fn fic_chapter(
        spine: &Spine,
        chapter: &SpineChapter,
    ) -> Result<FicChapter, ErrorReport> {
        let spine = spine.clone();
        let chapter = chapter.clone();
        let fic_id = spine.id;
        let chapter_id = chapter.id;

        let fic10 = fic_id10(fic_id);
        let id10 = chapter_id10(chapter_id);

        load!("target/chapters/{fic10}{id10}", async move || {
            THROTTLE.tick().await;

            let url = f!["https://www.royalroad.com/chapter/{chapter_id}"];

            let html = web::get(url).await?.body;

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
                id10: chapter_id10(chapter.id),
                title: chapter.title.clone(),
                timestamp: chapter.timestamp,
                html,
            }
        })
    }

    fn fic_id10(id: u64) -> String {
        format!("RYL{id:07}")
    }

    fn chapter_id10(id: u64) -> String {
        format!("C{id:09}")
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Fic {
        id: u64,
        id10: String,
        title: String,
        chapters: BTreeSet<FicChapter>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FicChapter {
        id: u64,
        id10: String,
        timestamp: i64,
        title: String,
        html: String,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Spine {
        id: u64,
        id10: String,
        title: String,
        chapters: BTreeSet<SpineChapter>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SpineChapter {
        id: u64,
        id10: String,
        timestamp: i64,
        title: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RichSpine {
        id10: String,
        title: String,
        author: String,
        length: u64,
        chapters: BTreeSet<RichSpineChapter>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RichSpineChapter {
        id10: String,
        timestamp: i64,
        title: String,
        length: u64,
        starts_with: String,
    }
}

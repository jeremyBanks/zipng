#![cfg_attr(debug_assertions, allow(unused))]

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

use bstr::BStr;
use bstr::ByteSlice;
use bstr::ByteVec;
use color_eyre::install;
use miette::Report as ErrorReport;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use scraper::Html;
use scraper::Selector;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use static_assertions::assert_obj_safe;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time;
use tracing::debug;
use tracing::info;
use tracing::instrument;
use tracing::metadata::LevelFilter;
use tracing::trace;
use tracing::warn;
use tracing_error::ErrorLayer;
use tracing_error::SpanTrace;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::EnvFilter;
use twox_hash::Xxh3Hash64;

use crate::load::load;
use crate::load::Load;
use crate::wrapped_error::DebugResultExt;

mod load;
mod wrapped_error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ErrorReport> {
    if cfg!(debug_assertions) {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", f!("warn,{}=trace", env!("CARGO_CRATE_NAME")));
        }
        if env::var("RUST_BACKTRACE").is_err() {
            env::set_var("RUST_BACKTRACE", "0");
        }
        if env::var("RUST_SPANTRACE").is_err() {
            env::set_var("RUST_SPANTRACE", "1");
        }
    } else {
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", f!("error,{}=warn", env!("CARGO_CRATE_NAME")));
        }
    }

    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_target(false)
            .with_level(false)
            .with_span_events(FmtSpan::FULL)
            .with_file(true)
            .with_line_number(true)
            .without_time()
            .finish()
            .with(ErrorLayer::default()),
    )
    .wrap()?;

    color_eyre::install().wrap()?;

    let s = load!("target/test", async || {
        time::sleep(time::Duration::from_secs(1)).await;
        "hello world".to_string()
    })?;

    // let ryl_story_id: u64 = 22518;
    // let archive_datetime: u64 = 2022_03_24_02_32_33;

    // let fic_url = f!["https://www.royalroad.com/fiction/{ryl_story_id}"];
    // let archived_fic_url = f!["https://web.archive.org/web/{archive_datetime}/{fic_url}"];

    // let html = reqwest::get(archived_fic_url)
    //     .await
    //     .wrap()?
    //     .text()
    //     .await
    //     .wrap()?;

    // let document = Html::parse_document(&html);

    // // let next = Selector::parse("link[rel=next]").wrap()?;

    // let chapters = Selector::parse("table#chapters tbody tr").wrap()?;
    // for chapter in document.select(&chapters) {
    //     let html = chapter.html();
    //     trace!("{html}");
    //     for text in chapter.text() {
    //         let s = BStr::new(text.as_bytes().trim());
    //         if !s.is_empty() {
    //             trace!("{s}");
    //         }
    //     }
    // }

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

    static LOCAL_PREFIX: &str = "target/web";

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Page {
        url: Arc<str>,
        url_final: Arc<str>,
        content_type: Option<String>,
        body: Vec<u8>,
    }

    pub async fn get(url: impl AsRef<str>) -> Result<Page, ErrorReport> {
        let url = url.as_ref().to_string();
        let digest = digest(url.as_bytes());
        load!("{LOCAL_PREFIX}/{digest}", async move || {
            let request = reqwest::get(url.to_string());
            let response = request.await.wrap()?;
            let content_type =
                if let Some(header) = response.headers().get(http::header::CONTENT_TYPE) {
                    Some(header.to_str().wrap()?.to_string())
                } else {
                    None
                };
            let url_final = response.url().to_string();
            let body = response.bytes().await.wrap()?.to_vec();
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

    static LOCAL_PREFIX: &str = "target/ia";
    static URL_PREFIX: &str = "https://web.archive.org";
    static OLDEST_SUFFIX: &str = "0id_";
    static LATEST_SUFFIX: &str = "3id_";

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Page {
        url: Arc<str>,
        datetime: u64,
        body: Vec<u8>,
    }

    pub async fn get(url: &str) -> Result<Page, ErrorReport> {
        todo!()
    }

    pub async fn get_before(url: &str, datetime: u64) -> Result<Page, ErrorReport> {
        todo!()
    }
}

mod fic {
    use super::*;

    /// `Fic` represents the full contents and metadata of a fic.
    assert_obj_safe!(Fic<Chapter = dyn FicChapter, Chapters = Vec<Box<dyn FicChapter>>>);
    pub trait Fic {
        /// A unique identifier for the site this fic was originally published
        /// on.
        fn site_id(&self) -> &'static str;

        /// The ID of the fic. This is unique per-site.
        fn id(&self) -> u64;

        /// The title of the fic.
        fn title(&self) -> Cow<Arc<str>> {
            Owned(format!("Untitled {}-{}", self.site_id(), self.id()).into())
        }

        type Chapter: FicChapter;
        type Chapters: IntoIterator<Item = Self::Chapter>;
        fn chapters(&self) -> &Self::Chapters;

        /// Timestamp of publication/creation as seconds from unix epoch, if
        /// known.
        fn published(&self) -> Option<u64> {
            None
        }
    }

    pub trait FicChapter {
        /// A unique identifier for the site this fic was originally published
        /// on.
        fn site_id(&self) -> &'static str;

        /// The ID of the chapter. This is unique per-site.
        fn id(&self) -> u64;

        /// The title of the chapter.
        fn title(&self) -> Cow<Arc<str>> {
            Owned(format!("Untitled {}", self.id()).into())
        }

        /// The source HTML of the chapter, as it was originally published.
        ///
        /// This may not be suitable for direct inclusion in other documents.
        fn html_original(&self) -> Cow<Arc<str>>;

        /// Timestamp of publication/creation as seconds from unix epoch, if
        /// known.
        fn published(&self) -> Option<u64> {
            None
        }
    }

    /// `Spine` represents the shallow cover/metadata/index/TOC of a fic
    /// (i.e. it's a [`Fic`] without the chapter contents).
    assert_obj_safe!(Spine<Chapter = dyn SpineChapter, Chapters = Vec<Box<dyn SpineChapter>>>);
    pub trait Spine {
        /// A unique identifier for the site this fic was originally published
        /// on.
        fn site_id(&self) -> &'static str;

        /// The ID of the fic. This is unique per-site.
        fn id(&self) -> u64;

        /// The title of the fic.
        fn title(&self) -> Cow<Arc<str>> {
            Owned(format!("Untitled {}-{}", self.site_id(), self.id()).into())
        }

        type Chapter: SpineChapter;
        type Chapters: IntoIterator<Item = Self::Chapter>;
        fn chapters(&self) -> &Self::Chapters;

        /// Timestamp of publication/creation as seconds from unix epoch, if
        /// known.
        fn published(&self) -> Option<u64> {
            None
        }
    }

    pub trait SpineChapter {
        /// A unique identifier for the site this fic was originally published
        /// on.
        fn site_id(&self) -> &'static str {
            "RYL"
        }

        /// The ID of the chapter. This is unique per-site.
        fn id(&self) -> u64;

        /// The title of the chapter.
        fn title(&self) -> Cow<Arc<str>> {
            Owned(format!("Untitled {}", self.id()).into())
        }

        /// Timestamp of publication/creation as seconds from unix epoch, if
        /// known.
        fn published(&self) -> Option<u64> {
            None
        }
    }
}

mod royalroad {
    use super::*;

    static SITE_ID: &str = "RYL";
    static LOCAL_PREFIX: &str = "./data/royalroad";
    static URL_PREFIX: &str = "https://www.royalroad.com";

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Fic {
        id: u64,
        title: Arc<str>,
        chapters: BTreeSet<FicChapter>,
    }

    impl fic::Fic for Fic {
        fn site_id(&self) -> &'static str {
            SITE_ID
        }

        fn id(&self) -> u64 {
            self.id
        }

        fn title(&self) -> Cow<Arc<str>> {
            Borrowed(&self.title)
        }

        type Chapter = FicChapter;
        type Chapters = BTreeSet<FicChapter>;
        fn chapters(&self) -> &Self::Chapters {
            &self.chapters
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FicChapter {
        id: u64,
        timestamp: u64,
        title: Arc<str>,
        html_original: Arc<str>,
    }

    impl fic::FicChapter for FicChapter {
        fn site_id(&self) -> &'static str {
            SITE_ID
        }

        fn id(&self) -> u64 {
            self.id
        }

        fn title(&self) -> Cow<Arc<str>> {
            Borrowed(&self.title)
        }

        fn html_original(&self) -> Cow<Arc<str>> {
            Borrowed(&self.html_original)
        }

        fn published(&self) -> Option<u64> {
            Some(self.timestamp)
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Spine {
        id: u64,
        title: Arc<str>,
        chapters: BTreeSet<SpineChapter>,
    }

    impl fic::Spine for Spine {
        fn site_id(&self) -> &'static str {
            "RYL"
        }

        fn id(&self) -> u64 {
            self.id
        }

        fn title(&self) -> Cow<Arc<str>> {
            Borrowed(&self.title)
        }

        type Chapter = SpineChapter;
        type Chapters = BTreeSet<SpineChapter>;
        fn chapters(&self) -> &Self::Chapters {
            &self.chapters
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SpineChapter {
        id: u64,
        timestamp: u64,
        title: Arc<str>,
    }

    impl fic::SpineChapter for SpineChapter {
        fn site_id(&self) -> &'static str {
            "RYL"
        }

        fn id(&self) -> u64 {
            self.id
        }

        fn title(&self) -> Cow<Arc<str>> {
            Borrowed(&self.title)
        }

        fn published(&self) -> Option<u64> {
            Some(self.timestamp)
        }
    }

    pub async fn fic(id: u64) -> Result<Fic, ErrorReport> {
        let cover = load!("{LOCAL_PREFIX}/fics/{id}.json", async move || -> Spine {
            let url = f!["{URL_PREFIX}/fiction/{id}"];
            Spine {
                id: 0,
                title: "TODO".into(),
                chapters: BTreeSet::new(),
            }
        })?;

        todo!()
    }
}

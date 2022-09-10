#![cfg_attr(debug_assertions, allow(unused))]

use std::collections::BTreeSet;
use std::fmt::Debug;
use std::format as f;
use std::fs;
use std::future::Future;
use std::hash::BuildHasher;
use std::hash::BuildHasherDefault;
use std::hash::Hash;
use std::hash::Hasher;
use std::path::Path;
use std::pin::Pin;

use bstr::BStr;
use bstr::ByteSlice;
use bstr::ByteVec;
use miette::Report as ErrorReport;
use scraper::Html;
use scraper::Selector;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use tracing::debug;
use tracing::info;
use tracing::instrument;
use tracing::metadata::LevelFilter;
use tracing::trace;
use tracing::warn;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;
use twox_hash::Xxh3Hash64;
use wrapped_error::DebugResultExt;

mod wrapped_error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ErrorReport> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_env_filter(
                std::env::var("RUST_LOG")
                    .unwrap_or_else(|_| f!("warn,{}=trace", env!("CARGO_CRATE_NAME"))),
            )
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

    let html = reqwest::get(archived_fic_url)
        .await
        .wrap()?
        .text()
        .await
        .wrap()?;

    let document = Html::parse_document(&html);

    // let next = Selector::parse("link[rel=next]").wrap()?;

    let chapters = Selector::parse("table#chapters tbody tr").wrap()?;
    for chapter in document.select(&chapters) {
        let html = chapter.html();
        trace!("{html}");
        for text in chapter.text() {
            let s = BStr::new(text.as_bytes().trim());
            if !s.is_empty() {
                trace!("{s}");
            }
        }
    }

    Ok(())
}

trait Record:
    DeserializeOwned + Serialize + Clone + Debug + Send + Sync + Eq + Ord + Hash + 'static
{
}

impl<T> Record for T where
    T: DeserializeOwned + Serialize + Clone + Debug + Send + Sync + Eq + Ord + Hash + 'static
{
}

async fn load<Output: Record>(
    path: Option<&Path>,
    fetch: impl FnOnce() -> Box<dyn Future<Output = Result<Output, ErrorReport>> + Unpin>,
) -> Result<Output, ErrorReport> {
    trace!("Loading {path:?}");

    if let Some(path) = path {
        // TODO: replace this with a tokio/async operation
        match fs::read(path) {
            Ok(bytes) => match serde_json::from_slice(&bytes) {
                Ok(output) => {
                    debug!("Loaded {path:?}");
                    return Ok(output);
                },
                Err(err) => {
                    warn!("Found existing file at {path:?} but parsing failed: {err}");
                },
            },
            Err(err) => {
                info!("Failed to read existing file at {path:?}: {err}");
            },
        }
    }

    let fetched = Box::pin(fetch()).await?;

    todo!()
}

macro_rules! load {
    ($path:expr, async $($move:ident)? || $( -> $output:path)? { $($body:tt)* }) => {
        {
            let path = format!($path) + ".json";
            let path = Path::new(&path);
            let output$(: $output)? = load(
                Some(path),
                || Box::new(async $($move)? { Ok({ $($body)* }) }),
            ).await?;
            Ok::<_, ErrorReport>(output)
        }
    };

    (async $($move:ident)? || $( -> $output:path)? { $($body:tt)* }) => {
        {
            let output$(: $output)? = load(
                None,
                || Box::new(async $($move)? { Ok({ $($body)* }) }),
            ).await?;
            Ok::<_, ErrorReport>(output)
        }
    };

    { $($body:tt)* } => {
        {
            let output = load(
                None,
                || Box::new(async { Ok({ $($body)* }) }),
            ).await?;
            Ok::<_, ErrorReport>(output)
        }
    };
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
        url: String,
        url_final: String,
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
                url,
                url_final,
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
        url: String,
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

mod royalroad {

    use super::*;

    static LOCAL_PREFIX: &str = "data/royalroad";
    static URL_PREFIX: &str = "https://www.royalroad.com";

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Fic {
        pub fic: Cover,
        pub chapters: Vec<Chapter>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Cover {
        pub id: u32,
        pub title: String,
        pub chapter_ids: BTreeSet<u32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Chapter {
        pub id: u32,
        pub title: String,
        pub html: String,
    }

    pub async fn fic(id: u32) -> Result<Fic, ErrorReport> {
        let cover = load!("{LOCAL_PREFIX}/fics/{id}.json", async move || -> Cover {
            let url = f!["{URL_PREFIX}/fiction/{id}"];
            Cover {
                id: 0,
                title: "TODO".to_string(),
                chapter_ids: BTreeSet::new(),
            }
        })?;

        todo!()
    }
}

use super::*;

pub trait Load:
    DeserializeOwned + Serialize + Clone + Debug + Send + Sync + PartialEq + Hash + 'static
{
}

impl<T> Load for T where
    T: DeserializeOwned + Serialize + Clone + Debug + Send + Sync + PartialEq + Hash + 'static
{
}

pub async fn load<Output: Load>(
    path: Option<&Path>,
    fetch: impl FnOnce() -> tokio::task::JoinHandle<Result<Output, ErrorReport>>,
) -> Result<Output, ErrorReport> {
    trace!("Loading {path:?}");

    if let Some(path) = path {
        match fs::read(path).await {
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

    let fetched = fetch().await.wrap()??; // uh oh

    todo!()
}

#[macro_export(crate)]
macro_rules! load {
($path:expr, async $($move:ident)? || $( -> $output:path)? { $($body:tt)* }) => {
    {
        let path = format!($path) + ".json";
        let path = Path::new(&path);
        let output$(: $output)? = load(
            Some(path),
            $($move)? || tokio::spawn(async $($move)? { Ok({ $($body)* }) }),
        ).await?;
        Ok::<_, ErrorReport>(output)
    }
};

(async $($move:ident)? || $( -> $output:path)? { $($body:tt)* }) => {
    {
        let output$(: $output)? = load(
            None,
            $($move)? || tokio::spawn(async $($move)? { Ok({ $($body)* }) }),
        ).await?;
        Ok::<_, ErrorReport>(output)
    }
};
}

//! This is only available in local builds where dev-dependencies are enabled.

pub use crate::init;
#[macro_export]
macro_rules! init {
    () => {
        if ::std::env::var("RUST_LOG").is_err() {
            ::std::env::set_var(
                "RUST_LOG",
                format!("warn,zipng=trace,{}=trace", env!("CARGO_CRATE_NAME")),
            );
        }

        ::tracing::subscriber::set_global_default(
            ::tracing_subscriber::layer::SubscriberExt::with(
                ::tracing_subscriber::fmt()
                    .with_env_filter(::tracing_subscriber::EnvFilter::from_default_env())
                    .pretty()
                    .with_span_events(::tracing_subscriber::fmt::format::FmtSpan::CLOSE)
                    .finish(),
                ::tracing_error::ErrorLayer::default(),
            ),
        )?;

        ::color_eyre::install()?;
    };
}

pub use crate::save;
#[macro_export]
macro_rules! save {
    ($value:tt $(.$ext:ident)+) => {{
        $(
            let mut f = ::std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(format!("test_data/{}.{}", env!("CARGO_CRATE_NAME"), stringify!($ext)))?;
            #[allow(unused_braces, unused_parentheses)]
            std::io::Write::write_all(&mut f, &*{$value})?;
        )+
        Ok(())
    }};
}

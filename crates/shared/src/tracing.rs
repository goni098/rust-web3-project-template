use strum::IntoEnumIterator;

/// Initializes the tracing subscriber with stdout and stderr layers
///
/// Logs at ERROR level go to stderr, all other levels to stdout

#[derive(strum::Display, strum::EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum Crate {
    HttpServer,
    EvmLib,
    EvmScanner,
    EvmStream,
    SolLib,
    SolanaScanner,
    SolanaStream,
}

pub fn subscribe() {
    use tracing::Level;
    use tracing_subscriber::{
        EnvFilter, Layer, filter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
    };

    let out_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(
            Crate::iter().fold(EnvFilter::from_default_env(), |filter, c| {
                filter.add_directive(c.to_string().parse().expect("Invalid filter directive"))
            }),
        )
        .with_filter(filter::filter_fn(|metadata| {
            *metadata.level() != Level::ERROR
        }));

    let err_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_filter(filter::LevelFilter::ERROR);

    tracing_subscriber::registry()
        .with(out_layer)
        .with(err_layer)
        .init();
}

/// Initializes the tracing subscriber with stdout and stderr layers
/// 
/// Logs at ERROR level go to stderr, all other levels to stdout
pub fn subscribe() {
    use tracing::Level;
    use tracing_subscriber::{
        EnvFilter, Layer, filter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
    };

    const BINS: [&str; 6] = [
        "err_trace",
        "http_server",
        "evm_scanner",
        "evm_stream",
        "solana_scanner",
        "solana_stream",
    ];

    let out_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(
            BINS.iter()
                .fold(EnvFilter::from_default_env(), |filter, bin| {
                    filter.add_directive(bin.parse().expect("Invalid filter directive"))
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
        .with(tracing_error::ErrorLayer::default())
        .init();
}

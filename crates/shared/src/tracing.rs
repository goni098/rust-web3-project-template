pub fn subscribe() {
    use tracing::Level;
    use tracing_subscriber::{
        EnvFilter, Layer, filter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
    };

    let bins = ["err_trace", "http_server"];

    let out_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(
            bins.iter()
                .fold(EnvFilter::from_default_env(), |filter, bin| {
                    filter.add_directive(bin.parse().unwrap())
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

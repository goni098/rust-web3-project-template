use std::sync::Arc;

pub fn tls_connector() -> tokio_rustls::TlsConnector {
    let mut root_store = tokio_rustls::rustls::RootCertStore::empty();

    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = tokio_rustls::rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    tokio_rustls::TlsConnector::from(Arc::new(config))
}

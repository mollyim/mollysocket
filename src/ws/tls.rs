use native_tls;

pub fn build_tls_connector() -> Result<native_tls::TlsConnector, native_tls::Error> {
    let mut builder = native_tls::TlsConnector::builder();
    builder.danger_accept_invalid_certs(true);
    builder.build()
}

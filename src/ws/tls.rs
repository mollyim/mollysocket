use native_tls::{Certificate, TlsConnector};

pub fn build_tls_connector() -> Result<TlsConnector, native_tls::Error> {
    let root_ca = include_bytes!("certs/signal-messenger.pem");
    let root_ca = Certificate::from_pem(root_ca).unwrap();
    let mut builder = TlsConnector::builder();
    builder.disable_built_in_roots(true);
    builder.add_root_certificate(root_ca);
    builder.build()
}

#[cfg(test)]
mod tests {
    use std::net::TcpStream;

    use super::*;

    #[test]
    fn connect_trusted_server() {
        let builder = build_tls_connector().unwrap();
        let s = TcpStream::connect("chat.staging.signal.org:443").unwrap();
        builder.connect("chat.staging.signal.org", s).unwrap();
    }

    #[test]
    fn connect_untrusted_server() {
        let builder = build_tls_connector().unwrap();
        let s = TcpStream::connect("signal.org:443").unwrap();
        builder.connect("signal.org", s).unwrap_err();
    }
}

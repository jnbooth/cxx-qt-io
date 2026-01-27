use cxx_qt_io::QSslConfiguration;

#[cxx::bridge]
mod ffi {
    #[namespace = "ffi::qsslconfiguration"]
    extern "Rust" {
        fn alpn_protocol() -> &'static str;
        fn next_protocol() -> &'static str;
    }
}

fn alpn_protocol() -> &'static str {
    QSslConfiguration::ALPNProtocolHTTP2
}

fn next_protocol() -> &'static str {
    QSslConfiguration::NextProtocolHttp1_1
}

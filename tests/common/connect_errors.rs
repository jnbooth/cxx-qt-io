#![allow(clippy::print_stderr)]
use std::pin::Pin;

pub trait ConnectErrors {
    fn connect_errors(self: Pin<&mut Self>, context: &'static str);
}

#[cfg(feature = "net")]
impl ConnectErrors for cxx_qt_io::QTcpServer {
    fn connect_errors(self: Pin<&mut Self>, context: &'static str) {
        self.on_accept_error(move |_, error| {
            eprintln!("[{context}] QTcpServer::accept_error: {error}");
        })
        .release();
    }
}

#[cfg(feature = "ssl")]
impl ConnectErrors for cxx_qt_io::QSslServer {
    fn connect_errors(mut self: Pin<&mut Self>, context: &'static str) {
        self.as_mut()
            .on_error_occurred(move |_, _, error| {
                eprintln!("[{context}] QSslServer::error_occurred: {error}");
            })
            .release();

        self.as_mut()
            .on_handshake_interrupted_on_error(move |_, _, error| {
                eprintln!("[{context}] QSslServer::handshake_interrupted_on_error: {error}");
            })
            .release();

        self.as_mut()
            .on_peer_verify_error(move |_, _, error| {
                eprintln!("[{context}] QSslServer::peer_verify_error: {error}");
            })
            .release();

        self.as_mut()
            .on_ssl_errors(move |_, _, errors| {
                for error in errors {
                    eprintln!("[{context}] QSslServer::ssl_errors: {error}");
                }
            })
            .release();

        self.as_tcp_server_mut().connect_errors(context);
    }
}

#[cfg(feature = "net")]
impl ConnectErrors for cxx_qt_io::QAbstractSocket {
    fn connect_errors(self: Pin<&mut Self>, context: &'static str) {
        self.on_error_occurred(move |_, error| {
            eprintln!("[{context}] QAbstractSocket::error_occurred: {error}");
        })
        .release();
    }
}

#[cfg(feature = "net")]
impl ConnectErrors for cxx_qt_io::QTcpSocket {
    fn connect_errors(mut self: Pin<&mut Self>, context: &'static str) {
        self.as_abstract_socket_mut().connect_errors(context);
    }
}

#[cfg(feature = "ssl")]
impl ConnectErrors for cxx_qt_io::QSslSocket {
    fn connect_errors(mut self: Pin<&mut Self>, context: &'static str) {
        self.as_mut()
            .on_handshake_interrupted_on_error(move |_, error| {
                eprintln!("[{context}] QSslSocket::handshake_interrupted_on_error: {error}");
            })
            .release();

        self.as_mut()
            .on_peer_verify_error(move |_, error| {
                eprintln!("[{context}] QSslSocket::peer_verify_error: {error}");
            })
            .release();

        self.as_mut()
            .on_ssl_errors(move |_, errors| {
                for error in errors {
                    eprintln!("[{context}] QSslSocket::ssl_errors: {error}");
                }
            })
            .release();

        self.as_tcp_socket_mut().connect_errors(context);
    }
}

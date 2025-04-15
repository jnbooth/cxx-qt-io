use cxx_qt_io::{QAbstractSocket, QSslServer, QSslSocket, QTcpServer, QTcpSocket};
use std::pin::Pin;

pub trait ConnectErrors {
    fn connect_errors(self: Pin<&mut Self>);
}

impl ConnectErrors for QTcpServer {
    fn connect_errors(self: Pin<&mut Self>) {
        self.on_accept_error(|_, error| {
            eprintln!("QTcpServer::accept_error: {error:?}");
        })
        .release();
    }
}

impl ConnectErrors for QSslServer {
    fn connect_errors(mut self: Pin<&mut Self>) {
        self.as_mut()
            .on_error_occurred(|_, _, error| {
                eprintln!("QSslServer::error_occurred: {error:?}");
            })
            .release();

        self.as_mut()
            .on_handshake_interrupted_on_error(|_, _, error| {
                eprintln!("QSslServer::handshake_interrupted_on_error: {error:?}");
            })
            .release();

        self.as_mut()
            .on_peer_verify_error(|_, _, error| {
                eprintln!("QSslServer::peer_verify_error: {error:?}");
            })
            .release();

        self.as_mut()
            .on_ssl_errors(|_, _, errors| {
                for error in errors.iter() {
                    eprintln!("QSslServer::ssl_errors: {error:?}");
                }
            })
            .release();

        self.as_tcp_server_mut().connect_errors();
    }
}

impl ConnectErrors for QAbstractSocket {
    fn connect_errors(self: Pin<&mut Self>) {
        self.on_error_occurred(|_, error| {
            eprintln!("QAbstractSocket::connect_errors: {error:?}");
        })
        .release();
    }
}

impl ConnectErrors for QTcpSocket {
    fn connect_errors(mut self: Pin<&mut Self>) {
        self.as_abstract_socket_mut().connect_errors();
    }
}

impl ConnectErrors for QSslSocket {
    fn connect_errors(mut self: Pin<&mut Self>) {
        self.as_mut()
            .on_handshake_interrupted_on_error(|_, error| {
                eprintln!("QSslSocket::handshake_interrupted_on_error: {error:?}");
            })
            .release();

        self.as_mut()
            .on_peer_verify_error(|_, error| {
                eprintln!("QSslSocket::peer_verify_error: {error:?}");
            })
            .release();

        self.as_mut()
            .on_ssl_errors(|_, errors| {
                for error in errors.iter() {
                    eprintln!("QSslSocket::ssl_errors: {error:?}");
                }
            })
            .release();

        self.as_tcp_socket_mut().connect_errors();
    }
}

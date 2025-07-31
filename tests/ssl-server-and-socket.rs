#![cfg(feature = "ssl")]
mod common;
use std::time::{Duration, Instant};

use common::{ConnectErrors, run_inside_app};
use cxx_qt_io::{
    QAbstractSocketSocketError, QAbstractSocketSocketState, QHostAddressSpecialAddress, QIODevice,
    QSslCertificate, QSslConfiguration, QSslError, QSslErrorSslError, QSslKey, QSslServer,
    QSslSocket,
};
use cxx_qt_lib::{QByteArray, QString};
use cxx_qt_lib_extras::QEventLoop;

const CERT: &[u8] = include_bytes!("local.crt");
const KEY: &[u8] = include_bytes!("local.key");
const PORT: u16 = 8011;
const TIMEOUT: Duration = Duration::from_secs(500);

#[test]
#[allow(clippy::expect_used)]
#[allow(clippy::unwrap_used)]
fn ssl_round_trip() {
    init_crates!();
    run_inside_app(|| {
        let cert =
            QSslCertificate::try_from(&QByteArray::from(CERT)).expect("invalid certificate file");
        let key = QSslKey::try_from(&QByteArray::from(KEY)).expect("invalid key file");

        let mut server_ptr = QSslServer::new();
        let mut server = server_ptr.pin_mut();
        server.as_mut().connect_errors("server");

        let mut server_config = QSslConfiguration::default_configuration();
        server_config.set_local_certificate(&cert);
        server_config.set_private_key(&key);
        server.as_mut().set_ssl_configuration(&server_config);
        server.as_mut().set_handshake_timeout(TIMEOUT);

        let mut client_socket_ptr = QSslSocket::new();
        let mut client_socket = client_socket_ptr.pin_mut();
        client_socket.as_mut().connect_errors("client_socket");

        client_socket
            .as_mut()
            .on_ssl_errors(move |mut client_socket, errors| {
                let errors = errors
                    .iter()
                    .map(QSslError::error)
                    .filter(|&error| {
                        !matches!(
                            error,
                            QSslErrorSslError::CertificateUntrusted
                                | QSslErrorSslError::SelfSignedCertificate,
                        )
                    })
                    .collect::<Vec<_>>();
                assert_eq!(errors, Vec::new());
                client_socket.as_mut().ignore_all_ssl_errors();
            })
            .release();

        client_socket
            .as_abstract_socket_mut()
            .on_error_occurred(|_, error| {
                assert!(
                    !(error == QAbstractSocketSocketError::SslHandshakeFailedError),
                    "handshake failed"
                );
            })
            .release();

        server
            .as_tcp_server_mut()
            .listen(&QHostAddressSpecialAddress::Any.into(), PORT);

        client_socket.as_mut().connect_to_host_encrypted(
            &QString::from("localhost"),
            PORT,
            QIODevice::ReadWrite,
        );

        assert!(
            wait_for_encrypted(&client_socket, TIMEOUT),
            "SSL handshake timed out"
        );

        assert_eq!(
            client_socket.state(),
            QAbstractSocketSocketState::ConnectedState
        );
    });
}

fn wait_for_encrypted(socket: &QSslSocket, timeout: Duration) -> bool {
    let now = Instant::now();
    let mut event_loop_ptr = QEventLoop::new();
    let mut event_loop = event_loop_ptr.pin_mut();
    loop {
        event_loop.as_mut().process_all_events();
        if socket.is_encrypted() {
            return true;
        }
        if now.elapsed() > timeout {
            return false;
        }
    }
}

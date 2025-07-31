#![cfg(feature = "net")]
mod common;
use std::io::{Read, Write};
use std::time::Duration;

use common::{run_inside_app, ConnectErrors};
use cxx_qt_io::{QHostAddressSpecialAddress, QIODevice, QTcpServer, QTcpSocket};

const PORT: u16 = 8012;
const TIMEOUT: Option<Duration> = Some(Duration::from_secs(500));

#[test]
#[allow(clippy::expect_used)]
#[allow(clippy::unwrap_used)]
fn tcp_round_trip() {
    init_crates!();
    run_inside_app(|| {
        let mut server_ptr = QTcpServer::new();
        let mut socket_ptr = QTcpSocket::new();
        let mut server = server_ptr.pin_mut();
        let mut client_socket = socket_ptr.pin_mut();
        server.as_mut().connect_errors("server");
        client_socket.as_mut().connect_errors("client_socket");

        let addr = QHostAddressSpecialAddress::LocalHost.into();

        server.as_mut().listen(&addr, PORT);

        client_socket
            .as_abstract_socket_mut()
            .connect_to_host((addr, PORT), QIODevice::ReadWrite);

        assert!(
            server.as_mut().wait_for_new_connection(TIMEOUT),
            "failed to acquire connection"
        );

        let mut server_socket_ptr = server.as_mut().next_pending_connection();

        let mut server_socket = server_socket_ptr.as_mut().expect("received null socket");

        server_socket.as_mut().connect_errors("server_socket");

        client_socket
            .as_abstract_socket_mut()
            .wait_for_connected(TIMEOUT);
        client_socket.write_all(b"test message").unwrap();
        client_socket.flush().unwrap();

        let mut buf = Vec::new();
        server_socket
            .as_io_device_mut()
            .wait_for_ready_read(TIMEOUT);
        server_socket.read_to_end(&mut buf).unwrap();

        server_socket.write_all(&buf).unwrap();
        server_socket.write_all(b" response").unwrap();
        buf.clear();
        server_socket.flush().unwrap();

        client_socket
            .as_io_device_mut()
            .wait_for_ready_read(TIMEOUT);
        client_socket.read_to_end(&mut buf).unwrap();

        assert_eq!(String::from_utf8_lossy(&buf), "test message response");
    });
}

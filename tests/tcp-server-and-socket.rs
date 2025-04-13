#![cfg(feature = "qt_network")]
use std::io::{Read, Write};
use std::time::Duration;

use cxx::UniquePtr;

use cxx_qt_io::{QHostAddressSpecialAddress, QIODevice, QTcpServer, QTcpSocket};

#[test]
fn round_trip() {
    let mut server_ptr = QTcpServer::new();
    let mut socket_ptr = QTcpSocket::new();
    let mut server = server_ptr.pin_mut();
    let mut client_socket = socket_ptr.pin_mut();

    let addr = QHostAddressSpecialAddress::LocalHost.into();
    let port = 8010;
    let timeout = Some(Duration::from_millis(500));

    server.as_mut().listen(&addr, port);

    client_socket
        .as_abstract_socket_mut()
        .connect_to_host((addr, port), QIODevice::ReadWrite);

    server.as_mut().wait_for_new_connection(timeout);

    let mut server_socket_ptr =
        unsafe { UniquePtr::from_raw(server.as_mut().next_pending_connection()) };

    let mut server_socket = server_socket_ptr.pin_mut();

    client_socket
        .as_abstract_socket_mut()
        .wait_for_connected(timeout);
    client_socket.write_all(b"test message").unwrap();
    client_socket.flush().unwrap();

    let mut buf = Vec::new();
    server_socket
        .as_io_device_mut()
        .wait_for_ready_read(timeout);
    server_socket.read_to_end(&mut buf).unwrap();

    server_socket.write_all(&buf).unwrap();
    server_socket.write_all(b" response").unwrap();
    buf.clear();
    server_socket.flush().unwrap();

    client_socket
        .as_io_device_mut()
        .wait_for_ready_read(timeout);
    client_socket.read_to_end(&mut buf).unwrap();

    assert_eq!(String::from_utf8_lossy(&buf), "test message response");
}

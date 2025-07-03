use std::ops::Deref;
use std::pin::Pin;
use std::time::Duration;

use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt::QObject;

use crate::util::{IsNonNull, MSecs};
use crate::{QHostAddress, QTcpSocket, SocketDescriptor};

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qtypes.h");
        type qintptr = cxx_qt_lib::qintptr;
        include!("cxx-qt-io/qabstractsocket.h");
        type QAbstractSocketSocketError = crate::QAbstractSocketSocketError;
        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddress = crate::QHostAddress;
        include!("cxx-qt-io/qnetworkproxy.h");
        type QNetworkProxy = crate::QNetworkProxy;

    }

    extern "C++" {
        include!("cxx-qt-io/qtcpserver.h");
        type QTcpSocket = crate::QTcpSocket;
    }

    unsafe extern "C++Qt" {
        /// The `QTcpServer` class provides a TCP-based server.
        ///
        /// Qt Documentation: [QTcpServer](https://doc.qt.io/qt-6/qtcpserver.html#details)
        #[qobject]
        #[base = QObject]
        type QTcpServer;

        /// Closes the server. The server will no longer listen for incoming connections.
        fn close(self: Pin<&mut QTcpServer>);

        /// Returns a human readable description of the last error that occurred.
        #[rust_name = "error_string"]
        fn errorString(self: &QTcpServer) -> QString;

        /// Returns `true` if the server has a pending connection; otherwise returns `false`.
        #[rust_name = "has_pending_connections"]
        fn hasPendingConnections(self: &QTcpServer) -> bool;

        /// Returns `true` if the server is currently listening for incoming connections; otherwise returns `false`.
        #[rust_name = "is_listening"]
        fn isListening(self: &QTcpServer) -> bool;

        /// Tells the server to listen for incoming connections on address `address` and port `port`. If `port` is 0, a port is chosen automatically. If address is [`QHostAddressSpecialAddress::Any`](crate::QHostAddressSpecialAddress::Any), the server will listen on all network interfaces.
        ///
        /// Returns `true` on success; otherwise returns `false`.
        fn listen(self: Pin<&mut QTcpServer>, address: &QHostAddress, port: u16) -> bool;

        /// Returns the backlog queue size of to be accepted connections.
        ///
        /// Introduced in Qt 6.3.
        #[cfg(cxxqt_qt_version_at_least_6_3)]
        #[rust_name = "listen_backlog_size"]
        fn listenBacklogSize(self: &QTcpServer) -> i32;

        /// Returns the maximum number of pending accepted connections. The default is 30.
        #[rust_name = "max_pending_connections"]
        fn maxPendingConnections(self: &QTcpServer) -> i32;

        #[doc(hidden)]
        #[rust_name = "next_pending_connection_raw"]
        fn nextPendingConnection(self: Pin<&mut QTcpServer>) -> *mut QTcpSocket;

        /// Pauses accepting new connections. Queued connections will remain in queue.
        #[rust_name = "pause_accepting"]
        fn pauseAccepting(self: Pin<&mut QTcpServer>);

        /// Returns the network proxy for this socket. By default [`QNetworkProxyProxyType::DefaultProxy`](crate::QNetworkProxyProxyType::DefaultProxy) is used.
        fn proxy(self: &QTcpServer) -> QNetworkProxy;

        /// Resumes accepting new connections.
        #[rust_name = "resume_accepting"]
        fn resumeAccepting(self: Pin<&mut QTcpServer>);

        #[doc(hidden)]
        #[rust_name = "server_address_or_null"]
        fn serverAddress(self: &QTcpServer) -> QHostAddress;

        /// Returns an error code for the last error that occurred.
        #[rust_name = "server_error"]
        fn serverError(self: &QTcpServer) -> QAbstractSocketSocketError;

        /// Returns the server's port if the server is listening for connections; otherwise returns 0.
        #[rust_name = "server_port"]
        fn serverPort(self: &QTcpServer) -> u16;

        /// Sets the backlog queue size of to be accepted connections to `size`. The operating system might reduce or ignore this value. By default, the queue size is 50.
        ///
        /// **Note:** This property must be set prior to calling [`listen`](QTcpServer::listen).
        ///
        /// Introduced in Qt 6.3.
        #[cfg(cxxqt_qt_version_at_least_6_3)]
        #[rust_name = "set_listen_backlog_size"]
        fn setListenBacklogSize(self: Pin<&mut QTcpServer>, size: i32);

        /// Sets the maximum number of pending accepted connections to `num_connections`. `QTcpServer` will accept no more than `num_connections` incoming connections before [`next_pending_connection`](QTcpServer::next_pending_connection) is called. By default, the limit is 30 pending connections.
        ///
        /// Clients may still able to connect after the server has reached its maximum number of pending connections (i.e., [`QTcpSocket`](crate::QTcpSocket) can still emit the [`connected`](crate::QAbstractSocket::connected) signal). `QTcpServer` will stop accepting the new connections, but the operating system may still keep them in queue.
        #[rust_name = "set_max_pending_connections"]
        fn setMaxPendingConnections(self: Pin<&mut QTcpServer>, num_connections: i32);

        /// Sets the explicit network proxy for this socket to `network_proxy`.
        ///
        /// To disable the use of a proxy for this socket, use the [`QNetworkProxyProxyType::NoProxy`](crate::QNetworkProxyProxyType::NoProxy) proxy type.
        #[rust_name = "set_proxy"]
        fn setProxy(self: Pin<&mut QTcpServer>, network_proxy: &QNetworkProxy);

        #[doc(hidden)]
        #[rust_name = "set_socket_descriptor_qintptr"]
        fn setSocketDescriptor(self: Pin<&mut QTcpServer>, socket_descriptor: qintptr) -> bool;

        #[doc(hidden)]
        #[rust_name = "socket_descriptor_or_negative"]
        fn socketDescriptor(self: &QTcpServer) -> qintptr;

        /// # Safety
        ///
        /// `timed_out` must be valid or null.
        #[doc(hidden)]
        #[rust_name = "wait_for_new_connection_msec"]
        unsafe fn waitForNewConnection(
            self: Pin<&mut QTcpServer>,
            msec: i32,
            timed_out: *mut bool,
        ) -> bool;

        /// This signal is emitted when accepting a new connection results in an error. The `socket_error` parameter describes the type of error that occurred.
        #[qsignal]
        #[rust_name = "accept_error"]
        fn acceptError(self: Pin<&mut QTcpServer>, socket_error: QAbstractSocketSocketError);

        /// This signal is emitted every time a new connection is available, regardless of whether it has been added to the pending connections queue or not.
        #[qsignal]
        #[rust_name = "new_connection"]
        fn newConnection(self: Pin<&mut QTcpServer>);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qtcpserver_init_default"]
        fn make_unique() -> UniquePtr<QTcpServer>;
    }
}

pub use ffi::QTcpServer;

impl QTcpServer {
    /// Constructs a `QTcpServer` object.
    pub fn new() -> UniquePtr<Self> {
        ffi::qtcpserver_init_default()
    }

    /// Returns the next pending connection as a connected [`QTcpSocket`] object.
    ///
    /// A null pointer is returned if this function is called when there are no pending connections.
    ///
    /// **Note:** The returned [`QTcpSocket`] object cannot be used from another thread.
    pub fn next_pending_connection(self: Pin<&mut Self>) -> UniquePtr<QTcpSocket> {
        let conn = self.next_pending_connection_raw();
        // SAFETY: `conn` is valid and Qt expects us to delete it when done.
        unsafe { UniquePtr::from_raw(conn) }
    }

    /// Returns the server's address if the server is listening for connections; otherwise returns `None`.
    pub fn server_address(&self) -> Option<QHostAddress> {
        self.server_address_or_null().nonnull()
    }

    /// Sets the socket descriptor this server should use when listening for incoming connections to `socket_descriptor`. Returns `true` if the socket is set successfully; otherwise returns `false`.
    ///
    /// The socket is assumed to be in listening state.
    pub fn set_socket_descriptor(
        self: Pin<&mut Self>,
        socket_descriptor: SocketDescriptor,
    ) -> bool {
        self.set_socket_descriptor_qintptr(socket_descriptor.into())
    }

    /// Returns the native socket descriptor the server uses to listen for incoming instructions, or `None` if the server is not listening.
    ///
    /// If the socket is using [`QNetworkProxy`](crate::QNetworkProxy), the returned descriptor may not be usable with native socket functions.
    pub fn socket_descriptor(&self) -> Option<SocketDescriptor> {
        SocketDescriptor::from(self.socket_descriptor_or_negative()).nonnull()
    }

    /// Waits for at most `duration` or until an incoming connection is available. Returns `true` if a connection is available; otherwise returns `false`.
    ///
    /// This is a blocking function call. Its use is disadvised in a single-threaded GUI application, since the whole application will stop responding until the function returns. This function is mostly useful when there is no event loop available.
    ///
    /// The non-blocking alternative is to connect to the [`new_connection`](QTcpServer::new_connection) signal.
    ///
    /// If `duration` is `None`, this function will not time out.
    pub fn wait_for_new_connection(self: Pin<&mut Self>, duration: Option<Duration>) -> bool {
        // SAFETY: Qt ignores the null pointer.
        unsafe { self.wait_for_new_connection_msec(duration.msecs(), std::ptr::null_mut()) }
    }
}

impl Deref for QTcpServer {
    type Target = QObject;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

#[cfg(test)]
mod tests {
    use cxx_qt_lib::QString;

    use super::*;
    use crate::QNetworkProxy;

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QTcpServerProps {
            #[cfg(cxxqt_qt_version_at_least_6_3)]
            listen_backlog_size: i32,
            max_pending_connections: i32,
            proxy: QNetworkProxy,
        }

        let mut proxy = QNetworkProxy::default();
        proxy.set_host_name(&QString::from("host"));

        let props = QTcpServerProps {
            #[cfg(cxxqt_qt_version_at_least_6_3)]
            listen_backlog_size: 10,
            max_pending_connections: 15,
            proxy,
        };

        let mut server = QTcpServer::new();

        #[cfg(cxxqt_qt_version_at_least_6_3)]
        server
            .pin_mut()
            .set_listen_backlog_size(props.listen_backlog_size);
        server
            .pin_mut()
            .set_max_pending_connections(props.max_pending_connections);
        server.pin_mut().set_proxy(&props.proxy);

        let actual_props = QTcpServerProps {
            #[cfg(cxxqt_qt_version_at_least_6_3)]
            listen_backlog_size: server.listen_backlog_size(),
            max_pending_connections: server.max_pending_connections(),
            proxy: server.proxy(),
        };

        assert_eq!(actual_props, props);
    }
}

use std::pin::Pin;
use std::time::Duration;

use crate::util::{IsNonNull, MSecs};
use crate::{QHostAddress, SocketDescriptor};

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
        #[cfg(cxxqt_qt_version_at_least_6_3)]
        #[rust_name = "listen_backlog_size"]
        fn listenBacklogSize(self: &QTcpServer) -> i32;

        /// Returns the maximum number of pending accepted connections. The default is 30.
        #[rust_name = "max_pending_connections"]
        fn maxPendingConnections(self: &QTcpServer) -> i32;

        /// Returns the next pending connection as a connected [`QTcpSocket`] object.
        ///
        /// The socket is created as a child of the server, which means that it is automatically deleted when the QTcpServer object is destroyed. It is still a good idea to delete the object explicitly when you are done with it, to avoid wasting memory.
        ///
        /// A null pointer is returned if this function is called when there are no pending connections.
        ///
        /// **Note:** The returned [`QTcpSocket`] object cannot be used from another thread.
        #[rust_name = "next_pending_connection"]
        fn nextPendingConnection(self: Pin<&mut QTcpServer>) -> *mut QTcpSocket;

        /// Pauses accepting new connections. Queued connections will remain in queue.
        #[rust_name = "pause_accepting"]
        fn pauseAccepting(self: Pin<&mut QTcpServer>);

        /// Returns the network proxy for this socket. By default [`QNetworkProxyProxyType::DefaultProxy`](crate::QNetworkProxyProxyType::DefaultProxy) is used.
        fn proxy(self: &QTcpServer) -> QNetworkProxy;

        /// Resumes accepting new connections.
        #[rust_name = "resume_accepting"]
        fn resumeAccepting(self: Pin<&mut QTcpServer>);

        #[rust_name = "server_address_or_null"]
        pub(self) fn serverAddress(self: &QTcpServer) -> QHostAddress;

        /// Returns an error code for the last error that occurred.
        #[rust_name = "server_error"]
        fn serverError(self: &QTcpServer) -> QAbstractSocketSocketError;

        /// Returns the server's port if the server is listening for connections; otherwise returns 0.
        #[rust_name = "server_port"]
        fn serverPort(self: &QTcpServer) -> u16;

        /// Sets the backlog queue size of to be accepted connections to `size`. The operating system might reduce or ignore this value. By default, the queue size is 50.
        ///
        /// **Note:** This property must be set prior to calling [`listen`](QTcpServer::listen).
        #[cfg(cxxqt_qt_version_at_least_6_3)]
        #[rust_name = "set_listen_backlog_size"]
        fn setListenBacklogSize(self: Pin<&mut QTcpServer>, size: i32);

        /// Sets the maximum number of pending accepted connections to `num_connections`. `QTcpServer` will accept no more than `num_connections` incoming connections before [`next_pending_connection`](QTcpServer::next_pending_connection) is called. By default, the limit is 30 pending connections.
        ///
        /// Clients may still able to connect after the server has reached its maximum number of pending connections (i.e., `QTcpSocket` can still emit the [`connected`](crate::QAbstractSocket::connected) signal). `QTcpServer` will stop accepting the new connections, but the operating system may still keep them in queue.
        #[rust_name = "set_max_pending_connections"]
        fn setMaxPendingConnections(self: Pin<&mut QTcpServer>, num_connections: i32);

        /// Sets the explicit network proxy for this socket to `network_proxy`.
        ///
        /// To disable the use of a proxy for this socket, use the [`QNetworkProxyProxyType::NoProxy`](crate::QNetworkProxyProxyType::NoProxy) proxy type.
        #[rust_name = "set_proxy"]
        fn setProxy(self: Pin<&mut QTcpServer>, network_proxy: &QNetworkProxy);

        #[rust_name = "set_socket_descriptor_qintptr"]
        pub(self) fn setSocketDescriptor(
            self: Pin<&mut QTcpServer>,
            socket_descriptor: qintptr,
        ) -> bool;

        #[rust_name = "socket_descriptor_or_negative"]
        pub(self) fn socketDescriptor(self: &QTcpServer) -> qintptr;

        /// # Safety
        ///
        /// `timed_out` must be valid or null.
        #[rust_name = "wait_for_new_connection_msec"]
        pub(self) unsafe fn waitForNewConnection(
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
}

pub use ffi::QTcpServer;

impl QTcpServer {
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
    pub fn wait_for_new_connection(self: Pin<&mut QTcpServer>, duration: Option<Duration>) -> bool {
        // SAFETY: Qt ignores the null pointer.
        unsafe { self.wait_for_new_connection_msec(duration.msecs(), std::ptr::null_mut()) }
    }
}

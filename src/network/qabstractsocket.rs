use crate::qio::{QIOExt, QIO};
use crate::util::{MSecs, NonNull};
use crate::{QHostAddress, QIODevice};
use cxx_qt::Upcast;
use cxx_qt_lib::{QFlags, QString, QVariant};
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;
use std::time::Duration;

#[cxx_qt::bridge]
mod ffi {
    /// This enum describes the network layer protocol values used in Qt.
    #[repr(i32)]
    #[derive(Debug)]
    enum QAbstractSocketNetworkLayerProtocol {
        /// IPv4
        IPv4Protocol,
        /// IPv6
        IPv6Protocol,
        /// Either IPv4 or IPv6
        AnyIPProtocol,
        /// Other than IPv4 and IPv6
        UnknownNetworkLayerProtocol = -1,
    }

    /// This enum describes the different flags you can pass to modify the behavior of `QAbstractSocket::bind()`.
    #[repr(i32)]
    #[derive(Debug)]
    enum QAbstractSocketBindFlag {
        /// The default option for the current platform. On Unix and macOS, this is equivalent to (`DontShareAddress + ReuseAddressHint`), and on Windows, it is equivalent to `ShareAddress`.
        DefaultForPlatform = 0x0,
        /// Allow other services to bind to the same address and port. This is useful when multiple processes share the load of a single service by listening to the same address and port (e.g., a web server with several pre-forked listeners can greatly improve response time). However, because any service is allowed to rebind, this option is subject to certain security considerations. Note that by combining this option with `ReuseAddressHint`, you will also allow your service to rebind an existing shared address. On Unix, this is equivalent to the `SO_REUSEADDR` socket option. On Windows, this is the default behavior, so this option is ignored.
        ShareAddress = 0x1,
        /// Bind the address and port exclusively, so that no other services are allowed to rebind. By passing this option to `QAbstractSocket::bind()`, you are guaranteed that on success, your service is the only one that listens to the address and port. No services are allowed to rebind, even if they pass `ReuseAddressHint`. This option provides more security than `ShareAddress`, but on certain operating systems, it requires you to run the server with administrator privileges. On Unix and macOS, not sharing is the default behavior for binding an address and port, so this option is ignored. On Windows, this option uses the `SO_EXCLUSIVEADDRUSE` socket option.
        DontShareAddress = 0x2,
        /// Provides a hint to `QAbstractSocket` that it should try to rebind the service even if the address and port are already bound by another socket. On Windows and Unix, this is equivalent to the `SO_REUSEADDR` socket option.
        ReuseAddressHint = 0x4,
    }

    /// This enum describes the socket errors that can occur.
    #[repr(i32)]
    #[derive(Debug)]
    enum QAbstractSocketSocketError {
        /// The connection was refused by the peer (or timed out).
        ConnectionRefusedError,
        /// The remote host closed the connection. Note that the client socket (i.e., this socket) will be closed after the remote close notification has been sent.
        RemoteHostClosedError,
        /// The host address was not found.
        HostNotFoundError,
        /// The socket operation failed because the application lacked the required privileges.
        SocketAccessError,
        /// The local system ran out of resources (e.g., too many sockets).
        SocketResourceError,
        /// The socket operation timed out.
        SocketTimeoutError,
        /// The datagram was larger than the operating system's limit (which can be as low as 8192 bytes).
        DatagramTooLargeError,
        /// An error occurred with the network (e.g., the network cable was accidentally plugged out).
        NetworkError,
        /// The address specified to `QAbstractSocket::bind()` is already in use and was set to be exclusive.
        AddressInUseError,
        /// The address specified to `QAbstractSocket::bind()` does not belong to the host.
        SocketAddressNotAvailableError,
        /// The requested socket operation is not supported by the local operating system (e.g., lack of IPv6 support).
        UnsupportedSocketOperationError,
        /// The socket is using a proxy, and the proxy requires authentication.
        UnfinishedSocketOperationError,
        /// The SSL/TLS handshake failed, so the connection was closed (only used in `QSslSocket`).
        ProxyAuthenticationRequiredError,
        /// Used by `QAbstractSocketEngine` only, The last operation attempted has not finished yet (still in progress in the background).
        SslHandshakeFailedError,
        /// Could not contact the proxy server because the connection to that server was denied.
        ProxyConnectionRefusedError,
        /// The connection to the proxy server was closed unexpectedly (before the connection to the final peer was established).
        ProxyConnectionClosedError,
        /// The connection to the proxy server timed out or the proxy server stopped responding in the authentication phase.
        ProxyConnectionTimeoutError,
        /// The proxy address set with `set_proxy()` (or the application proxy) was not found.
        ProxyNotFoundError,
        /// The connection negotiation with the proxy server failed, because the response from the proxy server could not be understood.
        ProxyProtocolError,
        /// An operation was attempted while the socket was in a state that did not permit it.
        OperationError,
        /// The SSL library being used reported an internal error. This is probably the result of a bad installation or misconfiguration of the library.
        SslInternalError,
        /// Invalid data (certificate, key, cypher, etc.) was provided and its use resulted in an error in the SSL library.
        SslInvalidUserDataError,
        /// A temporary error occurred (e.g., operation would block and socket is non-blocking).
        TemporaryError,
        /// An unidentified error occurred.
        UnknownSocketError = -1,
    }

    /// This enum represents the options that can be set on a socket. If desired, they can be set after having received the `connected()` signal from the socket or after having received a new socket from a `QTcpServer`.
    #[repr(i32)]
    #[derive(Debug)]
    enum QAbstractSocketSocketOption {
        /// Try to optimize the socket for low latency. For a `QTcpSocket` this would set the `TCP_NODELAY` option and disable Nagle's algorithm. Set this to 1 to enable.
        LowDelayOption,
        /// Set this to 1 to enable the `SO_KEEPALIVE` socket option.
        KeepAliveOption,
        /// Set this to an integer value to set `IP_MULTICAST_TTL` (TTL for multicast datagrams) socket option.
        MulticastTtlOption,
        /// Set this to 1 to enable the `IP_MULTICAST_LOOP` (multicast loopback) socket option.
        MulticastLoopbackOption,
        /// This option is not supported on Windows. This maps to the `IP_TOS` socket option. For possible values, see table below.
        ///
        /// | Value | Description          |
        /// | ----- | -------------------- |
        /// | 224   | Network control      |
        /// | 192   | Internetwork control |
        /// | 160   | CRITIC/ECP           |
        /// | 128   | Flash override       |
        /// | 96    | Flash                |
        /// | 64    | Immediate            |
        /// | 32    | Priority             |
        /// | 0     | Routine              |
        TypeOfServiceOption,
        /// Sets the socket send buffer size in bytes at the OS level. This maps to the `SO_SNDBUF` socket option. This option does not affect the `QIODevice` or `QAbstractSocket` buffers.
        SendBufferSizeSocketOption,
        /// Sets the socket receive buffer size in bytes at the OS level. This maps to the `SO_RCVBUF` socket option. This option does not affect the `QIODevice` or `QAbstractSocket` buffers (see `set_read_buffer_size()`).
        ReceiveBufferSizeSocketOption,
        /// Retrieves the Path Maximum Transmission Unit (PMTU) value currently known by the IP stack, if any. Some IP stacks also allow setting the MTU for transmission.
        PathMtuSocketOption,
    }

    /// This enum describes the behavior of when the socket should hold back with continuing data transfer. The only notification currently supported is `QSslSocket::ssl_errors()`.
    #[repr(i32)]
    #[derive(Debug)]
    enum QAbstractSocketPauseMode {
        /// Do not pause data transfer on the socket. This is the default and matches the behavior of Qt 4.
        PauseNever = 0x0,
        /// Pause data transfer on the socket upon receiving an SSL error notification. I.E. `QSslSocket::ssl_errors()`.
        PauseOnSslErrors = 0x1,
    }

    /// This enum describes the different states in which a socket can be.
    #[repr(i32)]
    #[derive(Debug)]
    enum QAbstractSocketSocketState {
        /// The socket is not connected.
        UnconnectedState,
        /// The socket is performing a host name lookup.
        HostLookupState,
        /// The socket has started establishing a connection.
        ConnectingState,
        /// A connection is established.
        ConnectedState,
        /// The socket is bound to an address and port.
        BoundState,
        /// The socket is about to close (data may still be waiting to be written).
        ListeningState,
        /// For internal use only.
        ClosingState,
    }

    /// This enum describes the transport layer protocol.
    #[repr(i32)]
    #[derive(Debug)]
    enum QAbstractSocketSocketType {
        /// TCP
        TcpSocket,
        /// UDP
        UdpSocket,
        /// SCTP
        SctpSocket,
        /// Other than TCP, UDP, and SCTP
        UnknownSocketType = -1,
    }

    extern "C++" {
        include!(<QtNetwork/QAuthenticator>);
        type QAuthenticator = crate::QAuthenticator;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
        include!("cxx-qt-io/qiodevice.h");
        type QIODeviceOpenMode = crate::QIODeviceOpenMode;
        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddress = crate::QHostAddress;
        include!("cxx-qt-io/qnetworkproxy.h");
        type QNetworkProxy = crate::QNetworkProxy;
    }

    extern "C++" {
        include!("cxx-qt-io/qabstractsocket.h");
        type QAbstractSocketNetworkLayerProtocol;
        type QAbstractSocketSocketType;
        type QAbstractSocketSocketError;
        type QAbstractSocketSocketState;
        type QAbstractSocketSocketOption;
        type QAbstractSocketBindFlag;
        type QAbstractSocketBindMode = super::QAbstractSocketBindMode;
        type QAbstractSocketPauseMode;
        type QAbstractSocketPauseModes = super::QAbstractSocketPauseModes;
    }

    unsafe extern "C++Qt" {
        type QIODevice = crate::QIODevice;

        /// The `QAbstractSocket` class provides the base functionality common to all socket types.
        ///
        /// Qt Documentation: [QAbstractSocket](https://doc.qt.io/qt-6/qabstractsocket.html#details)
        #[qobject]
        #[base = QIODevice]
        type QAbstractSocket;

        /// Aborts the current connection and resets the socket. Unlike `disconnect_from_host()`, this function immediately closes the socket, discarding any pending data in the write buffer.
        fn abort(self: Pin<&mut QAbstractSocket>);

        /// Binds to address on port `port`, using the `BindMode` `mode`.
        ///
        /// For UDP sockets, after binding, the signal `QUdpSocket::ready_read()` is emitted whenever a UDP datagram arrives on the specified address and port. Thus, this function is useful to write UDP servers.
        ///
        /// For TCP sockets, this function may be used to specify which interface to use for an outgoing connection, which is useful in case of multiple network interfaces.
        ///
        /// On success, the function returns `true` and the socket enters `BoundState`; otherwise it returns `false`.
        fn bind(
            self: Pin<&mut QAbstractSocket>,
            address: &QHostAddress,
            port: u16,
            mode: QAbstractSocketBindMode,
        ) -> bool;

        #[rust_name = "connect_to_host"]
        fn connectToHost(
            self: Pin<&mut QAbstractSocket>,
            host_name: &QString,
            port: u16,
            open_mode: QIODeviceOpenMode,
            protocol: QAbstractSocketNetworkLayerProtocol,
        );

        /// Attempts to close the socket. If there is pending data waiting to be written, `QAbstractSocket` will enter `ClosingState` and wait until all data has been written. Eventually, it will enter `UnconnectedState` and emit the `disconnected()` signal.
        #[rust_name = "disconnect_from_host"]
        fn disconnectFromHost(self: Pin<&mut QAbstractSocket>);

        /// Returns the type of error that last occurred.
        fn error(self: &QAbstractSocket) -> QAbstractSocketSocketError;

        /// This function writes as much as possible from the internal write buffer to the underlying network socket, without blocking. If any data was written, this function returns `true`; otherwise `false` is returned.
        ///
        /// Call this function if you need `QAbstractSocket` to start sending buffered data immediately. The number of bytes successfully written depends on the operating system. In most cases, you do not need to call this function, because `QAbstractSocket` will start sending data automatically once control goes back to the event loop. In the absence of an event loop, call `wait_for_bytes_written()` instead.
        fn flush(self: Pin<&mut QAbstractSocket>) -> bool;

        /// Returns true if the socket is valid and ready for use; otherwise returns false.
        ///
        /// **Note:** The socket's state must be `ConnectedState` before reading and writing can occur.
        #[rust_name = "is_valid"]
        fn isValid(self: &QAbstractSocket) -> bool;

        #[rust_name = "local_address_or_null"]
        pub(self) fn localAddress(self: &QAbstractSocket) -> QHostAddress;

        /// Returns the host port number (in native byte order) of the local socket if available; otherwise returns 0.
        #[rust_name = "local_port"]
        fn localPort(self: &QAbstractSocket) -> u16;

        /// Returns the pause mode of this socket.
        #[rust_name = "pause_mode"]
        fn pauseMode(self: &QAbstractSocket) -> QAbstractSocketPauseModes;

        #[rust_name = "peer_address_or_null"]
        pub(self) fn peerAddress(self: &QAbstractSocket) -> QHostAddress;

        #[rust_name = "peer_name_or_empty"]
        pub(self) fn peerName(self: &QAbstractSocket) -> QString;

        /// Returns the port of the econnected peer if the socket is in `ConnectedState`; otherwise returns 0.
        #[rust_name = "peer_port"]
        fn peerPort(self: &QAbstractSocket) -> u16;

        /// Returns the protocol tag for this socket. If the protocol tag is set then this is passed to `QNetworkProxyQuery` when this is created internally to indicate the protocol tag to be used.
        #[rust_name = "protocol_tag"]
        fn protocolTag(self: &QAbstractSocket) -> QString;

        /// Returns the network proxy for this socket. By default `QNetworkProxy::DefaultProxy` is used, which means this socket will query the default proxy settings for the application.
        fn proxy(self: &QAbstractSocket) -> QNetworkProxy;

        /// Returns the size of the internal read buffer. This limits the amount of data that the client can receive before you call `read()` or `read_all()`.
        ///
        /// A read buffer size of 0 (the default) means that the buffer has no size limit, ensuring that no data is lost.
        #[rust_name = "read_buffer_size"]
        fn readBufferSize(self: &QAbstractSocket) -> i64;

        /// Continues data transfer on the socket. This method should only be used after the socket has been set to pause upon notifications and a notification has been received. The only notification currently supported is `QSslSocket::ssl_errors()`.
        ///
        /// # Safety
        ///
        /// Calling this method if the socket is not paused results in undefined behavior.
        unsafe fn resume(self: Pin<&mut QAbstractSocket>);

        /// Controls whether to pause upon receiving a notification. The pauseMode parameter specifies the conditions in which the socket should be paused. The only notification currently supported is `QSslSocket::ssl_errors()`. If set to `PauseOnSslErrors`, data transfer on the socket will be paused and needs to be enabled explicitly again by calling `resume()`. By default this option is set to `PauseNever`.
        ///
        /// # Safety
        ///
        /// This option must be called before connecting to the server, otherwise it will result in undefined behavior.
        #[rust_name = "set_pause_mode"]
        unsafe fn setPauseMode(
            self: Pin<&mut QAbstractSocket>,
            pause_mode: QAbstractSocketPauseModes,
        );

        /// Sets the protocol tag for this socket to `tag`.
        #[rust_name = "set_protocol_tag"]
        fn setProtocolTag(self: Pin<&mut QAbstractSocket>, tag: &QString);

        /// Sets the explicit network proxy for this socket to networkProxy.
        ///
        /// To disable the use of a proxy for this socket, use the `QNetworkProxy::NoProxy` proxy type.
        #[rust_name = "set_proxy"]
        fn setProxy(self: Pin<&mut QAbstractSocket>, proxy: &QNetworkProxy);

        /// Sets the size of `QAbstractSocket`'s internal read buffer to be size bytes.
        ///
        /// If the buffer size is limited to a certain size, `QAbstractSocket` won't buffer more than this size of data. Exceptionally, a buffer size of 0 means that the read buffer is unlimited and all incoming data is buffered. This is the default.
        ///
        /// This option is useful if you only read the data at certain points in time (e.g., in a real-time streaming application) or if you want to protect your socket against receiving too much data, which may eventually cause your application to run out of memory.
        ///
        /// Only `QTcpSocket` uses `QAbstractSocket`'s internal buffer; `QUdpSocket` does not use any buffering at all, but rather relies on the implicit buffering provided by the operating system. Because of this, calling this function on `QUdpSocket` has no effect.
        #[rust_name = "set_read_buffer_size"]
        fn setReadBufferSize(self: Pin<&mut QAbstractSocket>, size: i64);

        #[rust_name = "set_socket_option_variant"]
        pub(self) fn setSocketOption(
            self: Pin<&mut QAbstractSocket>,
            option: QAbstractSocketSocketOption,
            variant: &QVariant,
        );

        /// Returns the value of the `option` option.
        #[rust_name = "socket_option"]
        fn socketOption(
            self: Pin<&mut QAbstractSocket>,
            option: QAbstractSocketSocketOption,
        ) -> QVariant;

        /// Returns the socket type (TCP, UDP, or other).
        #[rust_name = "socket_type"]
        fn socketType(self: &QAbstractSocket) -> QAbstractSocketSocketType;

        /// Returns the state of the socket.
        fn state(self: &QAbstractSocket) -> QAbstractSocketSocketState;

        #[rust_name = "wait_for_connected_msecs"]
        pub(self) fn waitForConnected(self: Pin<&mut QAbstractSocket>, msecs: i32) -> bool;

        #[rust_name = "wait_for_disconnected_msecs"]
        fn waitForDisconnected(self: Pin<&mut QAbstractSocket>, msecs: i32) -> bool;

        /// This signal is emitted after `connect_to_host()` has been called and a connection has been successfully established.
        ///
        /// **Note:** On some operating systems the `connected()` signal may be directly emitted from the `connect_to_host()` call for connections to the localhost.
        #[qsignal]
        fn connected(self: Pin<&mut QAbstractSocket>);

        /// This signal is emitted when the socket has been disconnected.
        #[qsignal]
        fn disconnected(self: Pin<&mut QAbstractSocket>);

        /// This signal is emitted after an error occurred. The `socket_error` parameter describes the type of error that occurred.
        ///
        /// When this signal is emitted, the socket may not be ready for a reconnect attempt. In that case, attempts to reconnect should be done from the event loop. For example, use `QChronoTimer::single_shot()` with 0ns as the timeout.
        #[qsignal]
        #[rust_name = "error_occurred"]
        fn errorOccurred(self: Pin<&mut QAbstractSocket>, socket_error: QAbstractSocketSocketError);

        /// This signal is emitted after connectToHost() has been called and the host lookup has succeeded.
        ///
        /// **Note:** `QAbstractSocket` may emit `host_found()` directly from the `connect_to_host()` call since a DNS result could have been cached.
        #[qsignal]
        #[rust_name = "host_found"]
        fn hostFound(self: Pin<&mut QAbstractSocket>);

        /// This signal can be emitted when a `proxy` that requires authentication is used. The `authenticator` object can then be filled in with the required details to allow authentication and continue the connection.
        ///
        /// **Note:** It is not possible to use a `QueuedConnection` to connect to this signal, as the connection will fail if the authenticator has not been filled in with new information when the signal returns.
        ///
        /// # Safety
        ///
        /// `authenticator` must be valid.
        #[qsignal]
        #[rust_name = "proxy_authentication_required"]
        unsafe fn proxyAuthenticationRequired(
            self: Pin<&mut QAbstractSocket>,
            proxy: &QNetworkProxy,
            authenticator: *mut QAuthenticator,
        );

        /// This signal is emitted whenever `QAbstractSocket`'s state changes. The `socket_state` parameter is the new state.
        #[qsignal]
        #[rust_name = "state_changed"]
        fn stateChanged(self: Pin<&mut QAbstractSocket>, socket_state: QAbstractSocketSocketState);
    }
}

pub use ffi::{
    QAbstractSocket, QAbstractSocketBindFlag, QAbstractSocketNetworkLayerProtocol,
    QAbstractSocketPauseMode, QAbstractSocketSocketError, QAbstractSocketSocketOption,
    QAbstractSocketSocketState, QAbstractSocketSocketType,
};

pub type QAbstractSocketBindMode = QFlags<QAbstractSocketBindFlag>;
unsafe_impl_qflag!(QAbstractSocketBindFlag, "QAbstractSocketBindMode");

pub type QAbstractSocketPauseModes = QFlags<QAbstractSocketPauseMode>;
unsafe_impl_qflag!(QAbstractSocketPauseMode, "QAbstractSocketPauseModes");

impl QAbstractSocket {
    /// Returns the host address of the local socket if available; otherwise returns `None`.
    ///
    /// This is normally the main IP address of the host, but can be `QHostAddress::LocalHost` (127.0.0.1) for connections to the local host.
    pub fn local_address(&self) -> Option<QHostAddress> {
        self.local_address_or_null().nonnull()
    }

    /// Returns the address of the connected peer if the socket is in `ConnectedState`; otherwise returns `None`.
    pub fn peer_address(&self) -> Option<QHostAddress> {
        self.peer_address_or_null().nonnull()
    }

    /// Returns the name of the peer as specified by `connect_to_host()`, or `None` if `connect_to_host()` has not been called.
    pub fn peer_name(&self) -> Option<QString> {
        self.peer_name_or_empty().nonnull()
    }

    /// Sets the given `option` to the value described by `value`.
    pub fn set_socket_option<T>(
        self: Pin<&mut QAbstractSocket>,
        option: QAbstractSocketSocketOption,
        variant: T,
    ) where
        T: Into<QVariant>,
    {
        self.set_socket_option_variant(option, &variant.into());
    }

    /// Waits until the socket is connected, up to `duration`. If the connection has been established, this function returns `true`; otherwise it returns `false`. In the case where it returns `false`, you can call `error()` to determine the cause of the error.
    ///
    /// If `duration` is `None`, this function will not time out.
    ///
    /// **Note:** This function may wait slightly longer than `duration`, depending on the time it takes to complete the host lookup.
    ///
    /// **Note:** Multiple calls to this functions do not accumulate the time. If the function times out, the connecting process will be aborted.
    ///
    /// **Note:** This function may fail randomly on Windows. Consider using the event loop and the `connected()` signal if your software will run on Windows.
    pub fn wait_for_connected(self: Pin<&mut QAbstractSocket>, duration: Option<Duration>) -> bool {
        self.wait_for_connected_msecs(duration.msecs())
    }

    /// Waits until the socket has disconnected, up to `duration`. If the connection was successfully disconnected, this function returns `true`; otherwise it returns `false` (if the operation timed out, if an error occurred, or if this `QAbstractSocket` is already disconnected). In the case where it returns `false`, you can `call error()` to determine the cause of the error.
    ///
    /// If `duration` is `None`, this function will not time out.
    ///
    /// **Note:** This function may fail randomly on Windows. Consider using the event loop and the `disconnected()` signal if your software will run on Windows.
    pub fn wait_for_disconnected(
        self: Pin<&mut QAbstractSocket>,
        duration: Option<Duration>,
    ) -> bool {
        self.wait_for_disconnected_msecs(duration.msecs())
    }
}

impl Deref for QAbstractSocket {
    type Target = QIODevice;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl QIO for QAbstractSocket {
    fn flush(self: Pin<&mut Self>) -> bool {
        self.flush()
    }

    fn get_error_kind(&self) -> io::ErrorKind {
        self.error().into()
    }
}

impl Read for Pin<&mut QAbstractSocket> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QIOExt::read(self.as_mut(), buf)
    }
}

impl Write for Pin<&mut QAbstractSocket> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        QIOExt::write(self.as_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        QIOExt::flush(self.as_mut())
    }
}

impl From<QAbstractSocketSocketError> for io::ErrorKind {
    fn from(value: QAbstractSocketSocketError) -> Self {
        #[allow(clippy::match_same_arms)]
        match value {
            QAbstractSocketSocketError::ConnectionRefusedError => io::ErrorKind::ConnectionRefused,
            QAbstractSocketSocketError::RemoteHostClosedError => io::ErrorKind::ConnectionAborted,
            QAbstractSocketSocketError::HostNotFoundError => io::ErrorKind::NotFound,
            QAbstractSocketSocketError::SocketAccessError => io::ErrorKind::PermissionDenied,
            QAbstractSocketSocketError::SocketTimeoutError => io::ErrorKind::TimedOut,
            QAbstractSocketSocketError::DatagramTooLargeError => io::ErrorKind::InvalidData,
            QAbstractSocketSocketError::NetworkError => io::ErrorKind::BrokenPipe,
            QAbstractSocketSocketError::AddressInUseError => io::ErrorKind::AddrInUse,
            QAbstractSocketSocketError::SocketAddressNotAvailableError => {
                io::ErrorKind::AddrNotAvailable
            }
            QAbstractSocketSocketError::UnsupportedSocketOperationError => {
                io::ErrorKind::Unsupported
            }
            QAbstractSocketSocketError::UnfinishedSocketOperationError => {
                io::ErrorKind::ConnectionRefused
            }
            QAbstractSocketSocketError::ProxyAuthenticationRequiredError => {
                io::ErrorKind::ConnectionAborted
            }
            QAbstractSocketSocketError::SslHandshakeFailedError => io::ErrorKind::ConnectionAborted,
            QAbstractSocketSocketError::ProxyConnectionRefusedError => {
                io::ErrorKind::ConnectionRefused
            }
            QAbstractSocketSocketError::ProxyConnectionClosedError => {
                io::ErrorKind::ConnectionAborted
            }
            QAbstractSocketSocketError::ProxyConnectionTimeoutError => io::ErrorKind::TimedOut,
            QAbstractSocketSocketError::ProxyNotFoundError => io::ErrorKind::NotFound,
            QAbstractSocketSocketError::TemporaryError => io::ErrorKind::WouldBlock,
            _ => io::ErrorKind::Other,
        }
    }
}

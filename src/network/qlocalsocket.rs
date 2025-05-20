use crate::util::{IsNonNull, MSecs};
use crate::{
    QAbstractSocketSocketError, QAbstractSocketSocketState, QIODevice, QIODeviceOpenMode,
    SocketDescriptor,
};
use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt::QObject;
use cxx_qt_lib::{QFlags, QString};
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;
use std::time::Duration;

#[cxx_qt::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum QLocalSocketLocalSocketError {
        /// The connection was refused by the peer (or timed out).
        ConnectionRefusedError,
        /// The remote socket closed the connection. Note that the client socket (i.e., this socket) will be closed after the remote close notification has been sent.
        PeerClosedError,
        /// The local socket name was not found.
        ServerNotFoundError,
        //// The socket operation failed because the application lacked the required privileges.
        SocketAccessError,
        /// The local system ran out of resources (e.g., too many sockets).
        SocketResourceError,
        /// The socket operation timed out.
        SocketTimeoutError,
        /// The datagram was larger than the operating system's limit (which can be as low as 8192 bytes).
        DatagramTooLargeError,
        /// An error occurred with the connection.
        ConnectionError,
        /// The requested socket operation is not supported by the local operating system.
        UnsupportedSocketOperationError = 10,
        /// The requested socket operation is not supported by the local operating system.
        OperationError = 19,
        /// An unidentified error occurred.
        UnknownSocketError = -1,
    }

    #[repr(i32)]
    #[derive(Debug)]
    enum QLocalSocketLocalSocketState {
        /// The socket is not connected.
        UnconnectedState = 0,
        /// The socket has started establishing a connection.
        ConnectingState = 2,
        /// A connection is established.
        ConnectedState = 3,
        /// The socket is about to close (data may still be waiting to be written).
        ClosingState = 6,
    }

    #[repr(i32)]
    enum QLocalSocketSocketOption {
        /// No options have been set.
        NoOptions = 0x0,
        /// The socket will try to connect to an abstract address. This flag is specific to Linux and Android. On other platforms is ignored.
        AbstractNamespaceOption = 0x01,
    }

    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qtypes.h");
        type qintptr = cxx_qt_lib::qintptr;
        include!("cxx-qt-io/qiodevice.h");
        type QIODevice = crate::QIODevice;
        type QIODeviceOpenMode = crate::QIODeviceOpenMode;
    }

    extern "C++" {
        include!("cxx-qt-io/qlocalsocket.h");
        type QLocalSocketLocalSocketError;
        type QLocalSocketLocalSocketState;
        type QLocalSocketSocketOption;
        #[allow(unused)]
        type QLocalSocketSocketOptions = super::QLocalSocketSocketOptions;
    }

    unsafe extern "C++Qt" {
        /// The `QLocalSocket` class provides a local socket.
        ///
        /// Qt Documentation: [QLocalSocket](https://doc.qt.io/qt-6/qlocalsocket.html#details)
        #[qobject]
        #[base = QIODevice]
        type QLocalSocket;

        /// Aborts the current connection and resets the socket. Unlike [`disconnect_from_server`](QLocalSocket::disconnect_from_server), this function immediately closes the socket, clearing any pending data in the write buffer.
        fn abort(self: Pin<&mut QLocalSocket>);

        /// Attempts to make a connection to [`self.server_name()`](QLocalSocket::server_name). [`set_server_name`](QLocalSocket::set_server_name) must be called before you open the connection. Alternatively you can use [`connect_to_server`](QLocalSocket::connect_to_server);
        ///
        /// The socket is opened in the given `open_mode` and first enters [`QLocalSocketLocalSocketState::ConnectingState`]. If a connection is established, `QLocalSocket` enters [`QLocalSocketLocalSocketState::ConnectedState`] and emits [`connected`](QLocalSocket::connected).
        ///
        /// After calling this function, the socket can emit [`error_occurred`](QLocalSocket::error_occurred) to signal that an error occurred.
        #[rust_name = "connect_to_current_server"]
        fn connectToServer(self: Pin<&mut QLocalSocket>, open_mode: QIODeviceOpenMode);

        /// Set the server name and attempts to make a connection to it.
        ///
        /// The socket is opened in the given `open_mode` and first enters [`QLocalSocketLocalSocketState::ConnectingState`]. If a connection is established, `QLocalSocket` enters [`QLocalSocketLocalSocketState::ConnectedState`] and emits [`connected`](QLocalSocket::connected).
        ///
        /// After calling this function, the socket can emit [`error_occurred`](QLocalSocket::error_occurred) to signal that an error occurred.
        #[rust_name = "connect_to_server"]
        fn connectToServer(
            self: Pin<&mut QLocalSocket>,
            name: &QString,
            open_mode: QIODeviceOpenMode,
        );

        /// Attempts to close the socket. If there is pending data waiting to be written, `QLocalSocket` will enter [`QLocalSocketLocalSocketState::ClosingState`] and wait until all data has been written. Eventually, it will enter [`QLocalSocketLocalSocketState::UnconnectedState`] and emit the [`disconnected`](QLocalSocket::disconnected) signal.
        #[rust_name = "disconnect_from_server"]
        fn disconnectFromServer(self: Pin<&mut QLocalSocket>);

        /// Returns the type of error that last occurred.
        fn error(self: &QLocalSocket) -> QLocalSocketLocalSocketError;

        /// This function writes as much as possible from the internal write buffer to the socket, without blocking. If any data was written, this function returns `true`; otherwise `false` is returned.
        ///
        /// Call this function if you need `QLocalSocket` to start sending buffered data immediately. The number of bytes successfully written depends on the operating system. In most cases, you do not need to call this function, because `QLocalSocket` will start sending data automatically once control goes back to the event loop. In the absence of an event loop, call [`wait_for_bytes_written`](QLocalSocket::wait_for_bytes_written) instead.
        fn flush(self: Pin<&mut QLocalSocket>) -> bool;

        /// Returns the server path that the socket is connected to.
        ///
        /// **Note:** The return value of this function is platform specific.
        #[rust_name = "full_server_name"]
        fn fullServerName(self: &QLocalSocket) -> QString;

        /// Returns `true` if the socket is valid and ready for use; otherwise returns `false`.
        ///
        /// **Note:** The socket's state must be [`QLocalSocketLocalSocketState::ConnectedState`] before reading and writing can occur.
        #[rust_name = "is_valid"]
        fn isValid(self: &QLocalSocket) -> bool;

        /// Returns the size of the internal read buffer. This limits the amount of data that the client can receive before you call [`read`](QIODevice::read) or [`read_all`](QIODevice::read_all). A read buffer size of 0 (the default) means that the buffer has no size limit, ensuring that no data is lost.
        #[rust_name = "read_buffer_size"]
        fn readBufferSize(self: &QLocalSocket) -> i64;

        #[doc(hidden)]
        #[rust_name = "server_name_or_empty"]
        fn serverName(self: &QLocalSocket) -> QString;

        /// Sets the size of `QLocalSocket`'s internal read buffer to be `size` bytes.
        ///
        /// If the buffer size is limited to a certain size, `QLocalSocket` won't buffer more than this size of data. Exceptionally, a buffer size of 0 means that the read buffer is unlimited and all incoming data is buffered. This is the default.
        ///
        /// This option is useful if you only read the data at certain points in time (e.g., in a real-time streaming application) or if you want to protect your socket against receiving too much data, which may eventually cause your application to run out of memory.
        #[rust_name = "set_read_buffer_size"]
        fn setReadBufferSize(self: Pin<&mut QLocalSocket>, size: i64);

        /// Set the `name` of the peer to connect to. On Windows `name` is the name of a named pipe; on Unix `name` is the name of a local domain socket.
        ///
        /// This function must be called when the socket is not connected.
        #[rust_name = "set_server_name"]
        fn setServerName(self: Pin<&mut QLocalSocket>, name: &QString);

        #[doc(hidden)]
        #[rust_name = "set_socket_descriptor_qintptr"]
        fn setSocketDescriptor(
            self: Pin<&mut QLocalSocket>,
            socket_descriptor: qintptr,
            socket_state: QLocalSocketLocalSocketState,
            open_mode: QIODeviceOpenMode,
        ) -> bool;

        #[doc(hidden)]
        #[rust_name = "socket_descriptor_or_negative"]
        fn socketDescriptor(self: &QLocalSocket) -> qintptr;

        /// Returns the state of the socket.
        fn state(self: &QLocalSocket) -> QLocalSocketLocalSocketState;

        #[doc(hidden)]
        #[rust_name = "wait_for_connected_msecs"]
        fn waitForConnected(self: Pin<&mut QLocalSocket>, msecs: i32) -> bool;

        #[rust_name = "wait_for_disconnected_msecs"]
        fn waitForDisconnected(self: Pin<&mut QLocalSocket>, msecs: i32) -> bool;

        /// This signal is emitted after [`connect_to_server`](QLocalSocket::connect_to_server) has been called and a connection has been successfully established.
        #[qsignal]
        fn connected(self: Pin<&mut QLocalSocket>);

        /// This signal is emitted when the socket has been disconnected.
        #[qsignal]
        fn disconnected(self: Pin<&mut QLocalSocket>);

        /// This signal is emitted after an error occurred. The `socket_error` parameter describes the type of error that occurred.
        #[qsignal]
        #[rust_name = "error_occurred"]
        fn errorOccurred(self: Pin<&mut QLocalSocket>, socket_error: QLocalSocketLocalSocketError);

        /// This signal is emitted whenever `QLocalSocket`'s state changes. The `socket_state` parameter is the new state.
        #[qsignal]
        #[rust_name = "state_changed"]
        fn stateChanged(self: Pin<&mut QLocalSocket>, socket_state: QLocalSocketLocalSocketState);
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        include!("cxx-qt-io/common.h");

        #[rust_name = "upcast_qlocalsocket_qobject"]
        unsafe fn upcast(socket: *const QLocalSocket) -> *const QObject;
        #[rust_name = "downcast_qobject_qlocalsocket"]
        unsafe fn downcast(socket: *const QObject) -> *const QLocalSocket;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlocalsocket_new"]
        fn make_unique() -> UniquePtr<QLocalSocket>;
    }
}

pub use ffi::{
    QLocalSocket, QLocalSocketLocalSocketError, QLocalSocketLocalSocketState,
    QLocalSocketSocketOption,
};

/// [`QFlags`] of [`QLocalSocketSocketOption`].
pub type QLocalSocketSocketOptions = QFlags<QLocalSocketSocketOption>;

unsafe_impl_qflag!(QLocalSocketSocketOption, "QLocalSocketSocketOptions");

impl QLocalSocket {
    /// Constructs a new local socket.
    pub fn new() -> UniquePtr<Self> {
        ffi::qlocalsocket_new()
    }

    /// Returns the name of the peer as specified by [`set_server_name`](QLocalSocket::set_server_name), or `None` if [`set_server_name`](QLocalSocket::set_server_name) has not been called or [`connect_to_server`](QLocalSocket::connect_to_server) failed.
    pub fn server_name(self: &QLocalSocket) -> Option<QString> {
        self.server_name_or_empty().nonnull()
    }

    /// Initializes `QLocalSocket` with the native socket descriptor `socket_descriptor`. Returns `true` if `socket_descriptor` is accepted as a valid socket descriptor; otherwise returns `false`. The socket is opened in the mode specified by `open_mode`, and enters the socket state specified by `socket_state`. Read and write buffers are cleared, discarding any pending data.
    ///
    /// **Note:** It is not possible to initialize two local sockets with the same native socket descriptor.
    pub fn set_socket_descriptor(
        self: Pin<&mut Self>,
        socket_descriptor: SocketDescriptor,
        socket_state: QLocalSocketLocalSocketState,
        open_mode: QIODeviceOpenMode,
    ) -> bool {
        self.set_socket_descriptor_qintptr(socket_descriptor.into(), socket_state, open_mode)
    }

    /// Returns the native socket descriptor of the `QLocalSocket` object if this is available; otherwise returns `None`.
    ///
    /// The socket descriptor is not available when `QLocalSocket` is in [`QLocalSocketLocalSocketState::UnconnectedState`].
    pub fn socket_descriptor(&self) -> Option<SocketDescriptor> {
        SocketDescriptor::from(self.socket_descriptor_or_negative()).nonnull()
    }

    /// Waits until the socket is connected, up to `duration`. If the connection has been established, this function returns `true`; otherwise it returns `false`. In the case where it returns `false`, you can call [`error`](QLocalSocket::error) to determine the cause of the error.
    ///
    /// If `duration` is `None`, this function will not time out.
    pub fn wait_for_connected(self: Pin<&mut Self>, duration: Option<Duration>) -> bool {
        self.wait_for_connected_msecs(duration.msecs())
    }

    /// Waits until the socket has disconnected, up to `duration`. If the connection was successfully disconnected, this function returns `true`; otherwise it returns `false` (if the operation timed out, if an error occurred, or if this `QLocalSocket` is already disconnected). In the case where it returns `false`, you can call [`error`](QLocalSocket::error) to determine the cause of the error.
    ///
    /// If `duration` is `None`, this function will not time out.
    pub fn wait_for_disconnected(self: Pin<&mut QLocalSocket>, duration: Option<Duration>) -> bool {
        self.wait_for_disconnected_msecs(duration.msecs())
    }

    /// Casts this object to `QIODevice`.
    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    /// Mutably casts this object to `QIODevice`.
    pub fn as_io_device_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QIODevice> {
        self.as_mut().upcast_pin()
    }
}

impl Deref for QLocalSocket {
    type Target = QIODevice;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl AsRef<QIODevice> for QLocalSocket {
    fn as_ref(&self) -> &QIODevice {
        self.upcast()
    }
}

unsafe impl Upcast<QObject> for QLocalSocket {
    unsafe fn upcast_ptr(this: *const Self) -> *const QObject {
        ffi::upcast_qlocalsocket_qobject(this)
    }

    unsafe fn from_base_ptr(base: *const QObject) -> *const Self {
        ffi::downcast_qobject_qlocalsocket(base)
    }
}

impl AsRef<QObject> for QLocalSocket {
    fn as_ref(&self) -> &QObject {
        self.upcast()
    }
}

impl Read for Pin<&mut QLocalSocket> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_io_device_mut().read(buf)
    }
}

impl Write for Pin<&mut QLocalSocket> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_io_device_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.as_mut().flush();
        Ok(())
    }
}

impl From<QLocalSocketLocalSocketError> for QAbstractSocketSocketError {
    fn from(value: QLocalSocketLocalSocketError) -> Self {
        Self { repr: value.repr }
    }
}

impl From<QLocalSocketLocalSocketError> for io::ErrorKind {
    fn from(value: QLocalSocketLocalSocketError) -> Self {
        QAbstractSocketSocketError::from(value).into()
    }
}

impl From<QLocalSocketLocalSocketState> for QAbstractSocketSocketState {
    fn from(value: QLocalSocketLocalSocketState) -> Self {
        Self { repr: value.repr }
    }
}

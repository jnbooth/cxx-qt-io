use crate::util::MSecs;
use crate::QTcpServer;
use cxx_qt::Upcast;
use std::pin::Pin;
use std::time::Duration;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qsslconfiguration.h");
        type QSslConfiguration = crate::QSslConfiguration;
        include!("cxx-qt-io/qsslerror.h");
        type QSslError = crate::QSslError;
        include!("cxx-qt-io/qsslpresharedkeyauthenticator.h");
        type QSslPreSharedKeyAuthenticator = crate::QSslPreSharedKeyAuthenticator;
        include!("cxx-qt-io/qlist.h");
        type QList_QSslError = cxx_qt_lib::QList<QSslError>;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslserver.h");
        type QSslSocket = crate::QSslSocket;
        type QTcpServer = crate::QTcpServer;
    }

    unsafe extern "C++Qt" {
        /// Implements an encrypted, secure TCP server over TLS.
        ///
        /// Qt Documentation: [QSslServer](https://doc.qt.io/qt-6/qsslserver.html#details)
        #[qobject]
        #[base = QTcpServer]
        type QSslServer;

        /// Returns the currently configured handshake timeout.
        #[rust_name = "handshake_timeout"]
        fn handshakeTimeout(self: &QSslServer) -> i32;

        #[rust_name = "set_handshake_timeout_msecs"]
        pub(self) fn setHandshakeTimeout(self: Pin<&mut QSslServer>, timeout: i32);

        /// Sets the `ssl_configuration` to use for all following incoming connections.
        ///
        /// This must be called before [`listen`](QTcpServer::listen) to ensure that the desired configuration was in use during all handshakes.
        #[rust_name = "set_ssl_configuration"]
        pub fn setSslConfiguration(
            self: Pin<&mut QSslServer>,
            ssl_configuration: &QSslConfiguration,
        );

        /// Returns the current ssl configuration.
        #[rust_name = "ssl_configuration"]
        pub fn sslConfiguration(self: &QSslServer) -> QSslConfiguration;

        /// `QSslServer` can emit this signal several times during the SSL handshake, before encryption has been established, to indicate that an error has occurred while establishing the identity of the peer. The error is usually an indication that socket is unable to securely identify the peer.
        ///
        /// This signal provides you with an early indication when something's wrong. By connecting to this signal, you can manually choose to tear down the connection from inside the connected slot before the handshake has completed. If no action is taken, `QSslServer` will proceed to emitting [`ssl_errors`](QSslServer::ssl_errors).
        #[qsignal]
        #[rust_name = "peer_verify_error"]
        unsafe fn peerVerifyError(
            self: Pin<&mut QSslServer>,
            socket: *mut QSslSocket,
            error: &QSslError,
        );

        /// `QSslServer` emits this signal when it negotiates a PSK ciphersuite, and therefore a PSK authentication is then required.
        ///
        /// When using PSK, the client must send to the server a valid identity and a valid pre shared key, in order for the SSL handshake to continue. Applications can provide this information in a slot connected to this signal, by filling in the passed `authenticator` object according to their needs.
        ///
        /// **Note:** Ignoring this signal, or failing to provide the required credentials, will cause the handshake to fail, and therefore the connection to be aborted.
        ///
        /// **Note:** The `authenticator` object is owned by the `socket` and must not be deleted by the application.
        #[qsignal]
        #[rust_name = "pre_shared_key_authentication_required"]
        unsafe fn preSharedKeyAuthenticationRequired(
            self: Pin<&mut QSslServer>,
            socket: *mut QSslSocket,
            authenticator: *mut QSslPreSharedKeyAuthenticator,
        );

        /// `QSslServer` emits this signal after the SSL handshake to indicate that one or more errors have occurred while establishing the identity of the peer. The errors are usually an indication that `socket` is unable to securely identify the peer. Unless any action is taken, the connection will be dropped after this signal has been emitted.
        ///
        /// If you want to continue connecting despite the errors that have occurred, you must call [`QSslSocket::ignore_ssl_errors`] from inside a slot connected to this signal. If you need to access the error list at a later point, you can call [`QSslSocket::ssl_handshake_errors`].
        ///
        /// `errors` contains one or more errors that prevent `QSslSocket` from verifying the identity of the peer.
        ///
        /// **Note:** You cannot use `QueuedConnection` when connecting to this signal, or calling [`ignore_ssl_errors`](QSslSocket::ignore_ssl_errors) will have no effect.
        #[qsignal]
        #[rust_name = "ssl_errors"]
        unsafe fn sslErrors(
            self: Pin<&mut QSslServer>,
            socket: *mut QSslSocket,
            errors: &QList_QSslError,
        );

        /// This signal is emitted when the client, connected to `socket`, initiates the TLS handshake.
        #[qsignal]
        #[rust_name = "started_encryption_handshake"]
        unsafe fn startedEncryptionHandshake(self: Pin<&mut QSslServer>, socket: *mut QSslSocket);
    }
}

pub use ffi::QSslServer;

impl QSslServer {
    /// Sets the `timeout` to use for all incoming handshakes.
    ///
    /// This is relevant in the scenario where a client, whether malicious or accidental, connects to the server but makes no attempt at communicating or initiating a handshake. `QSslServer` will then automatically end the connection after `timeout` has elapsed.
    ///
    /// By default the timeout is 5 seconds.
    ///
    /// **Note:** The underlying TLS framework may have their own timeout logic now or in the future, this function does not affect that.
    ///
    /// **Note:** The timeout passed to this function will only apply to new connections. If a client is already connected it will use the timeout which was set when it connected.
    pub fn set_handshake_timeout(self: Pin<&mut Self>, timeout: Duration) {
        self.set_handshake_timeout_msecs(timeout.msecs());
    }

    /// Casts this object to `QTcpServer`.
    pub fn as_tcp_server(&self) -> &QTcpServer {
        self.upcast()
    }

    /// Mutably casts this object to `QTcpServer`.
    pub fn as_file_device_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QTcpServer> {
        self.as_mut().upcast_pin()
    }
}

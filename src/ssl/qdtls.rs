use std::fmt;
use std::ops::Deref;
use std::pin::Pin;

use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt::QObject;
use cxx_qt_lib::QByteArray;

use crate::qobject::debug_qobject;
use crate::util::{unpin_for_qt, IsNonNull};
use crate::{QHostAddress, QSslCipher, QSslSocketSslMode, QUdpSocket};

#[cxx_qt::bridge]
mod ffi {
    /// This enum describes general and TLS-specific errors that can be encountered by objects of the classes [`QDtlsClientVerifier`](crate::QDtlsClientVerifier) and [`QDtls`].
    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq)]
    enum QDtlsError {
        /// No error occurred, the last operation was successful.
        NoError,
        /// Input parameters provided by a caller were invalid.
        InvalidInputParameters,
        /// An operation was attempted in a state that did not permit it.
        InvalidOperation,
        /// [`QUdpSocket::write_datagram`](crate::QUdpSocket::write_datagram) failed, [`QUdpSocket::error`](crate::QAbstractSocket::error) and [`QUdpSocket::error_string`](crate::QIODevice::error_string) can provide more specific information.
        UnderlyingSocketError,
        /// TLS shutdown alert message was received.
        RemoteClosedConnectionError,
        /// Peer's identity could not be verified during the TLS handshake.
        PeerVerificationError,
        /// An error occurred while initializing an underlying TLS backend.
        TlsInitializationError,
        /// A fatal error occurred during TLS handshake, other than peer verification error or TLS initialization error.
        TlsFatalError,
        /// A failure to encrypt or decrypt a datagram, non-fatal, meaning [`QDtls`] can continue working after this error.
        TlsNonFatalError,
    }

    /// This enum describes the current state of DTLS handshake for a [`QDtls`] connection.
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum QDtlsHandshakeState {
        /// Nothing done yet.
        HandshakeNotStarted,
        /// Handshake was initiated and no errors were found so far.
        HandshakeInProgress,
        /// The identity of the peer can't be established.
        PeerVerificationFailed,
        /// Handshake completed successfully and encrypted connection was established.
        HandshakeComplete,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qtypes.h");
        type qint64 = cxx_qt_lib::qint64;

        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddress = crate::QHostAddress;
        include!("cxx-qt-io/qssl.h");
        type QSslSslProtocol = crate::QSslSslProtocol;
        include!("cxx-qt-io/qsslcipher.h");
        type QSslCipher = crate::QSslCipher;
        include!("cxx-qt-io/qsslconfiguration.h");
        type QSslConfiguration = crate::QSslConfiguration;
        include!("cxx-qt-io/qsslpresharedkeyauthenticator.h");
        type QSslPreSharedKeyAuthenticator = crate::QSslPreSharedKeyAuthenticator;
        include!("cxx-qt-io/qsslsocket.h");
        type QSslSocketSslMode = crate::QSslSocketSslMode;
        include!("cxx-qt-io/qudpsocket.h");
        type QUdpSocket = crate::QUdpSocket;
        include!("cxx-qt-io/qlist.h");
        type QList_QSslError = cxx_qt_lib::QList<crate::QSslError>;
    }

    extern "C++" {
        include!("cxx-qt-io/qdtls.h");
        type QDtlsError;
        type QDtlsHandshakeState;

        type QDtlsGeneratorParameters = crate::QDtlsGeneratorParameters;
    }

    unsafe extern "C++Qt" {
        /// This class provides encryption for UDP sockets.
        ///
        /// Qt Documentation: [QDtls](https://doc.qt.io/qt-6/qdtls.html#details)
        #[qobject]
        #[base = QObject]
        type QDtls;

        /// # Safety
        ///
        /// `socket` must be valid.
        #[doc(hidden)]
        #[rust_name = "abort_handshake_raw"]
        unsafe fn abortHandshake(self: Pin<&mut QDtls>, socket: *mut QUdpSocket) -> bool;

        /// Returns the current hash algorithm and secret, either default ones or previously set by a call to [`set_cookie_generator_parameters`](QDtls::set_cookie_generator_parameters).
        #[rust_name = "cookie_generator_parameters"]
        fn cookieGeneratorParameters(self: &QDtls) -> QDtlsGeneratorParameters;

        /// # Safety
        ///
        /// `socket` must be valid.
        #[doc(hidden)]
        #[rust_name = "decrypt_datagram_raw"]
        unsafe fn decryptDatagram(
            self: Pin<&mut QDtls>,
            socket: *mut QUdpSocket,
            dgram: &QByteArray,
        ) -> QByteArray;

        /// # Safety
        ///
        /// `socket` must be valid.
        #[doc(hidden)]
        #[rust_name = "do_handshake_raw"]
        unsafe fn doHandshake(
            self: Pin<&mut QDtls>,
            socket: *mut QUdpSocket,
            dgram: &QByteArray,
        ) -> bool;

        /// Returns either the default DTLS configuration or the configuration set by an earlier call to [`set_dtls_configuration`](QDtls::set_dtls_configuration).
        #[rust_name = "dtls_configuration"]
        fn dtlsConfiguration(self: &QDtls) -> QSslConfiguration;

        /// Returns the last error encountered by the connection or [`QDtlsError::NoError`].
        #[rust_name = "dtls_error"]
        fn dtlsError(self: &QDtls) -> QDtlsError;

        /// Returns a textual description for the last error encountered by the connection or empty string.
        #[rust_name = "dtls_error_string"]
        fn dtlsErrorString(self: &QDtls) -> QString;

        /// # Safety
        ///
        /// `socket` must be valid.
        #[doc(hidden)]
        #[rust_name = "handle_timeout_raw"]
        unsafe fn handleTimeout(self: Pin<&mut QDtls>, socket: *mut QUdpSocket) -> bool;

        /// Returns the current handshake state for this `QDtls`.
        #[rust_name = "handshake_state"]
        fn handshakeState(self: &QDtls) -> QDtlsHandshakeState;

        /// This method tells `QDtls` to ignore only the errors given in `errors_to_ignore`.
        ///
        /// You can also call this function after [`do_handshake`](QDtls::do_handshake) encountered the [`QDtlsError::PeerVerificationError`] error, and then resume the handshake by calling [`resume_handshake`](QDtls::resume_handshake).
        ///
        /// Later calls to this function will replace the list of errors that were passed in previous calls. You can clear the list of errors you want to ignore by calling this function with an empty list.
        #[rust_name = "ignore_verification_errors"]
        fn ignoreVerificationErrors(self: Pin<&mut QDtls>, errors_to_ignore: &QList_QSslError);

        /// Returns `true` if DTLS handshake completed successfully.
        #[rust_name = "is_connection_encrypted"]
        fn isConnectionEncrypted(self: &QDtls) -> bool;

        /// Returns the value previously set by [`set_mtu_hint`](QDtls::set_mtu_hint). The default value is 0.
        #[rust_name = "mtu_hint"]
        fn mtuHint(self: &QDtls) -> u16;

        #[doc(hidden)]
        #[rust_name = "peer_address_or_null"]
        fn peerAddress(self: &QDtls) -> QHostAddress;

        /// Returns the peer's port number, set by [`set_peer`](QDtls::set_peer), or 0.
        #[rust_name = "peer_port"]
        fn peerPort(self: &QDtls) -> u16;

        /// Returns errors found while establishing the identity of the peer.
        ///
        /// If you want to continue connecting despite the errors that have occurred, you must call [`ignore_verification_errors`](QDtls::ignore_verification_errors).
        #[rust_name = "peer_verification_errors"]
        fn peerVerificationErrors(self: &QDtls) -> QList_QSslError;

        /// Returns the host name set by [`set_peer`](QDtls::set_peer) or [`set_peer_verification_name`](QDtls::set_peer_verification_name). The default value is an empty string.
        #[rust_name = "peer_verification_name"]
        fn peerVerificationName(self: &QDtls) -> QString;

        /// # Safety
        ///
        /// `socket` must be valid.
        #[doc(hidden)]
        #[rust_name = "resume_handshake_raw"]
        unsafe fn resumeHandshake(self: Pin<&mut QDtls>, socket: *mut QUdpSocket) -> bool;

        #[doc(hidden)]
        #[rust_name = "session_cipher_or_null"]
        fn sessionCipher(self: &QDtls) -> QSslCipher;

        /// Returns the DTLS protocol version used by this connection, or [`QSslSslProtocol::UnknownProtocol`](crate::QSslSslProtocol::UnknownProtocol) if the connection isn't encrypted yet. The protocol for the connection is selected during the handshake phase.
        ///
        /// [`set_dtls_configuration`](QDtls::set_dtls_configuration) can set the preferred version before the handshake starts.
        #[rust_name = "session_protocol"]
        fn sessionProtocol(self: &QDtls) -> QSslSslProtocol;

        /// Sets the cryptographic hash algorithm and the secret from `params`. This function is only needed for a server-side QDtls connection. Returns `true` if successful.
        ///
        /// **Note:** This function must be called before the handshake starts.
        #[rust_name = "set_cookie_generator_parameters"]
        fn setCookieGeneratorParameters(
            self: Pin<&mut QDtls>,
            params: &QDtlsGeneratorParameters,
        ) -> bool;

        /// Sets the connection's TLS configuration from `configuration` and returns `true` if successful.
        ///
        /// **Note:** This function must be called before the handshake starts.
        #[rust_name = "set_dtls_configuration"]
        fn setDtlsConfiguration(self: Pin<&mut QDtls>, configuration: &QSslConfiguration) -> bool;

        /// `mtu_hint` is the maximum transmission unit (MTU), either discovered or guessed by the application. The application is not required to set this value.
        #[rust_name = "set_mtu_hint"]
        fn setMtuHint(self: Pin<&mut QDtls>, mtu_hint: u16);

        /// Sets the peer's `address`, `port`, and host name and returns `true` if successful. `address` must not be null, multicast, or broadcast. `verification_name` is the host name used for the certificate validation.
        #[rust_name = "set_peer"]
        fn setPeer(
            self: Pin<&mut QDtls>,
            address: &QHostAddress,
            port: u16,
            verification_name: &QString,
        ) -> bool;

        /// Sets the host `name` that will be used for the certificate validation and returns `true` if successful.
        ///
        /// **Note:** This function must be called before the handshake starts.
        #[rust_name = "set_peer_verification_name"]
        fn setPeerVerificationName(self: Pin<&mut QDtls>, name: &QString) -> bool;

        /// # Safety
        ///
        /// `socket` must be valid.
        #[doc(hidden)]
        #[rust_name = "shutdown_raw"]
        unsafe fn shutdown(self: Pin<&mut QDtls>, socket: *mut QUdpSocket) -> bool;

        /// Returns [`QSslSocketSslMode::SslServerMode`](crate::QSslSocketSslMode::SslServerMode) for a server-side connection and [`QSslSocketSslMode::SslClientMode`](crate::QSslSocketSslMode::SslClientMode) for a client.
        #[rust_name = "ssl_mode"]
        fn sslMode(self: &QDtls) -> QSslSocketSslMode;

        /// # Safety
        ///
        /// `socket` must be valid.
        #[doc(hidden)]
        #[rust_name = "write_datagram_encrypted_raw"]
        unsafe fn writeDatagramEncrypted(
            self: Pin<&mut QDtls>,
            socket: *mut QUdpSocket,
            dgram: &QByteArray,
        ) -> qint64;

        /// Packet loss can result in timeouts during the handshake phase. In this case `QDtls` emits this signal. Call [`handle_timeout`](QDtls::handle_timeout) to retransmit the handshake messages.
        #[qsignal]
        #[rust_name = "handshake_timeout"]
        fn handshakeTimeout(self: Pin<&mut QDtls>);

        /// This signal is emitted if the SSL/TLS handshake negotiates a PSK ciphersuite, and therefore a PSK authentication is then required.
        ///
        /// When using PSK, the client must send to the server a valid identity and a valid pre shared key, in order for the SSL handshake to continue. Applications can provide this information in a slot connected to this signal, by filling in the passed `authenticator` object according to their needs.
        ///
        /// **Note:** Ignoring this signal, or failing to provide the required credentials, will cause the handshake to fail, and therefore the connection to be aborted.
        ///
        /// **Note:** The `authenticator` object is owned by `QDtls` and must not be deleted by the application.
        #[qsignal]
        #[rust_name = "psk_required"]
        unsafe fn pskRequired(
            self: Pin<&mut QDtls>,
            authenticator: *mut QSslPreSharedKeyAuthenticator,
        );
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qdtls_init_ssl_mode"]
        fn make_unique(mode: QSslSocketSslMode) -> UniquePtr<QDtls>;
    }
}

pub use ffi::{QDtls, QDtlsError, QDtlsHandshakeState};

impl fmt::Debug for QDtls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QDtls {
    /// Creates a [`QDtls`] object depending on the `mode`:
    ///
    /// * [`QSslSocketSslMode::SslServerMode`]: Handles a server-side DTLS connection.
    /// * [`QSslSocketSslMode::SslClientMode`]: Handles a client-side DTLS connection.
    /// * [`QSslSocketSslMode::UnencryptedMode`]: Encryption will fail. The function will not panic, but there is no reason to do this.
    pub fn new(mode: QSslSocketSslMode) -> UniquePtr<Self> {
        ffi::qdtls_init_ssl_mode(mode)
    }

    /// Aborts the ongoing handshake. Returns `true` if one was on-going on `socket`; otherwise, sets a suitable error and returns `false`.
    pub fn abort_handshake(self: Pin<&mut Self>, socket: Pin<&mut QUdpSocket>) -> bool {
        // SAFETY: `unpin_for_qt(socket)` is passed directly to Qt.
        unsafe { self.abort_handshake_raw(unpin_for_qt(socket)) }
    }

    /// Decrypts `dgram` and returns its contents as plain text. The handshake must be completed before datagrams can be decrypted. Depending on the type of the TLS message the connection may write into `socket`.
    pub fn decrypt_datagram(
        self: Pin<&mut Self>,
        socket: Pin<&mut QUdpSocket>,
        dgram: &QByteArray,
    ) -> QByteArray {
        // SAFETY: `unpin_for_qt(socket)` is passed directly to Qt.
        unsafe { self.decrypt_datagram_raw(unpin_for_qt(socket), dgram) }
    }

    /// Starts or continues a DTLS handshake. When starting a server-side DTLS handshake, `dgram` must contain the initial `ClientHello` message read from [`QUdpSocket`]. This function returns `true` if no error was found. Handshake state can be tested using [`handshake_state`](QDtls::handshake_state). `false` return means some error occurred, use [`dtls_error`](QDtls::dtls_error) for more detailed information.
    ///
    /// **Note:** If the identity of the peer can't be established, the error is set to [`QDtlsError::PeerVerificationError`]. If you want to ignore verification errors and continue connecting, you must call [`ignore_verification_errors`](QDtls::ignore_verification_errors) and then [`resume_handshake`](QDtls::resume_handshake). If the errors cannot be ignored, you must call [`abort_handshake`](QDtls::abort_handshake).
    pub fn do_handshake(
        self: Pin<&mut Self>,
        socket: Pin<&mut QUdpSocket>,
        dgram: &QByteArray,
    ) -> bool {
        // SAFETY: `unpin_for_qt(socket)` is passed directly to Qt.
        unsafe { self.do_handshake_raw(unpin_for_qt(socket), dgram) }
    }

    /// If a timeout occurs during the handshake, the [`handshake_timeout`](QDtls::handshake_timeout) signal is emitted. The application must call this function to retransmit handshake messages. Returns `true` if a timeout has occurred, `false` otherwise.
    pub fn handle_timeout(self: Pin<&mut Self>, socket: Pin<&mut QUdpSocket>) -> bool {
        // SAFETY: `unpin_for_qt(socket)` is passed directly to Qt.
        unsafe { self.handle_timeout_raw(unpin_for_qt(socket)) }
    }

    /// Returns the peer's address, set by [`set_peer`](QDtls::set_peer), or `None`.
    pub fn peer_address(&self) -> Option<QHostAddress> {
        self.peer_address_or_null().nonnull()
    }

    /// If peer verification errors were ignored during the handshake, this function resumes and completes the handshake and returns `true`. Returns `false` if the handshake could not be resumed.
    pub fn resume_handshake(self: Pin<&mut Self>, socket: Pin<&mut QUdpSocket>) -> bool {
        // SAFETY: `unpin_for_qt(socket)` is passed directly to Qt.
        unsafe { self.resume_handshake_raw(unpin_for_qt(socket)) }
    }

    /// Returns the cryptographic cipher used by this connection, or `None` if the connection isn't encrypted. The cipher for the session is selected during the handshake phase. The cipher is used to encrypt and decrypt data.
    ///
    /// [`QSslConfiguration`](crate::QSslConfiguration) provides functions for setting the ordered list of ciphers from which the handshake phase will eventually select the session cipher. This ordered list must be in place before the handshake phase begins.
    pub fn session_cipher(&self) -> Option<QSslCipher> {
        self.session_cipher_or_null().nonnull()
    }

    /// Sends an encrypted shutdown alert message and closes the DTLS connection. Handshake state changes to [`QDtlsHandshakeState::HandshakeNotStarted`]. This function returns `true` on success.
    pub fn shutdown(self: Pin<&mut Self>, socket: Pin<&mut QUdpSocket>) -> bool {
        // SAFETY: `unpin_for_qt(socket)` is passed directly to Qt.
        unsafe { self.shutdown_raw(unpin_for_qt(socket)) }
    }

    /// Encrypts `dgram` and writes the encrypted data into `socket`. Returns the number of bytes written, or -1 in case of error. The handshake must be completed before writing encrypted data.
    pub fn write_datagram_encrypted(
        self: Pin<&mut Self>,
        socket: Pin<&mut QUdpSocket>,
        dgram: &QByteArray,
    ) -> i64 {
        // SAFETY: `unpin_for_qt(socket)` is passed directly to Qt.
        unsafe {
            self.write_datagram_encrypted_raw(unpin_for_qt(socket), dgram)
                .into()
        }
    }
}

impl Deref for QDtls {
    type Target = QObject;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl fmt::Display for QDtlsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

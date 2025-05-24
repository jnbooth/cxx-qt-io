use crate::util::{IsNonNull, MSecs};
use crate::{
    QAbstractSocket, QAbstractSocketNetworkLayerProtocol, QIODevice, QIODeviceOpenMode,
    QSslCertificate, QSslSslProtocol, QTcpSocket,
};
#[cfg(cxxqt_qt_version_at_least_6_1)]
use crate::{QSslImplementedClass, QSslSupportedFeature};
use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt::QObject;
use cxx_qt_lib::{QList, QString};
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;
use std::time::Duration;

#[cxx_qt::bridge]
mod ffi {
    /// Describes the peer verification modes for [`QSslSocket`]. The default mode is [`AutoVerifyPeer`](QSslSocketPeerVerifyMode::AutoVerifyPeer), which selects an appropriate mode depending on the socket's [`QSslSocketSslMode`].
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslSocketPeerVerifyMode {
        /// [`QSslSocket`] will not request a certificate from the peer. You can set this mode if you are not interested in the identity of the other side of the connection. The connection will still be encrypted, and your socket will still send its local certificate to the peer if it's requested.
        VerifyNone,
        /// [`QSslSocket`] will request a certificate from the peer, but does not require this certificate to be valid. This is useful when you want to display peer certificate details to the user without affecting the actual SSL handshake. This mode is the default for servers. Note: In Schannel this value acts the same as [`VerifyNone`](QSslSocketPeerVerifyMode::VerifyNone).
        QueryPeer,
        /// [`QSslSocket`] will request a certificate from the peer during the SSL handshake phase, and requires that this certificate is valid. On failure, [`QSslSocket`] will emit the [`QSslSocket::ssl_errors`] signal. This mode is the default for clients.
        VerifyPeer,
        /// [`QSslSocket`] will automatically use [`QueryPeer`](QSslSocketPeerVerifyMode::QueryPeer) for server sockets and [`VerifyPeer`](QSslSocketPeerVerifyMode::VerifyPeer) for client sockets.
        AutoVerifyPeer,
    }

    /// Describes the connection modes available for [`QSslSocket`].
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslSocketSslMode {
        /// The socket is unencrypted. Its behavior is identical to [`QTcpSocket`].
        UnencryptedMode,
        /// The socket is a client-side SSL socket. It is either already encrypted, or it is in the SSL handshake phase (see [`QSslSocket::is_encrypted`]).
        SslClientMode,
        /// The socket is a server-side SSL socket. It is either already encrypted, or it is in the SSL handshake phase (see [`QSslSocket::is_encrypted`]).
        SslServerMode,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-io/qiodevice.h");
        type QIODevice = crate::QIODevice;
        type QIODeviceOpenMode = crate::QIODeviceOpenMode;
        include!("cxx-qt-io/qabstractsocket.h");
        type QAbstractSocket = crate::QAbstractSocket;
        type QAbstractSocketNetworkLayerProtocol = crate::QAbstractSocketNetworkLayerProtocol;
        include!("cxx-qt-io/qssl.h");
        type QSslAlertLevel = crate::QSslAlertLevel;
        type QSslAlertType = crate::QSslAlertType;
        type QSslEncodingFormat = crate::QSslEncodingFormat;
        type QSslKeyAlgorithm = crate::QSslKeyAlgorithm;
        type QSslSslProtocol = crate::QSslSslProtocol;
        include!("cxx-qt-io/qsslcertificate.h");
        type QSslCertificate = crate::QSslCertificate;
        include!("cxx-qt-io/qsslcipher.h");
        type QSslCipher = crate::QSslCipher;
        include!("cxx-qt-io/qsslconfiguration.h");
        type QSslConfiguration = crate::QSslConfiguration;
        include!("cxx-qt-io/qsslerror.h");
        type QSslError = crate::QSslError;
        include!("cxx-qt-io/qsslkey.h");
        type QSslKey = crate::QSslKey;
        include!("cxx-qt-io/qsslpresharedkeyauthenticator.h");
        type QSslPreSharedKeyAuthenticator = crate::QSslPreSharedKeyAuthenticator;
        include!("cxx-qt-io/qlist.h");
        type QList_QString = cxx_qt_lib::QList<QString>;
        type QList_QSslCertificate = cxx_qt_lib::QList<QSslCertificate>;
        type QList_QSslError = cxx_qt_lib::QList<QSslError>;
        type QList_QSslSslProtocol = cxx_qt_lib::QList<QSslSslProtocol>;
        type QList_QOcspResponse = cxx_qt_lib::QList<crate::QOcspResponse>;
    }

    #[cfg(cxxqt_qt_version_at_least_6_1)]
    extern "C++" {
        type QSslImplementedClass = crate::QSslImplementedClass;
        type QSslSupportedFeature = crate::QSslSupportedFeature;
        type QList_QSslImplementedClass = cxx_qt_lib::QList<QSslImplementedClass>;
        type QList_QSslSupportedFeature = cxx_qt_lib::QList<QSslSupportedFeature>;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslsocket.h");
        type QSslSocketPeerVerifyMode;
        type QSslSocketSslMode;
        type QTcpSocket = crate::QTcpSocket;
    }

    unsafe extern "C++Qt" {
        /// The `QSslSocket` class provides an SSL encrypted socket for both clients and servers.
        ///
        /// Qt Documentation: [QSslSocket](https://doc.qt.io/qt-6/qsslsocket.html#details)
        #[qobject]
        #[base = QTcpSocket]
        type QSslSocket;

        /// Starts an encrypted connection to the device `host_name` on `port`, using `mode` as the open mode. This is equivalent to calling [`connect_to_host`](QAbstractSocket::connect_to_host) to establish the connection, followed by a call to [`start_client_encryption`](QSslSocket::start_client_encryption). The `protocol` parameter can be used to specify which network protocol to use (eg. IPv4 or IPv6). The `ssl_peer_name` enables the usage of a different host name for the certificate validation instead of the one used for the TCP connection (`host_name`).
        ///
        /// `QSslSocket` first enters the [`QAbstractSocketSocketState::HostLookupState`](crate::QAbstractSocketSocketState::HostLookupState). Then, after entering either the event loop or one of the `wait_for...()` functions, it enters the [`QAbstractSocketSocketState::ConnectingState`](crate::QAbstractSocketSocketState::ConnectingState), emits [`connected`](QAbstractSocket::connected), and then initiates the SSL client handshake. At each state change, `QSslSocket` emits signal [`state_changed`](QAbstractSocket::state_changed).
        ///
        /// After initiating the SSL client handshake, if the identity of the peer can't be established, signal [`ssl_errors`](QSslSocket::ssl_errors) is emitted. If you want to ignore the errors and continue connecting, you must call [`ignore_ssl_errors`](QSslSocket::ignore_ssl_errors), either from inside a slot function connected to the [`ssl_errors`](QSslSocket::ssl_errors) signal, or prior to entering encrypted mode. If [`ignore_ssl_errors`](QSslSocket::ignore_ssl_errors) is not called, the connection is dropped, signal [`disconnected`](QAbstractSocket::disconnected) is emitted, and `QSslSocket` returns to the [`QAbstractSocketSocketState::UnconnectedState`](crate::QAbstractSocketSocketState::UnconnectedState).
        ///
        /// If the SSL handshake is successful, `QSslSocket` emits [`encrypted`](QSslSocket::encrypted).
        #[rust_name = "connect_to_host_encrypted_with"]
        fn connectToHostEncrypted(
            self: Pin<&mut QSslSocket>,
            host_name: &QString,
            port: u16,
            ssl_peer_name: &QString,
            mode: QIODeviceOpenMode,
            protocol: QAbstractSocketNetworkLayerProtocol,
        );

        /// If an application wants to conclude a handshake even after receiving [`handshake_interrupted_on_error`](QSslSocket::handshake_interrupted_on_error) signal, it must call this function. This call must be done from a slot function attached to the signal. The signal-slot connection must be direct.
        #[rust_name = "continue_interrupted_handshake"]
        fn continueInterruptedHandshake(self: Pin<&mut QSslSocket>);

        /// Returns the number of encrypted bytes that are awaiting decryption. Normally, this function will return 0 because `QSslSocket` decrypts its incoming data as soon as it can.
        #[rust_name = "encrypted_bytes_available"]
        fn encryptedBytesAvailable(self: &QSslSocket) -> i64;

        /// Returns the number of encrypted bytes that are waiting to be written to the network.
        #[rust_name = "encrypted_bytes_to_write"]
        fn encryptedBytesToWrite(self: &QSslSocket) -> i64;

        /// This slot tells `QSslSocket` to ignore errors during `QSslSocket`'s handshake phase and continue connecting. If you want to continue with the connection even if errors occur during the handshake phase, then you must call this slot, either from a slot connected to [`ssl_errors`](QSslSocket::ssl_errors), or before the handshake phase. If you don't call this slot, either in response to errors or before the handshake, the connection will be dropped after the [`ssl_errors`](QSslSocket::ssl_errors) signal has been emitted.
        ///
        /// If there are no errors during the SSL handshake phase (i.e., the identity of the peer is established with no problems), QSslSocket will not emit the [`ssl_errors`](QSslSocket::ssl_errors) signal, and it is unnecessary to call this function.
        ///
        /// **Warning:** Be sure to always let the user inspect the errors reported by the [`ssl_errors`](QSslSocket::ssl_errors) signal, and only call this method upon confirmation from the user that proceeding is ok. If there are unexpected errors, the connection should be aborted. Calling this method without inspecting the actual errors will most likely pose a security risk for your application. Use it with great care!
        #[rust_name = "ignore_all_ssl_errors"]
        fn ignoreSslErrors(self: Pin<&mut QSslSocket>);

        /// This method tells `QSslSocket` to ignore only the errors given in `errors`.
        ///
        /// **Note:** Because most SSL errors are associated with a certificate, for most of them you must set the expected certificate this SSL error is related to.
        ///
        /// Multiple calls to this function will replace the list of errors that were passed in previous calls. You can clear the list of errors you want to ignore by calling this function with an empty list.
        #[rust_name = "ignore_ssl_errors"]
        fn ignoreSslErrors(self: Pin<&mut QSslSocket>, errors: &QList_QSslError);

        /// Returns `true` if the socket is encrypted; otherwise, `false` is returned.
        ///
        /// An encrypted socket encrypts all data that is written by calling [`write`](QIODevice::write) or [`put_char`](QIODevice::put_char) before the data is written to the network, and decrypts all incoming data as the data is received from the network, before you call [`read`](QIODevice::read), [`read_line`](QIODevice::read_line) or [`get_char`](QIODevice::get_char).
        ///
        /// `QSslSocket` emits [`encrypted`](QSslSocket::encrypted) when it enters encrypted mode.
        ///
        /// You can call [`session_cipher`](QSslSocket::session_cipher) to find which cryptographic cipher is used to encrypt and decrypt your data.
        #[rust_name = "is_encrypted"]
        fn isEncrypted(self: &QSslSocket) -> bool;

        #[doc(hidden)]
        #[rust_name = "local_certificate_or_empty"]
        fn localCertificate(self: &QSslSocket) -> QSslCertificate;

        /// Returns the socket's local certificate chain, or an empty list if no local certificates have been assigned.
        #[rust_name = "local_certificate_chain"]
        fn localCertificateChain(self: &QSslSocket) -> QList_QSslCertificate;

        /// Returns the current mode for the socket; either [`QSslSocketSslMode::UnencryptedMode`], where `QSslSocket` behaves identially to [`QTcpSocket`], or one of [`QSslSocketSslMode::SslClientMode`] or [`QSslSocketSslMode::SslServerMode`], where the client is either negotiating or in encrypted mode.
        ///
        /// When the mode changes, QSslSocket emits [`mode_changed`](QSslSocket::mode_changed).
        fn mode(self: &QSslSocket) -> QSslSocketSslMode;

        /// This function returns Online Certificate Status Protocol responses that a server may send during a TLS handshake using OCSP stapling. The list is empty if no definitive response or no response at all was received.
        #[rust_name = "ocsp_responses"]
        fn ocspResponses(self: &QSslSocket) -> QList_QOcspResponse;

        #[doc(hidden)]
        #[rust_name = "peer_certificate_or_null"]
        fn peerCertificate(self: &QSslSocket) -> QSslCertificate;

        /// Returns the peer's chain of digital certificates, or an empty list of certificates.
        ///
        /// Peer certificates are checked automatically during the handshake phase. This function is normally used to fetch certificates for display, or for performing connection diagnostics. Certificates contain information about the peer and the certificate issuers, including host name, issuer names, and issuer public keys.
        ///
        /// The peer certificates are set in `QSslSocket` during the handshake phase, so it is safe to call this function from a slot connected to the [`ssl_errors`](QSslSocket::ssl_errors) signal or the [`encrypted`](QSslSocket::encrypted) signal.
        ///
        /// If an empty list is returned, it can mean the SSL handshake failed, or it can mean the host you are connected to doesn't have a certificate, or it can mean there is no connection.
        ///
        /// If you want to get only the peer's immediate certificate, use [`peer_certificate`](QSslSocket::peer_certificate).
        #[rust_name = "peer_certificate_chain"]
        fn peerCertificateChain(self: &QSslSocket) -> QList_QSslCertificate;

        /// Returns the maximum number of certificates in the peer's certificate chain to be checked during the SSL handshake phase, or 0 (the default) if no maximum depth has been set, indicating that the whole certificate chain should be checked.
        ///
        /// The certificates are checked in issuing order, starting with the peer's own certificate, then its issuer's certificate, and so on.
        #[rust_name = "peer_verify_depth"]
        fn peerVerifyDepth(self: &QSslSocket) -> i32;

        /// Returns the socket's verify mode. This mode decides whether `QSslSocket` should request a certificate from the peer (i.e., the client requests a certificate from the server, or a server requesting a certificate from the client), and whether it should require that this certificate is valid.
        ///
        /// The default mode is [`QSslSocketPeerVerifyMode::AutoVerifyPeer`], which tells `QSslSocket` to use [`QSslSocketPeerVerifyMode::VerifyPeer`] for clients and [`QSslSocketPeerVerifyMode::QueryPeer`] for servers.
        #[rust_name = "peer_verify_mode"]
        fn peerVerifyMode(self: &QSslSocket) -> QSslSocketPeerVerifyMode;

        /// Returns the different hostname for the certificate validation, as set by [`set_peer_verify_name`](QSslSocket::set_peer_verify_name) or by [`connect_to_host_encrypted`](QSslSocket::connect_to_host_encrypted).
        #[rust_name = "peer_verify_name"]
        fn peerVerifyName(self: &QSslSocket) -> QString;

        /// Returns this socket's private key.
        #[rust_name = "private_key"]
        fn privateKey(self: &QSslSocket) -> QSslKey;

        /// Returns the socket's SSL protocol. By default, [`QSslSslProtocol::SecureProtocols`] is used.
        fn protocol(self: &QSslSocket) -> QSslSslProtocol;

        /// Returns the socket's cryptographic cipher, or a null cipher if the connection isn't encrypted. The socket's cipher for the session is set during the handshake phase. The cipher is used to encrypt and decrypt data transmitted through the socket.
        ///
        /// `QSslSocket` also provides functions for setting the ordered list of ciphers from which the handshake phase will eventually select the session cipher. This ordered list must be in place before the handshake phase begins.
        #[rust_name = "session_cipher"]
        fn sessionCipher(self: &QSslSocket) -> QSslCipher;

        #[doc(hidden)]
        #[rust_name = "session_protocol_or_default"]
        fn sessionProtocol(self: &QSslSocket) -> QSslSslProtocol;

        /// Sets the socket's local certificate to `certificate`. The local certificate is necessary if you need to confirm your identity to the peer. It is used together with the private key; if you set the local certificate, you must also set the private key.
        ///
        /// The local certificate and private key are always necessary for server sockets, but are also rarely used by client sockets if the server requires the client to authenticate.
        ///
        /// **Note:** Secure Transport SSL backend on macOS may update the default keychain (the default is probably your login keychain) by importing your local certificates and keys. This can also result in system dialogs showing up and asking for permission when your application is using these private keys. If such behavior is undesired, set the `QT_SSL_USE_TEMPORARY_KEYCHAIN` environment variable to a non-zero value; this will prompt `QSslSocket` to use its own temporary keychain.
        #[rust_name = "set_local_certificate"]
        fn setLocalCertificate(self: Pin<&mut QSslSocket>, certificate: &QSslCertificate);

        /// Sets the socket's local certificate to the first one found in file `path`, which is parsed according to the specified `format`.
        #[rust_name = "set_local_certificate_from_file"]
        fn setLocalCertificate(
            self: Pin<&mut QSslSocket>,
            path: &QString,
            format: QSslEncodingFormat,
        );

        /// Sets the certificate chain to be presented to the peer during the SSL handshake to be `local_chain`.
        #[rust_name = "set_local_certificate_chain"]
        fn setLocalCertificateChain(
            self: Pin<&mut QSslSocket>,
            local_chain: &QList_QSslCertificate,
        );

        /// Sets the maximum number of certificates in the peer's certificate chain to be checked during the SSL handshake phase, to `depth`. Setting a depth of 0 means that no maximum depth is set, indicating that the whole certificate chain should be checked.
        ///
        /// The certificates are checked in issuing order, starting with the peer's own certificate, then its issuer's certificate, and so on.
        #[rust_name = "set_peer_verify_depth"]
        fn setPeerVerifyDepth(self: Pin<&mut QSslSocket>, depth: i32);

        /// Sets the verify mode to `mode`. This mode decides whether `QSslSocket` should request a certificate from the peer (i.e., the client requests a certificate from the server, or a server requesting a certificate from the client), and whether it should require that this certificate is valid.
        ///
        /// The default mode is [`QSslSocketPeerVerifyMode::AutoVerifyPeer`], which tells `QSslSocket` to use [`QSslSocketPeerVerifyMode::VerifyPeer`] for clients and [`QSslSocketPeerVerifyMode::QueryPeer`] for servers.
        #[rust_name = "set_peer_verify_mode"]
        fn setPeerVerifyMode(self: Pin<&mut QSslSocket>, mode: QSslSocketPeerVerifyMode);

        /// Sets a different host name, given by `host_name`, for the certificate validation instead of the one used for the TCP connection.
        #[rust_name = "set_peer_verify_name"]
        fn setPeerVerifyName(self: Pin<&mut QSslSocket>, host_name: &QString);

        /// Sets the socket's private key to `key`. The private key and the local certificate are used by clients and servers that must prove their identity to SSL peers.
        ///
        /// Both the key and the local certificate are required if you are creating an SSL server socket. If you are creating an SSL client socket, the key and local certificate are required if your client must identify itself to an SSL server.
        #[rust_name = "set_private_key"]
        fn setPrivateKey(self: Pin<&mut QSslSocket>, key: &QSslKey);

        /// Reads the string in file `file_name` and decodes it using a specified `algorithm` and encoding `format` to construct an SSL key. If the encoded key is encrypted, `pass_phrase` is used to decrypt it.
        ///
        /// The socket's private key is set to the constructed key. The private key and the local certificate are used by clients and servers that must prove their identity to SSL peers.
        ///
        /// Both the key and the local certificate are required if you are creating an SSL server socket. If you are creating an SSL client socket, the key and local certificate are required if your client must identify itself to an SSL server.
        #[rust_name = "set_private_key_from_file"]
        fn setPrivateKey(
            self: Pin<&mut QSslSocket>,
            file_name: &QString,
            algorithm: QSslKeyAlgorithm,
            format: QSslEncodingFormat,
            pass_phrase: &QByteArray,
        );

        /// Sets the socket's SSL protocol to `protocol`. This will affect the next initiated handshake; calling this function on an already-encrypted socket will not affect the socket's protocol.
        #[rust_name = "set_protocol"]
        fn setProtocol(self: Pin<&mut QSslSocket>, protocol: QSslSslProtocol);

        /// Sets the socket's SSL configuration to be the contents of `configuration`. This function sets the local certificate, the ciphers, the private key and the CA certificates to those stored in configuration.
        ///
        /// It is not possible to set the SSL-state related fields.
        #[rust_name = "set_ssl_configuration"]
        fn setSslConfiguration(self: Pin<&mut QSslSocket>, configuration: &QSslConfiguration);

        /// Returns the socket's SSL configuration state. The default SSL configuration of a socket is to use the default ciphers, default CA certificates, no local private key or certificate.
        ///
        /// The SSL configuration also contains fields that can change with time without notice.
        #[rust_name = "ssl_configuration"]
        fn sslConfiguration(self: &QSslSocket) -> QSslConfiguration;

        /// Returns a list of the last SSL errors that occurred. This is the same list as QSslSocket passes via the [`ssl_errors`](QSslSocket::ssl_errors) signal. If the connection has been encrypted with no errors, this function will return an empty list.
        #[rust_name = "ssl_handshake_errors"]
        fn sslHandshakeErrors(self: &QSslSocket) -> QList_QSslError;

        /// Starts a delayed SSL handshake for a client connection. This function can be called when the socket is in the [`QAbstractSocketSocketState::ConnectedState`](crate::QAbstractSocketSocketState::ConnectedState) but still in the [`QSslSocketSslMode::UnencryptedMode`]. If it is not yet connected, or if it is already encrypted, this function has no effect.
        ///
        /// Clients that implement `STARTTLS` functionality often make use of delayed SSL handshakes. Most other clients can avoid calling this function directly by using [`connect_to_host_encrypted`](QSslSocket::connect_to_host_encrypted) instead, which automatically performs the handshake.
        #[rust_name = "start_client_encryption"]
        fn startClientEncryption(self: Pin<&mut QSslSocket>);

        /// Starts a delayed SSL handshake for a server connection. This function can be called when the socket is in the [`QAbstractSocketSocketState::ConnectedState`](crate::QAbstractSocketSocketState::ConnectedState) but still in the [`QSslSocketSslMode::UnencryptedMode`]. If it is not connected or it is already encrypted, the function has no effect.
        ///
        /// For server sockets, calling this function is the only way to initiate the SSL handshake. Most servers will call this function immediately upon receiving a connection, or as a result of having received a protocol-specific command to enter SSL mode (e.g, the server may respond to receiving the string `"STARTTLS\r\n"` by calling this function).
        #[rust_name = "start_server_encryption"]
        fn startServerEncryption(self: Pin<&mut QSslSocket>);

        #[doc(hidden)]
        #[rust_name = "wait_for_encrypted_msecs"]
        fn waitForEncrypted(self: Pin<&mut QSslSocket>, msecs: i32) -> bool;

        /// `QSslSocket` emits this signal if an alert message was received from a peer. `level` tells if the alert was fatal or it was a warning. `type` is the code explaining why the alert was sent. When a textual description of the alert message is available, it is supplied in `description`.
        ///
        /// **Note:** The signal is mostly for informational and debugging purposes and does not require any handling in the application. If the alert was fatal, underlying backend will handle it and close the connection.
        ///
        /// **Note:** Not all backends support this functionality.
        #[qsignal]
        #[rust_name = "alert_received"]
        fn alertReceived(
            self: Pin<&mut QSslSocket>,
            level: QSslAlertLevel,
            alert_type: QSslAlertType,
            description: &QString,
        );

        /// `QSslSocket` emits this signal if an alert message was sent to a peer. `level` tells if the alert was fatal or it was a warning. `type` is the code explaining why the alert was sent. When a textual description of the alert message is available, it is supplied in `description`.
        ///
        /// **Note:** The signal is mostly for informational and debugging purposes and does not require any handling in the application. If the alert was fatal, underlying backend will handle it and close the connection.
        ///
        /// **Note:** Not all backends support this functionality.
        #[qsignal]
        #[rust_name = "alert_sent"]
        fn alertSent(
            self: Pin<&mut QSslSocket>,
            level: QSslAlertLevel,
            alert_type: QSslAlertType,
            description: &QString,
        );

        /// This signal is emitted when `QSslSocket` enters encrypted mode. After this signal has been emitted, [`QSslSocket::is_encrypted`] will return `true`, and all further transmissions on the socket will be encrypted.
        #[qsignal]
        fn encrypted(self: Pin<&mut QSslSocket>);

        /// This signal is emitted when `QSslSocket` writes its encrypted data to the network. The written parameter contains the number of bytes that were successfully written.
        #[qsignal]
        #[rust_name = "encrypted_bytes_written"]
        fn encryptedBytesWritten(self: Pin<&mut QSslSocket>, written: i64);

        /// `QSslSocket` emits this signal if a certificate verification error was found and if early error reporting was enabled in [`QSslConfiguration`](crate::QSslConfiguration). An application is expected to inspect the error and decide if it wants to continue the handshake, or abort it and send an alert message to the peer. The signal-slot connection must be direct.
        #[qsignal]
        #[rust_name = "handshake_interrupted_on_error"]
        fn handshakeInterruptedOnError(self: Pin<&mut QSslSocket>, error: &QSslError);

        /// This signal is emitted when `QSslSocket` changes from [`QSslSocketSslMode::UnencryptedMode`] to either [`QSslSocketSslMode::SslClientMode`] or [`QSslSocketSslMode::SslServerMode`]. `mode` is the new mode.
        #[qsignal]
        #[rust_name = "mode_changed"]
        fn modeChanged(self: Pin<&mut QSslSocket>, mode: QSslSocketSslMode);

        /// If TLS 1.3 protocol was negotiated during a handshake, `QSslSocket` emits this signal after receiving NewSessionTicket message. Session and session ticket's lifetime hint are updated in the socket's configuration. The session can be used for session resumption (and a shortened handshake) in future TLS connections.
        ///
        /// **Note:** This functionality enabled only with OpenSSL backend and requires OpenSSL v 1.1.1 or above.
        #[qsignal]
        #[rust_name = "new_session_ticket_received"]
        fn newSessionTicketReceived(self: Pin<&mut QSslSocket>);

        /// `QSslSocket` can emit this signal several times during the SSL handshake, before encryption has been established, to indicate that an error has occurred while establishing the identity of the peer. The error is usually an indication that `QSslSocket` is unable to securely identify the peer.
        ///
        /// This signal provides you with an early indication when something's wrong. By connecting to this signal, you can manually choose to tear down the connection from inside the connected slot before the handshake has completed. If no action is taken, `QSslSocket` will proceed to emitting [`ssl_errors`](QSslSocket::ssl_errors).
        #[qsignal]
        #[rust_name = "peer_verify_error"]
        fn peerVerifyError(self: Pin<&mut QSslSocket>, error: &QSslError);

        /// `QSslSocket` emits this signal when it negotiates a PSK ciphersuite, and therefore a PSK authentication is then required.
        ///
        /// When using PSK, the client must send to the server a valid identity and a valid pre shared key, in order for the SSL handshake to continue. Applications can provide this information in a slot connected to this signal, by filling in the passed `authenticator` object according to their needs.
        ///
        /// **Note:** Ignoring this signal, or failing to provide the required credentials, will cause the handshake to fail, and therefore the connection to be aborted.
        ///
        /// **Note:** The `authenticator` object is owned by the socket and must not be deleted by the application.
        #[qsignal]
        #[rust_name = "pre_shared_key_authentication_required"]
        unsafe fn preSharedKeyAuthenticationRequired(
            self: Pin<&mut QSslSocket>,
            authenticator: *mut QSslPreSharedKeyAuthenticator,
        );

        /// `QSslSocket` emits this signal after the SSL handshake to indicate that one or more errors have occurred while establishing the identity of the peer. The errors are usually an indication that `QSslSocket` is unable to securely identify the peer. Unless any action is taken, the connection will be dropped after this signal has been emitted.
        ///
        /// If you want to continue connecting despite the errors that have occurred, you must call [`ignore_ssl_errors`](QSslSocket::ignore_ssl_errors) from inside a slot connected to this signal. If you need to access the error list at a later point, you can call [`ssl_handshake_errors`](QSslSocket::ssl_handshake_errors).
        ///
        /// `errors` contains one or more errors that prevent `QSslSocket` from verifying the identity of the peer.
        ///
        /// **Note:** You cannot use [`ConnectionType::QueuedConnection`](cxx_qt_lib::ConnectionType::QueuedConnection) when connecting to this signal, or calling [`ignore_ssl_errors`](QSslSocket::ignore_ssl_errors) will have no effect.
        #[qsignal]
        #[rust_name = "ssl_errors"]
        fn sslErrors(self: Pin<&mut QSslSocket>, errors: &QList_QSslError);
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qsslsocket_ssl_library_build_version_number"]
        fn qsslsocketSslLibraryBuildVersionNumber() -> i64;
        #[rust_name = "qsslsocket_ssl_library_build_version_string"]
        fn qsslsocketSslLibraryBuildVersionString() -> QString;

        #[rust_name = "qsslsocket_ssl_library_version_number"]
        fn qsslsocketSslLibraryVersionNumber() -> i64;
        #[rust_name = "qsslsocket_ssl_library_version_string"]
        fn qsslsocketSslLibraryVersionString() -> QString;

        #[rust_name = "qsslsocket_supports_ssl"]
        fn qsslsocketSupportsSsl() -> bool;
    }

    #[cfg(cxxqt_qt_version_at_least_6_1)]
    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qsslsocket_active_backend"]
        fn qsslsocketActiveBackend() -> QString;
        #[rust_name = "qsslsocket_available_backends"]
        fn qsslsocketAvailableBackends() -> QList_QString;
        #[rust_name = "qsslsocket_set_active_backend"]
        fn qsslsocketSetActiveBackend(backend: &QString) -> bool;

        #[rust_name = "qsslsocket_implemented_classes"]
        fn qsslsocketImplementedClasses(backend: &QString) -> QList_QSslImplementedClass;
        #[rust_name = "qsslsocket_supported_features"]
        fn qsslsocketSupportedFeatures(backend: &QString) -> QList_QSslSupportedFeature;
        #[rust_name = "qsslsocket_supported_protocols"]
        fn qsslsocketSupportedProtocols(backend: &QString) -> QList_QSslSslProtocol;

        #[rust_name = "qsslsocket_is_class_implemented"]
        fn qsslsocketIsClassImplemented(support: QSslImplementedClass, backend: &QString) -> bool;
        #[rust_name = "qsslsocket_is_feature_supported"]
        fn qsslsocketIsFeatureSupported(support: QSslSupportedFeature, backend: &QString) -> bool;
        #[rust_name = "qsslsocket_is_protocol_supported"]
        fn qsslsocketIsProtocolSupported(support: QSslSslProtocol, backend: &QString) -> bool;
    }

    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/casting.h");

        #[rust_name = "upcast_qsslsocket_qobject"]
        unsafe fn upcastPtr(socket: *const QSslSocket) -> *const QObject;
        #[rust_name = "downcast_qobject_qsslsocket"]
        unsafe fn downcastPtr(socket: *const QObject) -> *const QSslSocket;

        #[rust_name = "upcast_qsslsocket_qiodevice"]
        unsafe fn upcastPtr(socket: *const QSslSocket) -> *const QIODevice;
        #[rust_name = "downcast_qiodevice_qsslsocket"]
        unsafe fn downcastPtr(socket: *const QIODevice) -> *const QSslSocket;

        #[rust_name = "upcast_qsslsocket_qabstractsocket"]
        unsafe fn upcastPtr(socket: *const QSslSocket) -> *const QAbstractSocket;
        #[rust_name = "downcast_qabstractsocket_qsslsocket"]
        unsafe fn downcastPtr(socket: *const QAbstractSocket) -> *const QSslSocket;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslsocket_init_default"]
        fn make_unique() -> UniquePtr<QSslSocket>;
    }
}

pub use ffi::{QSslSocket, QSslSocketPeerVerifyMode, QSslSocketSslMode};

impl QSslSocket {
    /// Starts an encrypted connection to the device `host_name` on `port`, using `mode` as the open mode. This is equivalent to calling [`connect_to_host`](QAbstractSocket::connect_to_host) to establish the connection, followed by a call to [`start_client_encryption`](QSslSocket::start_client_encryption).
    ///
    /// `QSslSocket` first enters the [`QAbstractSocketSocketState::HostLookupState`](crate::QAbstractSocketSocketState::HostLookupState). Then, after entering either the event loop or one of the `wait_for...()` functions, it enters the [`QAbstractSocketSocketState::ConnectingState`](crate::QAbstractSocketSocketState::ConnectingState), emits [`connected`](QAbstractSocket::connected), and then initiates the SSL client handshake. At each state change, `QSslSocket` emits signal [`state_changed`](QAbstractSocket::state_changed).
    ///
    /// After initiating the SSL client handshake, if the identity of the peer can't be established, signal [`ssl_errors`](QSslSocket::ssl_errors) is emitted. If you want to ignore the errors and continue connecting, you must call [`ignore_ssl_errors`](QSslSocket::ignore_ssl_errors), either from inside a slot function connected to the [`ssl_errors`](QSslSocket::ssl_errors) signal, or prior to entering encrypted mode. If [`ignore_ssl_errors`](QSslSocket::ignore_ssl_errors) is not called, the connection is dropped, signal [`disconnected`](QAbstractSocket::disconnected) is emitted, and `QSslSocket` returns to the [`QAbstractSocketSocketState::UnconnectedState`](crate::QAbstractSocketSocketState::UnconnectedState).
    ///
    /// If the SSL handshake is successful, `QSslSocket` emits [`encrypted`](QSslSocket::encrypted).
    pub fn connect_to_host_encrypted(
        self: Pin<&mut Self>,
        host_name: &QString,
        port: u16,
        mode: QIODeviceOpenMode,
    ) {
        self.connect_to_host_encrypted_with(
            host_name,
            port,
            &QString::default(),
            mode,
            QAbstractSocketNetworkLayerProtocol::AnyIPProtocol,
        );
    }

    /// Constructs a `QSslSocket` object. The new socket's cipher suite is set to be the one returned by [`QSslConfiguration::default_configuration()`](crate::QSslConfiguration::default_configuration)`.`[`ciphers()`](crate::QSslConfiguration::ciphers).
    pub fn new() -> UniquePtr<Self> {
        ffi::qsslsocket_init_default()
    }

    /// Returns the socket's local certificate, or `None` if no local certificate has been assigned.
    pub fn local_certificate(&self) -> Option<QSslCertificate> {
        self.local_certificate_or_empty().nonnull()
    }

    /// Returns the peer's digital certificate (i.e., the immediate certificate of the host you are connected to), or `None`, if the peer has not assigned a certificate.
    ///
    /// The peer certificate is checked automatically during the handshake phase, so this function is normally used to fetch the certificate for display or for connection diagnostic purposes. It contains information about the peer, including its host name, the certificate issuer, and the peer's public key.
    ///
    /// Because the peer certificate is set during the handshake phase, it is safe to access the peer certificate from a slot connected to the [`ssl_errors`](QSslSocket::ssl_errors) signal or the [`encrypted`](QSslSocket::encrypted) signal.
    ///
    /// If `None` is returned, it can mean the SSL handshake failed, or it can mean the host you are connected to doesn't have a certificate, or it can mean there is no connection.
    ///
    /// If you want to check the peer's complete chain of certificates, use [`peer_certificate_chain`](QSslSocket::peer_certificate_chain) to get them all at once.
    pub fn peer_certificate(&self) -> Option<QSslCertificate> {
        self.peer_certificate_or_null().nonnull()
    }

    /// Returns the socket's SSL/TLS protocol or `None` if the connection isn't encrypted. The socket's protocol for the session is set during the handshake phase.
    pub fn session_protocol(&self) -> Option<QSslSslProtocol> {
        let protocol = self.session_protocol_or_default();
        if protocol == QSslSslProtocol::UnknownProtocol && !self.is_encrypted() {
            None
        } else {
            Some(protocol)
        }
    }

    /// Waits until the socket has completed the SSL handshake and has emitted [`encrypted`](QSslSocket::encrypted), or `duration`, whichever comes first. If [`encrypted`](QSslSocket::encrypted) has been emitted, this function returns `true`; otherwise (e.g., the socket is disconnected, or the SSL handshake fails), `false` is returned.
    ///
    /// If `duration` is `None`, this function will not time out.
    pub fn wait_for_encrypted(self: Pin<&mut Self>, duration: Option<Duration>) -> bool {
        self.wait_for_encrypted_msecs(duration.msecs())
    }

    /// Returns the version number of the SSL library in use at compile time. If no SSL support is available then this will return `None`.
    pub fn ssl_library_build_version_number() -> Option<i64> {
        let version = ffi::qsslsocket_ssl_library_build_version_number();
        if version == -1 {
            None
        } else {
            Some(version)
        }
    }

    /// Returns the version string of the SSL library in use at compile time. If no SSL support is available then this will return `None`.
    pub fn ssl_library_build_version_string() -> Option<QString> {
        ffi::qsslsocket_ssl_library_build_version_string().nonnull()
    }

    /// Returns the version number of the SSL library in use. If no SSL support is available then this will return `None`.
    pub fn ssl_library_version_number() -> Option<i64> {
        let version = ffi::qsslsocket_ssl_library_version_number();
        if version == -1 {
            None
        } else {
            Some(version)
        }
    }

    /// Returns the version string of the SSL library in use. If no SSL support is available then this will return `None`.
    pub fn ssl_library_version_string() -> Option<QString> {
        ffi::qsslsocket_ssl_library_version_string().nonnull()
    }

    /// Casts this object to `QIODevice`.
    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    /// Mutably casts this object to `QIODevice`.
    pub fn as_io_device_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QIODevice> {
        self.as_mut().upcast_pin()
    }

    /// Casts this object to `QAbstractSocket`.
    pub fn as_abstract_socket(&self) -> &QAbstractSocket {
        self.upcast()
    }

    /// Mutably casts this object to `QAbstractSocket`.
    pub fn as_abstract_socket_mut<'a>(
        self: &'a mut Pin<&mut Self>,
    ) -> Pin<&'a mut QAbstractSocket> {
        self.as_mut().upcast_pin()
    }

    /// Casts this object to `QTcpSocket`.
    pub fn as_tcp_socket(&self) -> &QTcpSocket {
        self.upcast()
    }

    /// Mutably casts this object to `QSslSocket`.
    pub fn as_tcp_socket_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QTcpSocket> {
        self.as_mut().upcast_pin()
    }
}

#[cfg(cxxqt_qt_version_at_least_6_1)]
impl QSslSocket {
    /// Returns the name of the backend that `QSslSocket` and related classes use. If the active backend was not set explicitly, this function returns the name of a default backend that `QSslSocket` selects implicitly from the list of available backends.
    ///
    /// **Note:** When selecting a default backend implicitly, `QSslSocket` prefers the OpenSSL backend if available. If it's not available, the Schannel backend is implicitly selected on Windows, and Secure Transport on Darwin platforms. Failing these, if a custom TLS backend is found, it is used. If no other backend is found, the "certificate only" backend is selected.
    ///
    /// Introduced in Qt 6.1.
    pub fn active_backend() -> QString {
        ffi::qsslsocket_active_backend()
    }

    /// Returns the names of the currently available backends. These names are in lower case, e.g. `"openssl"`, `"securetransport"`, `"schannel"` (similar to the already existing feature names for TLS backends in Qt).
    ///
    /// Introduced in Qt 6.1.
    pub fn available_backends() -> QList<QString> {
        ffi::qsslsocket_available_backends()
    }

    /// This function returns backend-specific classes implemented by the backend named `backend_name`. If `backend_name` is `None`, it is understood as a query about the currently active backend.
    ///
    /// Introduced in Qt 6.1.
    pub fn implemented_classes(backend_name: Option<&QString>) -> QList<QSslImplementedClass> {
        match backend_name {
            Some(backend_name) => ffi::qsslsocket_implemented_classes(backend_name),
            None => ffi::qsslsocket_implemented_classes(&QString::default()),
        }
    }

    /// Returns `true` if a class `cl` is implemented by the backend named `backend_name`. If `backend_name` is `None`, it is understood as a query about the currently active backend.
    ///
    /// Introduced in Qt 6.1.
    pub fn is_class_implemented(cl: QSslImplementedClass, backend_name: Option<&QString>) -> bool {
        match backend_name {
            Some(backend_name) => ffi::qsslsocket_is_class_implemented(cl, backend_name),
            None => ffi::qsslsocket_is_class_implemented(cl, &QString::default()),
        }
    }

    /// Returns `true` if a feature `ft` is supported by the backend named `backend_name`. If `backend_name` is `None`, it is understood as a query about the currently active backend.
    ///
    /// Introduced in Qt 6.1.
    pub fn is_feature_supported(ft: QSslSupportedFeature, backend_name: Option<&QString>) -> bool {
        match backend_name {
            Some(backend_name) => ffi::qsslsocket_is_feature_supported(ft, backend_name),
            None => ffi::qsslsocket_is_feature_supported(ft, &QString::default()),
        }
    }

    /// Returns `true` if `protocol` is supported by the backend named `backend_name`. If `backend_name` is `None`, it is understood as a query about the currently active backend.
    ///
    /// Introduced in Qt 6.1.
    pub fn is_protocol_supported(
        protocol: QSslSslProtocol,
        backend_name: Option<&QString>,
    ) -> bool {
        match backend_name {
            Some(backend_name) => ffi::qsslsocket_is_protocol_supported(protocol, backend_name),
            None => ffi::qsslsocket_is_protocol_supported(protocol, &QString::default()),
        }
    }

    /// Returns `true` if a backend with name `backend_name` was set as active backend. `backend_name` must be one of names returned by [`QSslSocket::available_backends()`].
    ///
    /// Introduced in Qt 6.1.
    pub fn set_active_backend(backend_name: &QString) -> bool {
        ffi::qsslsocket_set_active_backend(backend_name)
    }

    /// This function returns features supported by a backend named `backend_name`. If `backend_name` is `None`, it is understood as a query about the currently active backend.
    ///
    /// Introduced in Qt 6.1.
    pub fn supported_features(backend_name: Option<&QString>) -> QList<QSslSupportedFeature> {
        match backend_name {
            Some(backend_name) => ffi::qsslsocket_supported_features(backend_name),
            None => ffi::qsslsocket_supported_features(&QString::default()),
        }
    }

    /// If a backend with name `backend_name` is available, this function returns the list of TLS protocol versions supported by this backend. Otherwise, this function returns an empty list. If `backend_name` is `None`, it is understood as a query about the currently active backend.
    ///
    /// Introduced in Qt 6.1.
    pub fn supported_protocols(backend_name: Option<&QString>) -> QList<QSslSslProtocol> {
        match backend_name {
            Some(backend_name) => ffi::qsslsocket_supported_protocols(backend_name),
            None => ffi::qsslsocket_supported_protocols(&QString::default()),
        }
    }

    /// Returns `true` if this platform supports SSL; otherwise, returns `false`. If the platform doesn't support SSL, the socket will fail in the connection phase.
    ///
    /// Introduced in Qt 6.1.
    pub fn supports_ssl() -> bool {
        ffi::qsslsocket_supports_ssl()
    }
}

impl Deref for QSslSocket {
    type Target = QTcpSocket;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl AsRef<QTcpSocket> for QSslSocket {
    fn as_ref(&self) -> &QTcpSocket {
        self.upcast()
    }
}

unsafe impl Upcast<QAbstractSocket> for QSslSocket {
    unsafe fn upcast_ptr(this: *const Self) -> *const QAbstractSocket {
        ffi::upcast_qsslsocket_qabstractsocket(this)
    }

    unsafe fn from_base_ptr(base: *const QAbstractSocket) -> *const Self {
        ffi::downcast_qabstractsocket_qsslsocket(base)
    }
}

impl AsRef<QAbstractSocket> for QSslSocket {
    fn as_ref(&self) -> &QAbstractSocket {
        self.upcast()
    }
}

unsafe impl Upcast<QIODevice> for QSslSocket {
    unsafe fn upcast_ptr(this: *const Self) -> *const QIODevice {
        ffi::upcast_qsslsocket_qiodevice(this)
    }

    unsafe fn from_base_ptr(base: *const QIODevice) -> *const Self {
        ffi::downcast_qiodevice_qsslsocket(base)
    }
}

impl AsRef<QIODevice> for QSslSocket {
    fn as_ref(&self) -> &QIODevice {
        self.upcast()
    }
}

unsafe impl Upcast<QObject> for QSslSocket {
    unsafe fn upcast_ptr(this: *const Self) -> *const QObject {
        ffi::upcast_qsslsocket_qobject(this)
    }

    unsafe fn from_base_ptr(base: *const QObject) -> *const Self {
        ffi::downcast_qobject_qsslsocket(base)
    }
}

impl AsRef<QObject> for QSslSocket {
    fn as_ref(&self) -> &QObject {
        self.upcast()
    }
}

impl Read for Pin<&mut QSslSocket> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_io_device_mut().read(buf)
    }
}

impl Write for Pin<&mut QSslSocket> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_io_device_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.as_abstract_socket_mut().flush();
        Ok(())
    }
}

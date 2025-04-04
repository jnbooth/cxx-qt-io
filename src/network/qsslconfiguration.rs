use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};
use cxx_qt_lib::{QByteArray, QList, QVariant};

use crate::util::NonNull;
use crate::{QSslCertificate, QSslCipher, QSslEllipticCurve, QSslKey};

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslConfigurationNextProtocolNegotiationStatus {
        /// No application protocol has been negotiated (yet).
        NextProtocolNegotiationNone,
        /// A next protocol has been negotiated (see `next_ngotiated_protocol()`).
        NextProtocolNegotiationNegotiated,
        /// The client and server could not agree on a common next application protocol.
        NextProtocolNegotiationUnsupported,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
        include!("cxx-qt-io/qssl.h");
        type QSslEncodingFormat = crate::QSslEncodingFormat;
        type QSslSslProtocol = crate::QSslSslProtocol;
        type QSslSslOption = crate::QSslSslOption;
        include!("cxx-qt-io/qsslcertificate.h");
        type QSslCertificate = crate::QSslCertificate;
        type QSslCertificatePatternSyntax = crate::QSslCertificatePatternSyntax;
        include!("cxx-qt-io/qsslcipher.h");
        type QSslCipher = crate::QSslCipher;
        include!("cxx-qt-io/qssldiffiehellmanparameters.h");
        type QSslDiffieHellmanParameters = crate::QSslDiffieHellmanParameters;
        include!("cxx-qt-io/qsslkey.h");
        type QSslKey = crate::QSslKey;
        include!("cxx-qt-io/qsslsocket.h");
        type QSslSocketPeerVerifyMode = crate::QSslSocketPeerVerifyMode;
        include!("cxx-qt-io/qlist.h");
        type QList_QByteArray = cxx_qt_lib::QList<cxx_qt_lib::QByteArray>;
        type QList_QSslCertificate = cxx_qt_lib::QList<QSslCertificate>;
        type QList_QSslCipher = cxx_qt_lib::QList<crate::QSslCipher>;
        type QList_QSslEllipticCurve = cxx_qt_lib::QList<crate::QSslEllipticCurve>;
        include!("cxx-qt-io/qmap.h");
        type QMap_QByteArray_QVariant = cxx_qt_lib::QMap<crate::QMapPair_QByteArray_QVariant>;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslconfiguration.h");
        type QSslConfigurationNextProtocolNegotiationStatus;
    }

    unsafe extern "C++" {
        type QSslConfiguration = super::QSslConfiguration;

        /// Adds `certificate` to this configuration's CA certificate database. The certificate database must be set prior to the SSL handshake. The CA certificate database is used by the socket during the handshake phase to validate the peer's certificate.
        ///
        /// **Note:** The default configuration uses the system CA certificate database. If that is not available (as is commonly the case on iOS), the default database is empty.
        #[rust_name = "add_ca_certificate"]
        fn addCaCertificate(&mut self, certificate: &QSslCertificate);

        /// Adds `certificates` to this configuration's CA certificate database. The certificate database must be set prior to the SSL handshake. The CA certificate database is used by the socket during the handshake phase to validate the peer's certificate.
        ///
        /// **Note:** The default configuration uses the system CA certificate database. If that is not available (as is commonly the case on iOS), the default database is empty.
        #[rust_name = "add_ca_certificates"]
        fn addCaCertificates(&mut self, certificates: &QList_QSslCertificate);

        /// Searches all files in the `path` for certificates encoded in the specified `format` and adds them to this socket's CA certificate database. `path` must be a file or a pattern matching one or more files, as specified by `syntax`. Returns `true` if one or more certificates are added to the socket's CA certificate database; otherwise returns `false`.
        ///
        /// The CA certificate database is used by the socket during the handshake phase to validate the peer's certificate.
        ///
        /// For more precise control, use `add_ca_certificate()`.
        #[rust_name = "add_ca_certificates_from_file"]
        fn addCaCertificates(
            &mut self,
            path: &QString,
            format: QSslEncodingFormat,
            syntax: QSslCertificatePatternSyntax,
        ) -> bool;

        /// This function returns the allowed protocols to be negotiated with the server through the Next Protocol Negotiation (NPN) or Application-Layer Protocol Negotiation (ALPN) TLS extension, as set by `set_allowed_next_protocols()`.
        #[rust_name = "allowed_next_protocols"]
        fn allowedNextProtocols(&self) -> QList_QByteArray;

        /// Returns the backend-specific configuration.
        ///
        /// Only options set by `set_backend_configuration_option()` or `set_backend_configuration()` will be returned. The internal standard configuration of the backend is not reported.
        #[rust_name = "backend_configuration"]
        fn backendConfiguration(&self) -> QMap_QByteArray_QVariant;

        /// Returns this connection's CA certificate database. The CA certificate database is used by the socket during the handshake phase to validate the peer's certificate. It can be modified prior to the handshake with `set_ca_certificates()`, or with `add_ca_certificate()` and `add_ca_certificates()`.
        #[rust_name = "ca_certificates"]
        fn caCertificates(&self) -> QList_QSslCertificate;

        /// Returns this connection's current cryptographic cipher suite. This list is used during the handshake phase for choosing a session cipher. The returned list of ciphers is ordered by descending preference. (i.e., the first cipher in the list is the most preferred cipher). The session cipher will be the first one in the list that is also supported by the peer.
        ///
        /// By default, the handshake phase can choose any of the ciphers supported by this system's SSL libraries, which may vary from system to system. The list of ciphers supported by this system's SSL libraries is returned by `supported_ciphers()`. You can restrict the list of ciphers used for choosing the session cipher for this socket by calling `set_ciphers()` with a subset of the supported ciphers. You can revert to using the entire set by calling `set_ciphers()` with the list returned by `supported_ciphers()`.
        fn ciphers(&self) -> QList_QSslCipher;

        /// Retrieves the current set of Diffie-Hellman parameters.
        #[rust_name = "diffie_hellman_parameters"]
        fn diffieHellmanParameters(&self) -> QSslDiffieHellmanParameters;

        /// This function returns `true` if DTLS cookie verification was enabled on a server-side socket.
        #[rust_name = "dtls_cookie_verification_enabled"]
        fn dtlsCookieVerificationEnabled(&self) -> bool;

        /// Returns this connection's current list of elliptic curves. This list is used during the handshake phase for choosing an elliptic curve (when using an elliptic curve cipher). The returned list of curves is ordered by descending preference (i.e., the first curve in the list is the most preferred one).
        ///
        /// By default, the handshake phase can choose any of the curves supported by this system's SSL libraries, which may vary from system to system. The list of curves supported by this system's SSL libraries is returned by `QSslSocket::supported_elliptic_curves()`.
        ///
        /// You can restrict the list of curves used for choosing the session cipher for this socket by calling `set_elliptic_curves()` with a subset of the supported ciphers. You can revert to using the entire set by calling `set_elliptic_curves()` with the list returned by `supported_elliptic_curves()`.
        #[rust_name = "elliptic_curves"]
        fn ellipticCurves(&self) -> QList_QSslEllipticCurve;

        /// Returns the ephemeral server key used for cipher algorithms with forward secrecy, e.g. DHE-RSA-AES128-SHA.
        ///
        /// The ephemeral key is only available when running in client mode, i.e. `SslClientMode`. When running in server mode or using a cipher algorithm without forward secrecy a null key is returned. The ephemeral server key will be set before emitting the `encrypted()` signal.
        #[cfg(cxxqt_qt_version_at_least_6_0)]
        #[rust_name = "ephemeral_server_key"]
        fn ephemeralServerKey(&self) -> QSslKey;

        /// Returns `true` if a verification callback will emit handshake_interrupted_on_error()` early, before concluding the handshake.
        ///
        /// **Note:** This function always returns `false` for all backends but OpenSSL.
        #[rust_name = "handshake_must_interrupt_on_error"]
        fn handshakeMustInterruptOnError(&self) -> bool;

        /// Returns `true` if this is a null `QSslConfiguration` object.
        ///
        /// A `QSslConfiguration` object is null if it has been default-constructed and no setter methods have been called.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        /// Returns the certificate to be presented to the peer during the SSL handshake process.
        #[rust_name = "local_certificate"]
        fn localCertificate(&self) -> QSslCertificate;

        /// Returns the certificate chain to be presented to the peer during the SSL handshake process.
        #[rust_name = "local_certificate_chain"]
        fn localCertificateChain(&self) -> QList_QSslCertificate;

        /// Returns `true` if errors with code `NoPeerCertificate` cannot be ignored.
        ///
        /// **Note:** Always returns `false` for all TLS backends but OpenSSL.
        #[cfg(cxxqt_qt_version_at_least_6_0)]
        #[rust_name = "missing_certificate_is_fatal"]
        fn missingCertificateIsFatal(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "next_negotiated_protocol_or_empty"]
        fn nextNegotiatedProtocol(&self) -> QByteArray;

        /// This function returns the status of the Next Protocol Negotiation (NPN) or Application-Layer Protocol Negotiation (ALPN). If the feature has not been enabled through `set_allowed_next_protocols()`, this function returns `NextProtocolNegotiationNone`. The status will be set before emitting the `encrypted()` signal.
        #[rust_name = "next_protocol_negotiation_status"]
        fn nextProtocolNegotiationStatus(&self) -> QSslConfigurationNextProtocolNegotiationStatus;

        /// Returns `true` if OCSP stapling was enabled by `set_ocsp_stapling_enabled()`, otherwise `false` (which is the default value).
        #[rust_name = "ocsp_stapling_enabled"]
        fn ocspStaplingEnabled(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "peer_certificate_or_null"]
        fn peerCertificate(&self) -> QSslCertificate;

        /// Returns the peer's chain of digital certificates, starting with the peer's immediate certificate and ending with the CA's certificate.
        ///
        /// Peer certificates are checked automatically during the handshake phase. This function is normally used to fetch certificates for display, or for performing connection diagnostics. Certificates contain information about the peer and the certificate issuers, including host name, issuer names, and issuer public keys.
        ///
        /// Because the peer certificate is set during the handshake phase, it is safe to access the peer certificate from a slot connected to the `QSslSocket::ssl_errors()` signal, `QNetworkReply::ssl_errors()` signal, or the `QSslSocket::encrypted()` signal.
        ///
        /// If an empty list is returned, it can mean the SSL handshake failed, or it can mean the host you are connected to doesn't have a certificate, or it can mean there is no connection.
        ///
        /// If you want to get only the peer's immediate certificate, use `peer_certificate()`.
        #[rust_name = "peer_certificate_chain"]
        fn peerCertificateChain(&self) -> QList_QSslCertificate;

        /// Returns the maximum number of certificates in the peer's certificate chain to be checked during the SSL handshake phase, or 0 (the default) if no maximum depth has been set, indicating that the whole certificate chain should be checked.
        ///
        /// The certificates are checked in issuing order, starting with the peer's own certificate, then its issuer's certificate, and so on.
        #[rust_name = "peer_verify_depth"]
        fn peerVerifyDepth(&self) -> i32;

        /// Returns the verify mode. This mode decides whether `QSslSocket` should request a certificate from the peer (i.e., the client requests a certificate from the server, or a server requesting a certificate from the client), and whether it should require that this certificate is valid.
        ///
        /// The default mode is `AutoVerifyPeer`, which tells `QSslSocket` to use `VerifyPeer` for clients, `QueryPeer` for servers.
        #[rust_name = "peer_verify_mode"]
        fn peerVerifyMode(&self) -> QSslSocketPeerVerifyMode;

        /// Returns the identity hint.
        #[rust_name = "pre_shared_key_identity_hint"]
        fn preSharedKeyIdentityHint(&self) -> QByteArray;

        #[doc(hidden)]
        #[rust_name = "private_key_or_null"]
        fn privateKey(&self) -> QSslKey;

        /// Returns the protocol setting for this SSL configuration.
        fn protocol(&self) -> QSslSslProtocol;

        #[doc(hidden)]
        #[rust_name = "session_cipher_or_null"]
        fn sessionCipher(&self) -> QSslCipher;

        /// Returns the socket's SSL/TLS protocol or UnknownProtocol if the connection isn't encrypted. The socket's protocol for the session is set during the handshake phase.
        #[rust_name = "session_protocol"]
        fn sessionProtocol(&self) -> QSslSslProtocol;

        #[doc(hidden)]
        #[rust_name = "session_ticket_or_empty"]
        fn sessionTicket(&self) -> QByteArray;

        #[doc(hidden)]
        #[rust_name = "session_ticket_life_time_hint_or_negative"]
        fn sessionTicketLifeTimeHint(&self) -> i32;

        /// This function sets the allowed protocols to be negotiated with the server through the Next Protocol Negotiation (NPN) or Application-Layer Protocol Negotiation (ALPN) TLS extension; each element in protocols must define one allowed protocol. The function must be called explicitly before connecting to send the NPN/ALPN extension in the SSL handshake. Whether or not the negotiation succeeded can be queried through `next_protocol_negotiation_status()`.
        #[rust_name = "set_allowed_next_protocols"]
        fn setAllowedNextProtocols(&mut self, protocols: &QList_QByteArray);

        /// Sets or clears the backend-specific configuration.
        #[rust_name = "set_backend_configuration"]
        fn setBackendConfiguration(&mut self, backend_configuration: &QMap_QByteArray_QVariant);

        #[doc(hidden)]
        #[rust_name = "set_backend_configuration_option_variant"]
        fn setBackendConfigurationOption(&mut self, name: &QByteArray, value: &QVariant);

        /// Sets this socket's CA certificate database to be `certificates`. The certificate database must be set prior to the SSL handshake. The CA certificate database is used by the socket during the handshake phase to validate the peer's certificate.
        ///
        /// **Note:** The default configuration uses the system CA certificate database. If that is not available (as is commonly the case on iOS), the default database is empty.
        #[rust_name = "set_ca_certificates"]
        fn setCaCertificates(&mut self, certificates: &QList_QSslCertificate);

        /// Sets the cryptographic cipher suite for this socket to ciphers, which must contain a subset of the ciphers in the list returned by `supported_ciphers()`.
        ///
        /// Restricting the cipher suite must be done before the handshake phase, where the session cipher is chosen.
        #[rust_name = "set_ciphers"]
        fn setCiphers(&mut self, ciphers: &QList_QSslCipher);

        /// Sets a custom set of Diffie-Hellman parameters to be used by this socket when functioning as a server to dhparams.
        #[rust_name = "set_diffie_hellman_parameters"]
        fn setDiffieHellmanParameters(&mut self, dhparams: &QSslDiffieHellmanParameters);

        /// This function enables DTLS cookie verification when `enable` is `true`.
        #[rust_name = "set_dtls_cookie_verification_enabled"]
        fn setDtlsCookieVerificationEnabled(&mut self, enable: bool);

        /// Sets the list of elliptic curves to be used by this socket to curves, which must contain a subset of the curves in the list returned by `supported_elliptic_curves()`.
        ///
        /// Restricting the elliptic curves must be done before the handshake phase, where the session cipher is chosen.
        #[rust_name = "set_elliptic_curves"]
        fn setEllipticCurves(&mut self, curves: &QList_QSslEllipticCurve);

        /// If `interrupt` is true and the underlying backend supports this option, errors found during certificate verification are reported immediately by emitting `QSslSocket::handshake_interrupted_on_error()`. This allows to stop the unfinished handshake and send a proper alert message to a peer. No special action is required from the application in this case. `QSslSocket` will close the connection after sending the alert message. If the application after inspecting the error wants to continue the handshake, it must call `QSslSocket::continue_interrupted_handshake()` from its slot function. The signal-slot connection must be direct.
        ///
        /// **Note:** When interrupting handshake is enabled, errors that would otherwise be reported by `QSslSocket::peer_verify_error()` are instead only reported by `QSslSocket::handshake_interrupted_on_error()`.
        ///
        /// **Note:** Even if the handshake was continued, these errors will be reported when emitting `QSslSocket::ssl_errors()` signal (and thus must be ignored in the corresponding function slot).
        #[cfg(cxxqt_qt_version_at_least_6_0)]
        #[rust_name = "set_handshake_must_interrupt_on_error"]
        fn setHandshakeMustInterruptOnError(&mut self, interrupt: bool);

        /// Sets the certificate to be presented to the peer during SSL handshake to be `certificate`.
        ///
        /// Setting the certificate once the connection has been established has no effect.
        ///
        /// A certificate is the means of identification used in the SSL process. The local certificate is used by the remote end to verify the local user's identity against its list of Certification Authorities. In most cases, such as in HTTP web browsing, only servers identify to the clients, so the client does not send a certificate.
        #[rust_name = "set_local_certificate"]
        fn setLocalCertificate(&mut self, certificate: &QSslCertificate);

        /// Sets the certificate chain to be presented to the peer during the SSL handshake to be `local_chain`.
        ///
        /// Setting the certificate chain once the connection has been established has no effect.
        ///
        /// A certificate is the means of identification used in the SSL process. The local certificate is used by the remote end to verify the local user's identity against its list of Certification Authorities. In most cases, such as in HTTP web browsing, only servers identify to the clients, so the client does not send a certificate.
        ///
        /// Unlike `set_local_certificate()` this method allows you to specify any intermediate certificates required in order to validate your certificate. The first item in the list must be the leaf certificate.
        #[rust_name = "set_local_certificate_chain"]
        fn setLocalCertificateChain(&mut self, local_chain: &QList_QSslCertificate);

        /// If `cannot_recover` is true, and verification mode in use is `VerifyPeer` or `AutoVerifyPeer` (for a client-side socket), the missing peer's certificate would be treated as an unrecoverable error that cannot be ignored. A proper alert message will be sent to the peer before closing the connection.
        ///
        /// **Note:** Only available if Qt was configured and built with OpenSSL backend.
        #[cfg(cxxqt_qt_version_at_least_6_0)]
        #[rust_name = "set_missing_certificate_is_fatal"]
        fn setMissingCertificateIsFatal(&mut self, cannot_recover: bool);

        /// If `enabled` is true, client `QSslSocket` will send a certificate status request to its peer when initiating a handshake. During the handshake `QSslSocket` will verify the server's response. This value must be set before the handshake starts.
        #[rust_name = "set_ocsp_stapling_enabled"]
        fn setOcspStaplingEnabled(&mut self, enabled: bool);

        /// Sets the maximum number of certificates in the peer's certificate chain to be checked during the SSL handshake phase, to `depth`. Setting a depth of 0 means that no maximum depth is set, indicating that the whole certificate chain should be checked.
        ///
        /// The certificates are checked in issuing order, starting with the peer's own certificate, then its issuer's certificate, and so on.
        #[rust_name = "set_peer_verify_depth"]
        fn setPeerVerifyDepth(&mut self, depth: i32);

        /// Sets the verify mode to `mode`. This mode decides whether `QSslSocket` should request a certificate from the peer (i.e., the client requests a certificate from the server, or a server requesting a certificate from the client), and whether it should require that this certificate is valid.
        ///
        /// The default mode is `AutoVerifyPeer`, which tells `QSslSocket` to use `VerifyPeer` for clients, `QueryPeer` for servers.
        #[rust_name = "set_peer_verify_mode"]
        fn setPeerVerifyMode(&mut self, mode: QSslSocketPeerVerifyMode);

        /// Sets the identity hint for a preshared key authentication to `hint`. This will affect the next initiated handshake; calling this function on an already-encrypted socket will not affect the socket's identity hint.
        ///
        /// The identity hint is used in `SslServerMode` only!
        #[rust_name = "set_pre_shared_key_identity_hint"]
        fn setPreSharedKeyIdentityHint(&mut self, hint: &QByteArray);

        /// Sets the connection's private key to `key`. The private key and the local certificate are used by clients and servers that must prove their identity to SSL peers.
        ///
        /// Both the key and the local certificate are required if you are creating an SSL server socket. If you are creating an SSL client socket, the key and local certificate are required if your client must identify itself to an SSL server.
        #[rust_name = "set_private_key"]
        fn setPrivateKey(&mut self, key: &QSslKey);

        /// Sets the protocol setting for this configuration to be `protocol`.
        ///
        /// Setting the protocol once the connection has already been established has no effect.
        #[rust_name = "set_protocol"]
        fn setProtocol(&mut self, protocol: QSslSslProtocol);

        /// Sets the session ticket to be used in an SSL handshake. `SslOptionDisableSessionPersistence` must be turned off for this to work, and `session_ticket` must be in ASN.1 format as returned by `session_ticket()`.
        #[rust_name = "set_session_ticket"]
        fn setSessionTicket(&mut self, session_ticket: &QByteArray);

        /// Enables or disables an SSL compatibility `option`. If `on` is `true`, the `option` is enabled. If `on` is `false`, the `option` is disabled.
        #[rust_name = "set_ssl_option"]
        fn setSslOption(&mut self, option: QSslSslOption, on: bool);

        /// Returns `true` if the specified SSL compatibility `option` is enabled.
        #[rust_name = "test_ssl_option"]
        fn testSslOption(&self, option: QSslSslOption) -> bool;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qsslconfiguration_default_configuration"]
        fn qsslconfigurationDefaultConfiguration() -> QSslConfiguration;

        #[rust_name = "qsslconfiguration_default_dtls_configuration"]
        fn qsslconfigurationDefaultDtlsConfiguration() -> QSslConfiguration;

        #[rust_name = "qsslconfiguration_set_default_configuration"]
        fn qsslconfigurationSetDefaultConfiguration(configuration: &QSslConfiguration);

        #[rust_name = "qsslconfiguration_set_default_dtls_configuration"]
        fn qsslconfigurationSetDefaultDtlsConfiguration(configuration: &QSslConfiguration);

        #[rust_name = "qsslconfiguration_supported_ciphers"]
        fn qsslconfigurationSupportedCiphers() -> QList_QSslCipher;

        #[rust_name = "qsslconfiguration_supported_elliptic_curves"]
        fn qsslconfigurationSupportedEllipticCurves() -> QList_QSslEllipticCurve;

        #[rust_name = "qsslconfiguration_system_ca_certificates"]
        fn qsslconfigurationSystemCaCertificates() -> QList_QSslCertificate;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslconfiguration_drop"]
        fn drop(extension: &mut QSslConfiguration);

        #[rust_name = "qsslconfiguration_init_default"]
        fn construct() -> QSslConfiguration;
        #[rust_name = "qsslconfiguration_clone"]
        fn construct(other: &QSslConfiguration) -> QSslConfiguration;

        #[rust_name = "qsslconfiguration_eq"]
        fn operatorEq(a: &QSslConfiguration, b: &QSslConfiguration) -> bool;
    }
}

pub use ffi::QSslConfigurationNextProtocolNegotiationStatus;

/// The `QSslConfiguration` class holds the configuration and state of an SSL connection.
///
/// Qt Documentation: [QSslConfiguration](https://doc.qt.io/qt-6/qsslconfiguration.html#details)
#[repr(C)]
pub struct QSslConfiguration {
    _space: MaybeUninit<usize>,
}

impl Clone for QSslConfiguration {
    fn clone(&self) -> Self {
        ffi::qsslconfiguration_clone(self)
    }
}

impl Drop for QSslConfiguration {
    fn drop(&mut self) {
        ffi::qsslconfiguration_drop(self);
    }
}

impl Default for QSslConfiguration {
    /// Constructs an empty SSL configuration. This configuration contains no valid settings and the state will be empty. `is_null()` will return true after this constructor is called.
    ///
    /// Once any setter methods are called, `is_null()` will return false.
    fn default() -> Self {
        ffi::qsslconfiguration_init_default()
    }
}

impl PartialEq for QSslConfiguration {
    fn eq(&self, other: &Self) -> bool {
        ffi::qsslconfiguration_eq(self, other)
    }
}

impl Eq for QSslConfiguration {}

impl QSslConfiguration {
    /// Returns the default SSL configuration to be used in new SSL connections.
    ///
    /// The default SSL configuration consists of:
    /// * no local certificate and no private key
    /// * protocol `SecureProtocols`
    /// * the system's default CA certificate list
    /// * the cipher list equal to the list of the SSL libraries' supported SSL ciphers that are 128 bits or more
    pub fn default_configuration() -> Self {
        ffi::qsslconfiguration_default_configuration()
    }

    /// Returns the default DTLS configuration to be used in new DTLS connections.
    ///
    /// The default DTLS configuration consists of:
    /// * no local certificate and no private key
    /// * protocol `DtlsV1_2OrLater`
    /// * the system's default CA certificate list
    /// * the cipher list equal to the list of the SSL libraries' supported TLS 1.2 ciphers that are 128 bits or more
    pub fn default_dtls_configuration() -> Self {
        ffi::qsslconfiguration_default_dtls_configuration()
    }

    /// Sets the default SSL configuration to be used in new SSL connections to be `configuration`. Existing connections are not affected by this call.
    pub fn set_default_configuration(configuration: &QSslConfiguration) {
        ffi::qsslconfiguration_set_default_configuration(configuration);
    }

    /// Sets the default DTLS configuration to be used in new DTLS connections to be `configuration`. Existing connections are not affected by this call.
    pub fn set_default_dtls_configuration(configuration: &QSslConfiguration) {
        ffi::qsslconfiguration_set_default_dtls_configuration(configuration);
    }

    /// Returns the list of cryptographic ciphers supported by this system. This list is set by the system's SSL libraries and may vary from system to system.
    pub fn supported_ciphers(&self) -> QList<QSslCipher> {
        ffi::qsslconfiguration_supported_ciphers()
    }

    /// Returns the list of elliptic curves supported by this system. This list is set by the system's SSL libraries and may vary from system to system.
    pub fn supported_elliptic_curves(&self) -> QList<QSslEllipticCurve> {
        ffi::qsslconfiguration_supported_elliptic_curves()
    }

    /// This function provides the CA certificate database provided by the operating system. The CA certificate database returned by this function is used to initialize the database returned by `ca_certificates()` on the default `QSslConfiguration`.
    pub fn system_ca_certificates(&self) -> QList<QSslCertificate> {
        ffi::qsslconfiguration_system_ca_certificates()
    }

    /// This function returns the protocol negotiated with the server if the Next Protocol Negotiation (NPN) or Application-Layer Protocol Negotiation (ALPN) TLS extension was enabled. In order for the NPN/ALPN extension to be enabled, `set_allowed_next_protocols()` needs to be called explicitly before connecting to the server.
    ///
    /// Returns `None` if no protocol could be negotiated or the extension was not enabled.
    pub fn next_negotiated_protocol(&self) -> Option<QByteArray> {
        self.next_negotiated_protocol_or_empty().nonnull()
    }

    /// Returns the peer's digital certificate (i.e., the immediate certificate of the host you are connected to), or a null certificate, if the peer has not assigned a certificate.
    ///
    /// The peer certificate is checked automatically during the handshake phase, so this function is normally used to fetch the certificate for display or for connection diagnostic purposes. It contains information about the peer, including its host name, the certificate issuer, and the peer's public key.
    ///
    /// Because the peer certificate is set during the handshake phase, it is safe to access the peer certificate from a slot connected to the `QSslSocket::ssl_errors()` signal, `QNetworkReply::ssl_errors()` signal, or the `QSslSocket::encrypted()` signal.
    ///
    /// If `None` is returned, it can mean the SSL handshake failed, or it can mean the host you are connected to doesn't have a certificate, or it can mean there is no connection.
    ///
    /// If you want to check the peer's complete chain of certificates, use `peer_certificate_chain()` to get them all at once.
    pub fn peer_certificate(&self) -> Option<QSslCertificate> {
        self.peer_certificate_or_null().nonnull()
    }

    /// Returns the SSL key assigned to this connection or `None` if none has been assigned yet.
    pub fn private_key(&self) -> Option<QSslKey> {
        self.private_key_or_null().nonnull()
    }

    /// Returns the socket's cryptographic cipher, or `None` if the connection isn't encrypted. The socket's cipher for the session is set during the handshake phase. The cipher is used to encrypt and decrypt data transmitted through the socket.
    ///
    /// The SSL infrastructure also provides functions for setting the ordered list of ciphers from which the handshake phase will eventually select the session cipher. This ordered list must be in place before the handshake phase begins.
    pub fn session_cipher(&self) -> Option<QSslCipher> {
        self.session_cipher_or_null().nonnull()
    }

    /// If `SslOptionDisableSessionPersistence` was turned off, this function returns the session ticket used in the SSL handshake in ASN.1 format, suitable to e.g. be persisted to disk. If no session ticket was used or `SslOptionDisableSessionPersistence` was not turned off, this function returns `None`.
    ///
    /// **Note:** When persisting the session ticket to disk or similar, be careful not to expose the session to a potential attacker, as knowledge of the session allows for eavesdropping on data encrypted with the session parameters.
    pub fn session_ticket(&self) -> Option<QByteArray> {
        self.session_ticket_or_empty().nonnull()
    }

    /// If `SslOptionDisableSessionPersistence` was turned off, this function returns the session ticket life time hint sent by the server (which might be 0). If the server did not send a session ticket (e.g. when resuming a session or when the server does not support it) or `SslOptionDisableSessionPersistence` was not turned off, this function returns `None`.
    pub fn session_ticket_life_time_hint(&self) -> Option<i32> {
        let hint = self.session_ticket_life_time_hint_or_negative();
        if hint == -1 {
            None
        } else {
            Some(hint)
        }
    }

    /// Sets the option name in the backend-specific configuration to `value`.
    ///
    /// Options supported by the OpenSSL (>= 1.0.2) backend are available in the supported configuration file commands documentation. The expected type for the `value` parameter is a `QByteArray` for all options.
    pub fn set_backend_configuration_option<T>(&mut self, name: &QByteArray, value: T)
    where
        T: Into<QVariant>,
    {
        self.set_backend_configuration_option_variant(name, &value.into());
    }
}

unsafe impl ExternType for QSslConfiguration {
    type Id = type_id!("QSslConfiguration");
    type Kind = cxx::kind::Trivial;
}

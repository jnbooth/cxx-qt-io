use cxx_qt_lib::QFlags;

#[cfg(cxxqt_qt_version_at_least_5_13)]
mod alternative_name_entry_type_5_13;
#[cfg(cxxqt_qt_version_at_least_5_13)]
pub use alternative_name_entry_type_5_13::QSslAlternativeNameEntryType;

#[cfg(not(cxxqt_qt_version_at_least_5_13))]
mod alternative_name_entry_type;
#[cfg(not(cxxqt_qt_version_at_least_5_13))]
pub use alternative_name_entry_type::QSslAlternativeNameEntryType;

#[cfg(cxxqt_qt_version_at_least_6_3)]
mod protocol_6_3;
#[cfg(cxxqt_qt_version_at_least_6_3)]
pub use protocol_6_3::QSslSslProtocol;

#[cfg(all(
    not(cxxqt_qt_version_at_least_6_3),
    any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_12)
))]
mod protocol_5_12;
#[cfg(all(
    not(cxxqt_qt_version_at_least_6_3),
    any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_12)
))]
pub use protocol_5_12::QSslSslProtocol;

#[cfg(not(any(cxxqt_qt_version_at_least_5_12, cxxqt_qt_version_at_least_6_0)))]
mod protocol;
#[cfg(not(any(cxxqt_qt_version_at_least_5_12, cxxqt_qt_version_at_least_6_0)))]
pub use protocol::QSslSslProtocol;

#[cxx::bridge]
mod ffi {
    /// Describes the two types of keys `QSslKey` supports.
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslKeyType {
        /// A private key.
        PrivateKey,
        /// A public key.
        PublicKey,
    }

    /// Describes supported encoding formats for certificates and keys.
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslEncodingFormat {
        /// The PEM format.
        Pem,
        /// The DER format.
        Der,
    }

    /// Describes the different key algorithms supported by `QSslKey`.
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslKeyAlgorithm {
        /// A key that should be treated as a 'black box' by `QSslKey`. Allows applications to add support for facilities such as PKCS#11 that Qt does not currently offer natively.
        Opaque,
        /// The RSA algorithm.
        Rsa,
        /// The DSA algorithm.
        Dsa,
        /// The Elliptic Curve algorithm.
        Ec,
        /// The Diffie-Hellman algorithm.
        Dh,
    }

    /// Describes the options that can be used to control the details of SSL behaviour. These options are generally used to turn features off to work around buggy servers.
    ///
    /// By default, `SslOptionDisableEmptyFragments` is turned on since this causes problems with a large number of servers. `SslOptionDisableLegacyRenegotiation` is also turned on, since it introduces a security risk. `SslOptionDisableCompression` is turned on to prevent the attack publicised by CRIME. `SslOptionDisableSessionPersistence` is turned on to optimize memory usage. The other options are turned off.
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslSslOption {
        /// Disables the insertion of empty fragments into the data when using block ciphers. When enabled, this prevents some attacks (such as the BEAST attack), however it is incompatible with some servers.
        SslOptionDisableEmptyFragments = 0x01,
        /// Disables the SSL session ticket extension. This can cause slower connection setup, however some servers are not compatible with the extension.
        SslOptionDisableSessionTickets = 0x02,
        /// Disables the SSL compression extension. When enabled, this allows the data being passed over SSL to be compressed, however some servers are not compatible with this extension.
        SslOptionDisableCompression = 0x04,
        /// Disables the SSL server name indication extension. When enabled, this tells the server the virtual host being accessed allowing it to respond with the correct certificate.
        SslOptionDisableServerNameIndication = 0x08,
        /// Disables the older insecure mechanism for renegotiating the connection parameters. When enabled, this option can allow connections for legacy servers, but it introduces the possibility that an attacker could inject plaintext into the SSL session.
        SslOptionDisableLegacyRenegotiation = 0x10,
        /// Disables SSL session sharing via the session ID handshake attribute.
        SslOptionDisableSessionSharing = 0x20,
        /// Disables storing the SSL session in ASN.1 format as returned by `QSslConfiguration::session_ticket()`. Enabling this feature adds memory overhead of approximately 1K per used session ticket.
        SslOptionDisableSessionPersistence = 0x40,
        /// Disables selecting the cipher chosen based on the servers preferences rather than the order ciphers were sent by the client. This option is only relevant to server sockets, and is only honored by the OpenSSL backend.
        SslOptionDisableServerCipherPreference = 0x80,
    }

    /// Describes the level of an alert message.
    #[cfg(cxxqt_qt_version_at_least_6_0)]
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslAlertLevel {
        /// Non-fatal alert message.
        Warning,
        /// Fatal alert message, the underlying backend will handle such an alert properly and close the connection.
        Fatal,
        /// An alert of unknown level of severity.
        Unknown,
    }

    /// See [RFC 8446, section 6](https://datatracker.ietf.org/doc/html/rfc8446#section-6) for the possible values and their meaning.
    #[cfg(cxxqt_qt_version_at_least_6_0)]
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslAlertType {
        CloseNotify,
        UnexpectedMessage = 10,
        BadRecordMac = 20,
        RecordOverflow = 22,
        DecompressionFailure = 30,
        HandshakeFailure = 40,
        NoCertificate = 41,
        BadCertificate = 42,
        UnsupportedCertificate = 43,
        CertificateRevoked = 44,
        CertificateExpired = 45,
        CertificateUnknown = 46,
        IllegalParameter = 47,
        UnknownCa = 48,
        AccessDenied = 49,
        DecodeError = 50,
        DecryptError = 51,
        ExportRestriction = 60,
        ProtocolVersion = 70,
        InsufficientSecurity = 71,
        InternalError = 80,
        InappropriateFallback = 86,
        UserCancelled = 90,
        NoRenegotiation = 100,
        MissingExtension = 109,
        UnsupportedExtension = 110,
        CertificateUnobtainable = 111,
        UnrecognizedName = 112,
        BadCertificateStatusResponse = 113,
        BadCertificateHashValue = 114,
        UnknownPskIdentity = 115,
        CertificateRequired = 116,
        NoApplicationProtocol = 120,
        UnknownAlertMessage = 255,
    }

    /// Enumerates classes that a TLS backend implements
    ///
    /// In `QtNetwork`, some classes have backend-specific implementation and thus can be left unimplemented. Enumerators in this enum indicate, which class has a working implementation in the backend.
    #[cfg(cxxqt_qt_version_at_least_6_1)]
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslImplementedClass {
        /// Class `QSslKey`.
        Key,
        /// Class `QSslCertificate`.
        Certificate,
        /// Class `QSslSocket`.
        Socket,
        /// Class `QSslDiffieHellmanParameters`.
        DiffieHellman,
        /// Class `QSslEllipticCurve`.
        EllipticCurve,
        /// Class `QDtls`.
        Dtls,
        /// Class `QDtlsClientVerifier`.
        DtlsCookie,
    }

    /// Enumerates possible features that a TLS backend supports.
    ///
    /// In QtNetwork TLS-related classes have public API, that may be left unimplemented by some backend, for example, our SecureTransport backend does not support server-side ALPN. Enumerators from `SslSupportedFeature` enum indicate that a particular feature is supported.
    #[cfg(cxxqt_qt_version_at_least_6_1)]
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslSupportedFeature {
        /// Indicates that `QSslCertificate::verify()` is implemented by the backend.
        CertificateVerification,
        /// Client-side ALPN (Application Layer Protocol Negotiation).
        ClientSideAlpn,
        /// Server-side ALPN.
        ServerSideAlpn,
        /// OCSP stapling (Online Certificate Status Protocol).
        Ocsp,
        /// Pre-shared keys.
        Psk,
        /// Session tickets.
        SessionTicket,
        /// Information about alert messages sent and received.
        Alerts,
    }

    extern "C++" {
        include!("cxx-qt-io/qssl.h");
        type QSslKeyType;
        type QSslEncodingFormat;
        type QSslKeyAlgorithm;
        type QSslSslOption;
        #[cfg(cxxqt_qt_version_at_least_6_0)]
        type QSslAlertLevel;
        #[cfg(cxxqt_qt_version_at_least_6_0)]
        type QSslAlertType;
        #[cfg(cxxqt_qt_version_at_least_6_1)]
        type QSslImplementedClass;
        #[cfg(cxxqt_qt_version_at_least_6_1)]
        type QSslSupportedFeature;
    }
}

pub use ffi::{QSslEncodingFormat, QSslKeyAlgorithm, QSslKeyType, QSslSslOption};

#[cfg(cxxqt_qt_version_at_least_6_0)]
pub use ffi::{QSslAlertLevel, QSslAlertType};

#[cfg(cxxqt_qt_version_at_least_6_1)]
pub use ffi::{QSslImplementedClass, QSslSupportedFeature};

pub type QSslOptions = QFlags<QSslSslOption>;

unsafe_impl_qflag!(QSslSslOption, "QSslSslOptions");

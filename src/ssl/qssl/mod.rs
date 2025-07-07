#[cfg(cxxqt_qt_version_at_least_6_1)]
mod v6_1;
use std::fmt;

use cxx_qt_lib::QFlags;
#[cfg(cxxqt_qt_version_at_least_6_1)]
pub use v6_1::{QSslImplementedClass, QSslSupportedFeature};

#[cxx::bridge]
mod ffi {
    /// Describes the key types for alternative name entries in [`QSslCertificate`](crate::QSslCertificate).
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslAlternativeNameEntryType {
        /// An email entry; the entry contains an email address that the certificate is valid for.
        EmailEntry,
        /// A DNS host name entry; the entry contains a host name entry that the certificate is valid for. The entry may contain wildcards.
        DnsEntry,
        /// An IP address entry; the entry contains an IP address entry that the certificate is valid for.
        IpAddressEntry,
    }

    /// Describes the two types of keys [`QSslKey`](crate::QSslKey) supports.
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
    enum QSslEncodingFormat {
        /// The PEM format.
        Pem,
        /// The DER format.
        Der,
    }

    /// Describes the different key algorithms supported by [`QSslKey`](crate::QSslKey).
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslKeyAlgorithm {
        /// A key that should be treated as a 'black box' by [`QSslKey`](crate::QSslKey). Allows applications to add support for facilities such as PKCS#11 that Qt does not currently offer natively.
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
    /// By default, [`SslOptionDisableEmptyFragments`](QSslSslOption::SslOptionDisableEmptyFragments) is turned on since this causes problems with a large number of servers. [`SslOptionDisableLegacyRenegotiation`](QSslSslOption::SslOptionDisableLegacyRenegotiation) is also turned on, since it introduces a security risk. [`SslOptionDisableCompression`](QSslSslOption::SslOptionDisableCompression) is turned on to prevent the attack publicised by CRIME. [`SslOptionDisableSessionPersistence`](QSslSslOption::SslOptionDisableSessionPersistence) is turned on to optimize memory usage. The other options are turned off.
    #[repr(i32)]
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
        /// Disables storing the SSL session in ASN.1 format as returned by [`QSslConfiguration::session_ticket`](crate::QSslConfiguration::session_ticket). Enabling this feature adds memory overhead of approximately 1K per used session ticket.
        SslOptionDisableSessionPersistence = 0x40,
        /// Disables selecting the cipher chosen based on the servers preferences rather than the order ciphers were sent by the client. This option is only relevant to server sockets, and is only honored by the OpenSSL backend.
        SslOptionDisableServerCipherPreference = 0x80,
    }

    /// Describes the protocol of the cipher.
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslSslProtocol {
        /// TLSv1.2.
        TlsV1_2 = 2,
        /// Any supported protocol. This value is used by [`QSslSocket`](crate::QSslSocket) only.
        AnyProtocol,
        /// The default option, using protocols known to be secure.
        SecureProtocols,

        /// TLSv1.2 and later versions.
        TlsV1_2OrLater = 7,

        /// DTLSv1.2.
        DtlsV1_2 = 10,
        /// DTLSv1.2 and later versions.
        DtlsV1_2OrLater,

        /// TLSv1.3.
        TlsV1_3,
        /// TLSv1.3 and later versions.
        TlsV1_3OrLater,

        /// The cipher's protocol cannot be determined.
        UnknownProtocol = -1,
    }

    /// Describes the level of an alert message.
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

    extern "C++" {
        include!("cxx-qt-io/qssl.h");
        type QSslAlternativeNameEntryType;
        type QSslKeyType;
        type QSslEncodingFormat;
        type QSslKeyAlgorithm;
        type QSslSslOption;
        type QSslSslProtocol;
        type QSslAlertLevel;
        type QSslAlertType;
    }
}

pub use ffi::{
    QSslAlertLevel, QSslAlertType, QSslAlternativeNameEntryType, QSslEncodingFormat,
    QSslKeyAlgorithm, QSslKeyType, QSslSslOption, QSslSslProtocol,
};

/// [`QFlags`] of [`QSslSslOption`].
pub type QSslSslOptions = QFlags<QSslSslOption>;

unsafe_impl_qflag!(QSslSslOption, "QSslSslOptions");

impl fmt::Display for QSslKeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::PrivateKey => "private",
            Self::PublicKey => "public",
            _ => "unknown",
        })
    }
}

impl fmt::Display for QSslEncodingFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::Pem => "PEM",
            Self::Der => "DER",
            _ => "unknown",
        })
    }
}

impl fmt::Display for QSslKeyAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::Opaque => "opaque",
            Self::Rsa => "RSA",
            Self::Dsa => "DSA",
            Self::Ec => "elliptic-curve",
            Self::Dh => "Diffie-Hellman",
            _ => "unknown",
        })
    }
}

impl fmt::Display for QSslSslProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::TlsV1_2 => "TLS v1.2",
            Self::AnyProtocol => "any protocol",
            Self::SecureProtocols => "secure protocols",
            Self::TlsV1_2OrLater => "TLS v1.2+",
            Self::DtlsV1_2 => "DTLS v1.2",
            Self::DtlsV1_2OrLater => "DTLS v1.2+",
            Self::TlsV1_3 => "TLS v1.3",
            Self::TlsV1_3OrLater => "TLS v1.3+",
            Self::UnknownProtocol => "unknown protocol",
            _ => unreachable!(),
        })
    }
}

impl fmt::Display for QSslAlertLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::Warning => "warning",
            Self::Fatal => "fatal",
            _ => "unknown",
        })
    }
}

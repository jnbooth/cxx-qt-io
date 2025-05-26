use std::fmt;

#[cxx::bridge]
mod ffi {
    /// Enumerates classes that a TLS backend implements.
    ///
    /// In QtNetwork, some classes have backend-specific implementation and thus can be left unimplemented. Enumerators in this enum indicate, which class has a working implementation in the backend.
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslImplementedClass {
        /// Class [`QSslKey`](crate::QSslKey).
        Key,
        /// Class [`QSslCertificate`](crate::QSslCertificate).
        Certificate,
        /// Class [`QSslSocket`](crate::QSslSocket).
        Socket,
        /// Class [`QSslDiffieHellmanParameters`](crate::QSslDiffieHellmanParameters).
        DiffieHellman,
        /// Class [`QSslEllipticCurve`](crate::QSslEllipticCurve).
        EllipticCurve,
        /// Class [`QDtls`](crate::QDtls).
        Dtls,
        /// Class [`QDtlsClientVerifier`](crate::QDtlsClientVerifier).
        DtlsCookie,
    }

    /// Enumerates possible features that a TLS backend supports.
    ///
    /// In QtNetwork TLS-related classes have public API, that may be left unimplemented by some backend, for example, our SecureTransport backend does not support server-side ALPN. Enumerators from this enum indicate that a particular feature is supported.
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslSupportedFeature {
        /// Indicates that [`QSslCertificate::verify`](crate::QSslCertificate::verify) is implemented by the backend.
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
        type QSslImplementedClass;
        type QSslSupportedFeature;
    }
}

pub use ffi::{QSslImplementedClass, QSslSupportedFeature};

impl fmt::Display for QSslImplementedClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::Key => "QSslKey",
            Self::Certificate => "QSslCertificate",
            Self::Socket => "QSslSocket",
            Self::DiffieHellman => "QSslDiffieHellmanParameters",
            Self::EllipticCurve => "QSslEllipticCurve",
            Self::Dtls => "QDtls",
            Self::DtlsCookie => "QDtlsClientVerifier",
            _ => "unknown",
        })
    }
}

impl fmt::Display for QSslSupportedFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::CertificateVerification => "certificate verification",
            Self::ClientSideAlpn => "client-side ALPN",
            Self::ServerSideAlpn => "server-side ALPN",
            Self::Ocsp => "OCSP stapling",
            Self::Psk => "pre-shared keys",
            Self::SessionTicket => "session tickets",
            Self::Alerts => "alert messages",
            _ => "unknown",
        })
    }
}

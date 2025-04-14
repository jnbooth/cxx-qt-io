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

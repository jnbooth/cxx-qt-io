#[cxx::bridge]
mod ffi {

    /// Enumerates classes that a TLS backend implements.
    ///
    /// In QtNetwork, some classes have backend-specific implementation and thus can be left unimplemented. Enumerators in this enum indicate, which class has a working implementation in the backend.
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
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
    }

    extern "C++" {
        include!("cxx-qt-io/qssl.h");
        type QSslImplementedClass;
    }
}

pub use ffi::QSslImplementedClass;

#[cxx::bridge]
mod ffi {
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

    extern "C++" {
        include!("cxx-qt-io/qssl.h");
        type QSslSslProtocol;
    }
}

pub use ffi::QSslSslProtocol;

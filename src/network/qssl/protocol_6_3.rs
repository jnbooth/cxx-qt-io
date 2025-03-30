#[cxx::bridge]
mod ffi {
    /// Describes the protocol of the cipher.
    #[repr(i32)]
    #[derive(Debug)]
    enum SslProtocol {
        TlsV1_2 = 2,
        AnyProtocol,
        SecureProtocols,

        TlsV1_2OrLater = 7,

        DtlsV1_2 = 10,
        DtlsV1_2OrLater,

        TlsV1_3,
        TlsV1_3OrLater,

        UnknownProtocol = -1,
    }

    extern "C++" {
        include!("cxx-qt-io/qssl.h");
        type SslProtocol;
    }
}

pub use ffi::SslProtocol;

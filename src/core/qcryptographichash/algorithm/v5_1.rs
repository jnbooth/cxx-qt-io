#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum QCryptographicHashAlgorithm {
        Md4,
        Md5,
        Sha1,
        Sha224,
        Sha256,
        Sha384,
        Sha512,
        Sha3_224,
        Sha3_256,
        Sha3_384,
        Sha3_512,
    }

    extern "C++" {
        include!("cxx-qt-io/qcryptographichash.h");
        type QCryptographicHashAlgorithm;
    }
}

pub use ffi::QCryptographicHashAlgorithm;

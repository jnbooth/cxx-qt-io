#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum CryptographicHashAlgorithm {
        Md4,
        Md5,
        Sha1,
        Sha224,
        Sha256,
        Sha384,
        Sha512,
        Keccak_224,
        Keccak_256,
        Keccak_384,
        Keccak_512,
        Sha3_224,
        Sha3_256,
        Sha3_384,
        Sha3_512,
    }

    extern "C++" {
        include!("cxx-qt-io/qcryptographichash.h");
        type CryptographicHashAlgorithm;
    }
}

pub use ffi::CryptographicHashAlgorithm;

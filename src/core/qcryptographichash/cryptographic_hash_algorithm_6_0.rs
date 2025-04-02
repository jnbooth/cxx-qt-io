#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum QCryptographicHashAlgorithm {
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
        Blake2b_160,
        Blake2b_256,
        Blake2b_384,
        Blake2b_512,
        Blake2s_128,
        Blake2s_160,
        Blake2s_224,
        Blake2s_256,
    }

    extern "C++" {
        include!("cxx-qt-io/qcryptographichash.h");
        type QCryptographicHashAlgorithm;
    }
}

pub use ffi::QCryptographicHashAlgorithm;

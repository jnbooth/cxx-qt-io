use std::fmt;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    #[namespace = "rust::cxxqtio1"]
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

    #[namespace = "rust::cxxqtio1"]
    extern "C++" {
        include!("cxx-qt-io/qcryptographichash.h");
        type QCryptographicHashAlgorithm;
    }
}

pub use ffi::QCryptographicHashAlgorithm;

impl fmt::Display for QCryptographicHashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::Md4 => "MD4",
            Self::Md5 => "MD5",
            Self::Sha1 => "SHA-1",
            Self::Sha224 => "SHA-224",
            Self::Sha256 => "SHA-256",
            Self::Sha384 => "SHA-384",
            Self::Sha512 => "SHA-512",
            Self::Keccak_224 => "Keccak-224",
            Self::Keccak_256 => "Keccak-256",
            Self::Keccak_384 => "Keccak-384",
            Self::Keccak_512 => "Keccak-512",
            Self::Sha3_224 => "SHA3-224",
            Self::Sha3_256 => "SHA3-256",
            Self::Sha3_384 => "SHA3-384",
            Self::Sha3_512 => "SHA3-512",
            Self::Blake2b_160 => "BLAKE2b-160",
            Self::Blake2b_256 => "BLAKE2b-256",
            Self::Blake2b_384 => "BLAKE2b-384",
            Self::Blake2b_512 => "BLAKE2b-512",
            Self::Blake2s_128 => "BLAKE2s-128",
            Self::Blake2s_160 => "BLAKE2s-160",
            Self::Blake2s_224 => "BLAKE2s-224",
            Self::Blake2s_256 => "BLAKE2s-256",
            _ => "unknown",
        })
    }
}

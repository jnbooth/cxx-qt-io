use cxx::{ExternType, type_id};
use cxx_qt_lib::QByteArray;

use crate::QCryptographicHashAlgorithm;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qdtlsgeneratorparameters.h");
        #[allow(unused)]
        type QDtlsGeneratorParameters = super::QDtlsGeneratorParameters;
    }
}

/// This class defines parameters for DTLS cookie generator.
///
/// Qt Documentation: [QDtlsClientVerifier::GeneratorParameters](https://doc.qt.io/qt-6/qdtlsclientverifier-generatorparameters.html#details)
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct QDtlsGeneratorParameters {
    pub hash: QCryptographicHashAlgorithm,
    pub secret: QByteArray,
}

impl QDtlsGeneratorParameters {
    /// Constructs `QDtlsGeneratorParameters` object from `algorithm` and `secret`.
    ///
    /// **Note:** all fields of `QDtlsGeneratorParameters` are public, so you can also construct one directly.
    pub fn new(algorithm: QCryptographicHashAlgorithm, secret: &QByteArray) -> Self {
        Self {
            hash: algorithm,
            secret: secret.clone(),
        }
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QDtlsGeneratorParameters {
    type Id = type_id!("QDtlsGeneratorParameters");
    type Kind = cxx::kind::Trivial;
}

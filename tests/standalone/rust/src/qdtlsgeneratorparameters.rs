#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
    }

    extern "C++" {
        include!("cxx-qt-io/qcryptographichash.h");
        #[namespace = "rust::cxxqtio1"]
        type QCryptographicHashAlgorithm = cxx_qt_io::QCryptographicHashAlgorithm;
    }

    extern "C++" {
        include!("cxx-qt-io/qdtlsgeneratorparameters.h");
        type QDtlsGeneratorParameters = cxx_qt_io::QDtlsGeneratorParameters;
    }

    #[namespace = "ffi::qdtlsgeneratorparams"]
    extern "Rust" {
        fn construct(
            hash: QCryptographicHashAlgorithm,
            secret: &QByteArray,
        ) -> QDtlsGeneratorParameters;
        fn get_hash(params: &QDtlsGeneratorParameters) -> QCryptographicHashAlgorithm;
        fn get_secret(params: &QDtlsGeneratorParameters) -> QByteArray;
    }
}

use ffi::{QByteArray, QCryptographicHashAlgorithm, QDtlsGeneratorParameters};

fn construct(hash: QCryptographicHashAlgorithm, secret: &QByteArray) -> QDtlsGeneratorParameters {
    QDtlsGeneratorParameters {
        hash,
        secret: secret.clone(),
    }
}

fn get_hash(params: &QDtlsGeneratorParameters) -> QCryptographicHashAlgorithm {
    params.hash
}

fn get_secret(params: &QDtlsGeneratorParameters) -> QByteArray {
    params.secret.clone()
}

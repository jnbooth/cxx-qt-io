use cxx::UniquePtr;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++" {
        include!("cxx-qt-io/qfile.h");
        type QFile = cxx_qt_io::QFile;
    }

    #[namespace = "ffi::qfile"]
    extern "Rust" {
        fn construct(name: &QString) -> UniquePtr<QFile>;
    }
}

use ffi::{QFile, QString};

fn construct(name: &QString) -> UniquePtr<QFile> {
    QFile::new(name)
}

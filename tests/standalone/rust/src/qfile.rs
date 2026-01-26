use cxx::UniquePtr;

#[cxx::bridge]
mod qdir_cxx {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++" {
        include!("cxx-qt-io/qfile.h");
        type QFile = cxx_qt_io::QFile;
    }

    extern "Rust" {
        fn construct_qfile(name: &QString) -> UniquePtr<QFile>;
    }
}

use qdir_cxx::{QFile, QString};

fn construct_qfile(name: &QString) -> UniquePtr<QFile> {
    QFile::new(name)
}

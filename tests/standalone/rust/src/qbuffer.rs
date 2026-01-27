use std::io::{Read, Write};
use std::pin::Pin;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++" {
        include!("cxx-qt-io/qbuffer.h");
        type QBuffer = cxx_qt_io::QBuffer;
    }

    #[namespace = "ffi::qbuffer"]
    extern "Rust" {
        fn read(buf: Pin<&mut QBuffer>) -> QString;
        fn write(buf: Pin<&mut QBuffer>);
    }
}

use ffi::{QBuffer, QString};

fn read(mut buf: Pin<&mut QBuffer>) -> QString {
    let mut text = String::new();
    buf.read_to_string(&mut text).unwrap();
    text.into()
}

fn write(mut buf: Pin<&mut QBuffer>) {
    write!(buf, "Test string").unwrap();
}

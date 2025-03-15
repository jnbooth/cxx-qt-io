use std::pin::Pin;

use crate::adapter::{QIOAdaptable, QIOAdapter};

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!(<QtNetwork/QAbstractSocket>);

        type QIODevice = crate::QIODevice;

        #[qobject]
        #[base = QIODevice]
        type QAbstractSocket;

        fn flush(self: Pin<&mut QAbstractSocket>) -> bool;
    }
}

pub use ffi::QAbstractSocket;

impl QIOAdaptable for QAbstractSocket {
    fn flush(device: Pin<&mut Self>) -> bool {
        device.flush()
    }
}

impl QAbstractSocket {
    pub fn adapter(self: Pin<&mut Self>) -> QIOAdapter<Self> {
        QIOAdapter::new(self)
    }
}

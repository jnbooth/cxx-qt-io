use std::pin::Pin;

use crate::adapter::{QIOAdaptable, QIOAdapter};

#[cxx_qt::bridge]
mod ffi {
    #[auto_rust_name]
    unsafe extern "C++Qt" {
        include!(<QtCore/QFileDevice>);

        type QIODevice = crate::QIODevice;

        #[qobject]
        #[base = QIODevice]
        type QFileDevice;

        fn flush(self: Pin<&mut QFileDevice>) -> bool;
    }
}

pub use ffi::QFileDevice;

impl QIOAdaptable for QFileDevice {
    fn flush(device: Pin<&mut Self>) -> bool {
        device.flush()
    }
}

impl QFileDevice {
    pub fn as_io(self: Pin<&mut Self>) -> QIOAdapter<Self> {
        QIOAdapter::new(self)
    }
}

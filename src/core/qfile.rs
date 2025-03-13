use std::pin::Pin;

use crate::adapter::QIOAdapter;
use crate::QFileDevice;

#[cxx_qt::bridge]
mod ffi {
    #[auto_rust_name]
    unsafe extern "C++Qt" {
        include!(<QtCore/QFile>);

        type QFileDevice = crate::QFileDevice;

        #[qobject]
        #[base = QFileDevice]
        type QFile;
    }
}

use cxx_qt::Upcast;
pub use ffi::QFile;

impl QFile {
    pub fn as_io(self: Pin<&mut Self>) -> QIOAdapter<QFileDevice> {
        QIOAdapter::new(self.upcast_pin())
    }
}

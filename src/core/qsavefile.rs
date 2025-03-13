use std::pin::Pin;

use crate::adapter::QIOAdapter;
use crate::QFileDevice;

#[cxx_qt::bridge]
mod ffi {
    #[auto_rust_name]
    unsafe extern "C++Qt" {
        include!(<QtCore/QSaveFile>);

        type QFileDevice = crate::QFileDevice;

        #[qobject]
        #[base = QFileDevice]
        type QSaveFile;
    }
}

use cxx_qt::Upcast;
pub use ffi::QSaveFile;

impl QSaveFile {
    pub fn as_io(self: Pin<&mut Self>) -> QIOAdapter<QFileDevice> {
        QIOAdapter::new(self.upcast_pin())
    }
}

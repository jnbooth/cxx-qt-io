use crate::adapter::{QIOExt, QIO};
use crate::QIODevice;
use cxx_qt::Upcast;
use std::io::{self, Read, Write};
use std::pin::Pin;

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

impl QIO for QAbstractSocket {
    fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    fn as_io_device_mut(self: Pin<&mut Self>) -> Pin<&mut QIODevice> {
        self.upcast_pin()
    }

    fn flush(self: Pin<&mut Self>) -> bool {
        self.flush()
    }

    fn get_error_kind(&self) -> io::ErrorKind {
        std::io::ErrorKind::Other
    }
}

impl Read for Pin<&mut QAbstractSocket> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QIOExt::read(self.as_mut(), buf)
    }
}

impl Write for Pin<&mut QAbstractSocket> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        QIOExt::write(self.as_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        QIOExt::flush(self.as_mut())
    }
}

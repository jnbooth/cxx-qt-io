use crate::qio::{QIOExt, QIO};
use crate::{QAbstractSocket, QIODevice};
use cxx_qt::Upcast;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++Qt" {
        include!(<QtNetwork/QTcpSocket>);
        type QIODevice = crate::QIODevice;
        type QAbstractSocket = crate::QAbstractSocket;

        #[qobject]
        #[base = QAbstractSocket]
        type QTcpSocket;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        include!("cxx-qt-io/common.h");

        #[doc(hidden)]
        #[rust_name = "upcast_qtcpsocket_qiodevice"]
        unsafe fn upcast(socket: *const QTcpSocket) -> *const QIODevice;
        #[doc(hidden)]
        #[rust_name = "downcast_qiodevice_qtcpsocket"]
        unsafe fn downcast(socket: *const QIODevice) -> *const QTcpSocket;
    }
}

pub use ffi::QTcpSocket;

impl QTcpSocket {
    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    pub fn as_io_device_mut(self: Pin<&mut Self>) -> Pin<&mut QIODevice> {
        self.upcast_pin()
    }

    pub fn as_abstract_socket(&self) -> &QAbstractSocket {
        self.upcast()
    }

    pub fn as_abstract_socket_mut(self: Pin<&mut Self>) -> Pin<&mut QAbstractSocket> {
        self.upcast_pin()
    }
}

impl Deref for QTcpSocket {
    type Target = QAbstractSocket;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl Upcast<QIODevice> for QTcpSocket {
    unsafe fn upcast_ptr(this: *const Self) -> *const QIODevice {
        ffi::upcast_qtcpsocket_qiodevice(this)
    }

    unsafe fn from_base_ptr(base: *const QIODevice) -> *const Self {
        ffi::downcast_qiodevice_qtcpsocket(base)
    }
}

impl QIO for QTcpSocket {
    fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    fn as_io_device_mut(self: Pin<&mut Self>) -> Pin<&mut QIODevice> {
        self.upcast_pin()
    }

    fn flush(self: Pin<&mut Self>) -> bool {
        self.as_abstract_socket_mut().flush()
    }

    fn get_error_kind(&self) -> io::ErrorKind {
        self.as_abstract_socket().get_error_kind()
    }
}

impl Read for Pin<&mut QTcpSocket> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QIOExt::read(self.as_mut(), buf)
    }
}

impl Write for Pin<&mut QTcpSocket> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        QIOExt::write(self.as_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        QIOExt::flush(self.as_mut())
    }
}

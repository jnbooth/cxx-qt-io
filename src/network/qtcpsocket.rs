use crate::{QAbstractSocket, QIODevice};
use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt::QObject;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qtcpsocket.h");
        type QIODevice = crate::QIODevice;
        type QAbstractSocket = crate::QAbstractSocket;
    }

    unsafe extern "C++Qt" {
        /// The `QTcpSocket` class provides a TCP socket.
        ///
        /// Qt Documentation: [QTcpSocket](https://doc.qt.io/qt-6/qtcpsocket.html#details)
        #[qobject]
        #[base = QAbstractSocket]
        type QTcpSocket;
    }

    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/casting.h");

        #[rust_name = "upcast_qtcpsocket_qobject"]
        unsafe fn upcastPtr(socket: *const QTcpSocket) -> *const QObject;
        #[rust_name = "downcast_qobject_qtcpsocket"]
        unsafe fn downcastPtr(socket: *const QObject) -> *const QTcpSocket;

        #[rust_name = "upcast_qtcpsocket_qiodevice"]
        unsafe fn upcastPtr(socket: *const QTcpSocket) -> *const QIODevice;
        #[rust_name = "downcast_qiodevice_qtcpsocket"]
        unsafe fn downcastPtr(socket: *const QIODevice) -> *const QTcpSocket;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qtcpsocket_init_default"]
        fn make_unique() -> UniquePtr<QTcpSocket>;
    }
}

pub use ffi::QTcpSocket;

impl QTcpSocket {
    /// Creates a `QTcpSocket` object in state [`QAbstractSocketSocketState::UnconnectedState`](crate::QAbstractSocketSocketState::UnconnectedState).
    pub fn new() -> UniquePtr<Self> {
        ffi::qtcpsocket_init_default()
    }

    /// Casts this object to `QIODevice`.
    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    /// Mutably casts this object to `QIODevice`.
    pub fn as_io_device_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QIODevice> {
        self.as_mut().upcast_pin()
    }

    /// Casts this object to `QAbstractSocket`.
    pub fn as_abstract_socket(&self) -> &QAbstractSocket {
        self.upcast()
    }

    /// Mutably casts this object to `QAbstractSocket`.
    pub fn as_abstract_socket_mut<'a>(
        self: &'a mut Pin<&mut Self>,
    ) -> Pin<&'a mut QAbstractSocket> {
        self.as_mut().upcast_pin()
    }
}

impl Deref for QTcpSocket {
    type Target = QAbstractSocket;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl AsRef<QAbstractSocket> for QTcpSocket {
    fn as_ref(&self) -> &QAbstractSocket {
        self.upcast()
    }
}

unsafe impl Upcast<QIODevice> for QTcpSocket {
    unsafe fn upcast_ptr(this: *const Self) -> *const QIODevice {
        ffi::upcast_qtcpsocket_qiodevice(this)
    }

    unsafe fn from_base_ptr(base: *const QIODevice) -> *const Self {
        ffi::downcast_qiodevice_qtcpsocket(base)
    }
}

impl AsRef<QIODevice> for QTcpSocket {
    fn as_ref(&self) -> &QIODevice {
        self.upcast()
    }
}

unsafe impl Upcast<QObject> for QTcpSocket {
    unsafe fn upcast_ptr(this: *const Self) -> *const QObject {
        ffi::upcast_qtcpsocket_qobject(this)
    }

    unsafe fn from_base_ptr(base: *const QObject) -> *const Self {
        ffi::downcast_qobject_qtcpsocket(base)
    }
}

impl AsRef<QObject> for QTcpSocket {
    fn as_ref(&self) -> &QObject {
        self.upcast()
    }
}

impl Read for Pin<&mut QTcpSocket> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_io_device_mut().read(buf)
    }
}

impl Write for Pin<&mut QTcpSocket> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_io_device_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.as_abstract_socket_mut().flush();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cxx_qt_lib::QString;

    use crate::{QAbstractSocketPauseMode, QAbstractSocketPauseModes};

    use super::*;

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QTcpSocketProps {
            pause_mode: i32,
            protocol_tag: QString,
            read_buffer_size: i64,
        }

        let props = QTcpSocketProps {
            pause_mode: QAbstractSocketPauseMode::PauseOnSslErrors.repr,
            protocol_tag: QString::from("xmpp"),
            read_buffer_size: 1024 * 6,
        };

        let mut socket_ptr = QTcpSocket::new();
        let mut socket = socket_ptr.pin_mut();
        unsafe {
            socket
                .as_abstract_socket_mut()
                .set_pause_mode(QAbstractSocketPauseModes::from_int(props.pause_mode));
        }
        socket
            .as_abstract_socket_mut()
            .set_protocol_tag(&props.protocol_tag);
        socket
            .as_abstract_socket_mut()
            .set_read_buffer_size(props.read_buffer_size);

        let actual_props = QTcpSocketProps {
            pause_mode: socket.pause_mode().to_int(),
            protocol_tag: socket.protocol_tag(),
            read_buffer_size: socket.read_buffer_size(),
        };

        assert_eq!(actual_props, props);
    }
}

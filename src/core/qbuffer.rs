use crate::qio::{QIOExt, QIO};
use crate::QIODevice;
use cxx_qt::Upcast;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
    }

    unsafe extern "C++Qt" {
        include!(<QtCore/QBuffer>);
        type QIODevice = crate::QIODevice;

        /// The `QBuffer` class provides a `QIODevice` interface for a `QByteArray`.
        ///
        /// Qt Documentation: [QBuffer](https://doc.qt.io/qt-6/qbuffer.html#details)
        #[qobject]
        #[base = QIODevice]
        type QBuffer;

        /// Returns a mutable reference to the `QBuffer`'s internal buffer. You can use it to modify the `QByteArray` behind the `QBuffer`'s back.
        #[rust_name = "buffer_mut"]
        fn buffer(self: Pin<&mut QBuffer>) -> &mut QByteArray;

        /// Returns a reference to the `QBuffer`'s internal buffer.
        fn buffer(self: &QBuffer) -> &QByteArray;

        /// Returns the data contained in the buffer.
        ///
        /// This is the same as `buffer()`.
        fn data(self: &QBuffer) -> &QByteArray;

        /// Makes `QBuffer` use the `QByteArray` pointed to by `byte_array` as its internal buffer. QBuffer` doesn't take ownership of the QByteArray.
        ///
        /// Does nothing if `is_open()` is `true`.
        ///
        /// If you open the buffer in write-only mode or read-write mode and write something into the `QBuffer`, `byte_array` will be modified.
        ///
        /// If `byte_array` is a null pointer, the buffer creates its own internal `QByteArray` to work on. This byte array is initially empty.
        ///
        /// # Safety
        ///
        /// The caller is responsible for ensuring that `byte_array` remains valid until the `QBuffer` is destroyed, or until `set_buffer()` is called to change the buffer.
        #[rust_name = "set_buffer"]
        unsafe fn setBuffer(self: Pin<&mut QBuffer>, byte_array: *mut QByteArray);
    }
}

pub use ffi::QBuffer;

impl QBuffer {
    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    pub fn as_io_device_mut(self: Pin<&mut Self>) -> Pin<&mut QIODevice> {
        self.upcast_pin()
    }

    pub fn as_slice(&self) -> &[u8] {
        self.buffer().as_slice()
    }

    pub fn as_mut_slice(self: Pin<&mut Self>) -> &mut [u8] {
        self.buffer_mut().as_mut_slice()
    }
}

impl AsRef<[u8]> for QBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Deref for QBuffer {
    type Target = QIODevice;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl QIO for QBuffer {}

impl Read for Pin<&mut QBuffer> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QIOExt::read(self.as_mut(), buf)
    }
}

impl Write for Pin<&mut QBuffer> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        QIOExt::write(self.as_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        QIOExt::flush(self.as_mut())
    }
}

use crate::qio::{QIOExt, QIO};
use crate::QIODevice;
use cxx::UniquePtr;
use cxx_qt::Upcast;
use cxx_qt_lib::QByteArray;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
    }

    extern "C++" {
        include!("cxx-qt-io/qbuffer.h");
    }

    unsafe extern "C++Qt" {
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

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qbuffer_set_data"]
        fn qbufferSetData(buffer: Pin<&mut QBuffer>, data: &[u8]);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qbuffer_default"]
        fn make_unique() -> UniquePtr<QBuffer>;
        #[rust_name = "qbuffer_new"]
        unsafe fn make_unique(byte_array: *mut QByteArray) -> UniquePtr<QBuffer>;
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

    /// Constructs an empty buffer with the given parent. You can call `set_data()` to fill the buffer with data, or you can open it in write mode and use `write()`.
    pub fn new() -> UniquePtr<Self> {
        ffi::qbuffer_default()
    }

    /// Constructs a `QBuffer` that uses the `QByteArray` pointed to by `byte_array` as its internal buffer, and with the given parent. `QBuffer` doesn't take ownership of the `QByteArray`.
    ///
    /// If you open the buffer in write-only mode or read-write mode and write something into the `QBuffer`, `byte_array` will be modified.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring that `byte_array` remains valid until the `QBuffer` is destroyed, or until `set_buffer()` is called to change the buffer.
    pub unsafe fn for_array(byte_array: *mut QByteArray) -> UniquePtr<Self> {
        unsafe { ffi::qbuffer_new(byte_array) }
    }

    /// Sets the contents of the internal buffer to be `data`. This is the same as assigning data to `buffer()`.
    pub fn set_data<T: AsRef<[u8]>>(self: Pin<&mut Self>, data: T) {
        ffi::qbuffer_set_data(self, data.as_ref());
    }
}

impl Deref for QBuffer {
    type Target = QIODevice;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl AsRef<[u8]> for QBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
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

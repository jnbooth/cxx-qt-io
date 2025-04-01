use crate::qio::{QIOExt, QIO};
use crate::{QFileDevice, QIODevice};
use cxx::UniquePtr;
use cxx_qt::Upcast;
use cxx_qt_lib::QString;
use std::io::{self, Read, Write};
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_rust_name]
    unsafe extern "C++Qt" {
        include!(<QtCore/QSaveFile>);
        type QIODevice = crate::QIODevice;
        type QFileDevice = crate::QFileDevice;

        /// The `QSaveFile` class provides an interface for safely writing to files.
        ///
        /// Qt Documentation: [QSaveFile](https://doc.qt.io/qt-6/qsavefile.html#details)
        #[qobject]
        #[base = QFileDevice]
        type QSaveFile;

        /// Cancels writing the new file.
        ///
        /// If the application changes its mind while saving, it can call `cancel_writing()`, which sets an error code so that `commit()` will discard the temporary file.
        ///
        /// Alternatively, it can simply make sure not to call `commit()`.
        ///
        /// Further write operations are possible after calling this method, but none of it will have any effect, the written file will be discarded.
        ///
        /// This method has no effect when direct write fallback is used. This is the case when saving over an existing file in a readonly directory: no temporary file can be created, so the existing file is overwritten no matter what, and `cancel_writing()` cannot do anything about that, the contents of the existing file will be lost.
        #[rust_name = "cancel_writing"]
        fn cancelWriting(self: Pin<&mut QSaveFile>);

        /// Commits the changes to disk, if all previous writes were successful.
        ///
        /// It is mandatory to call this at the end of the saving operation, otherwise the file will be discarded.
        ///
        /// If an error happened during writing, deletes the temporary file and returns `false`. Otherwise, renames it to the final `file_name` and returns `true` on success. Finally, closes the device.
        fn commit(self: Pin<&mut QSaveFile>) -> bool;

        /// Returns `true` if the fallback solution for saving files in read-only directories is enabled.
        #[rust_name = "direct_write_fallback"]
        fn directWriteFallback(self: &QSaveFile) -> bool;

        /// Allows writing over the existing file if necessary.
        ///
        /// `QSaveFile` creates a temporary file in the same directory as the final file and atomically renames it. However this is not possible if the directory permissions do not allow creating new files. In order to preserve atomicity guarantees, `open()` fails when it cannot create the temporary file.
        ///
        /// In order to allow users to edit files with write permissions in a directory with restricted permissions, call `set_direct_write_fallback()` with `enabled` set to `true`, and the following calls to `open()` will fallback to opening the existing file directly and writing into it, without the use of a temporary file. This does not have atomicity guarantees, i.e. an application crash or for instance a power failure could lead to a partially-written file on disk. It also means `cancel_writing()` has no effect, in such a case.
        ///
        /// Typically, to save documents edited by the user, call `set_direct_write_fallback(true)`, and to save application internal files (configuration files, data files, ...), keep the default setting which ensures atomicity.
        #[rust_name = "set_direct_write_fallback"]
        fn setDirectWriteFallback(self: Pin<&mut QSaveFile>, enabled: bool);

        /// Sets the `name` of the file. The name can have no path, a relative path, or an absolute path.
        #[rust_name = "set_file_name"]
        fn setFileName(self: Pin<&mut QSaveFile>, name: &QString);
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        include!("cxx-qt-io/common.h");

        #[doc(hidden)]
        #[rust_name = "upcast_qsavefile_qiodevice"]
        unsafe fn upcast(file: *const QSaveFile) -> *const QIODevice;
        #[doc(hidden)]
        #[rust_name = "downcast_qiodevice_qsavefile"]
        unsafe fn downcast(file: *const QIODevice) -> *const QSaveFile;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qsavefile_new"]
        fn make_unique(path: &QString) -> UniquePtr<QSaveFile>;
    }
}

pub use ffi::QSaveFile;

impl QSaveFile {
    pub fn new(path: &QString) -> UniquePtr<Self> {
        ffi::qsavefile_new(path)
    }

    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    pub fn as_io_device_mut(self: Pin<&mut Self>) -> Pin<&mut QIODevice> {
        self.upcast_pin()
    }

    pub fn as_file_device(&self) -> &QFileDevice {
        self.upcast()
    }

    pub fn as_file_device_mut(self: Pin<&mut Self>) -> Pin<&mut QFileDevice> {
        self.upcast_pin()
    }
}

impl Upcast<QIODevice> for QSaveFile {
    unsafe fn upcast_ptr(this: *const Self) -> *const QIODevice {
        ffi::upcast_qsavefile_qiodevice(this)
    }

    unsafe fn from_base_ptr(base: *const QIODevice) -> *const Self {
        ffi::downcast_qiodevice_qsavefile(base)
    }
}

impl QIO for QSaveFile {
    fn flush(self: Pin<&mut Self>) -> bool {
        self.as_file_device_mut().flush()
    }

    fn get_error_kind(&self) -> io::ErrorKind {
        self.as_file_device().get_error_kind()
    }
}

impl Read for Pin<&mut QSaveFile> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QIOExt::read(self.as_mut(), buf)
    }
}

impl Write for Pin<&mut QSaveFile> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        QIOExt::write(self.as_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        QIOExt::flush(self.as_mut())
    }
}

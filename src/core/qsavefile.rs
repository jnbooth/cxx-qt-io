use crate::{QFileDevice, QIODevice};
use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt::QObject;
use cxx_qt_lib::QString;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++" {
        include!("cxx-qt-io/qsavefile.h");
        type QIODevice = crate::QIODevice;
        type QFileDevice = crate::QFileDevice;
    }

    unsafe extern "C++Qt" {
        /// The `QSaveFile` class provides an interface for safely writing to files.
        ///
        /// Qt Documentation: [QSaveFile](https://doc.qt.io/qt-6/qsavefile.html#details)
        #[qobject]
        #[base = QFileDevice]
        type QSaveFile;

        /// Cancels writing the new file.
        ///
        /// If the application changes its mind while saving, it can call this function, which sets an error code so that [`commit`](QSaveFile::commit) will discard the temporary file.
        ///
        /// Alternatively, it can simply make sure not to call [`commit`](QSaveFile::commit).
        ///
        /// Further write operations are possible after calling this method, but none of it will have any effect, the written file will be discarded.
        ///
        /// This method has no effect when direct write fallback is used. This is the case when saving over an existing file in a readonly directory: no temporary file can be created, so the existing file is overwritten no matter what, and this function cannot do anything about that, the contents of the existing file will be lost.
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
        /// `QSaveFile` creates a temporary file in the same directory as the final file and atomically renames it. However this is not possible if the directory permissions do not allow creating new files. In order to preserve atomicity guarantees, [`open`](QIODevice::open) fails when it cannot create the temporary file.
        ///
        /// In order to allow users to edit files with write permissions in a directory with restricted permissions, call this function with `enabled` set to `true`, and the following calls to [`open`](QIODevice::open) will fallback to opening the existing file directly and writing into it, without the use of a temporary file. This does not have atomicity guarantees, i.e. an application crash or for instance a power failure could lead to a partially-written file on disk. It also means [`cancel_writing`](QSaveFile::cancel_writing) has no effect, in such a case.
        ///
        /// Typically, to save documents edited by the user, call `self.set_direct_write_fallback(true)`, and to save application internal files (configuration files, data files, ...), keep the default setting which ensures atomicity.
        #[rust_name = "set_direct_write_fallback"]
        fn setDirectWriteFallback(self: Pin<&mut QSaveFile>, enabled: bool);

        /// Sets the `name` of the file. The name can have no path, a relative path, or an absolute path.
        #[rust_name = "set_file_name"]
        fn setFileName(self: Pin<&mut QSaveFile>, name: &QString);
    }

    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/casting.h");

        #[rust_name = "upcast_qsavefile_qobject"]
        unsafe fn upcastPtr(file: *const QSaveFile) -> *const QObject;
        #[rust_name = "downcast_qobject_qsavefile"]
        unsafe fn downcastPtr(file: *const QObject) -> *const QSaveFile;

        #[rust_name = "upcast_qsavefile_qiodevice"]
        unsafe fn upcastPtr(file: *const QSaveFile) -> *const QIODevice;
        #[rust_name = "downcast_qiodevice_qsavefile"]
        unsafe fn downcastPtr(file: *const QIODevice) -> *const QSaveFile;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsavefile_init_default"]
        fn make_unique() -> UniquePtr<QSaveFile>;
        #[rust_name = "qsavefile_new"]
        fn make_unique(path: &QString) -> UniquePtr<QSaveFile>;
    }
}

pub use ffi::QSaveFile;

impl QSaveFile {
    /// Constructs a new file object to represent the file with the given `name`.
    pub fn new(path: &QString) -> UniquePtr<Self> {
        ffi::qsavefile_new(path)
    }

    /// Constructs a new file object. You need to call [`set_file_name`](QSaveFile::set_file_name) before [`open`](QIODevice::open).
    pub fn new_default() -> UniquePtr<Self> {
        ffi::qsavefile_init_default()
    }

    /// Casts this object to `QIODevice`.
    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }
    /// Mutably casts this object to `QIODevice`.
    pub fn as_io_device_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QIODevice> {
        self.as_mut().upcast_pin()
    }

    /// Casts this object to `QFileDevice`.
    pub fn as_file_device(&self) -> &QFileDevice {
        self.upcast()
    }

    /// Mutably casts this object to `QFileDevice`.
    pub fn as_file_device_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QFileDevice> {
        self.as_mut().upcast_pin()
    }
}

impl Deref for QSaveFile {
    type Target = QFileDevice;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl AsRef<QFileDevice> for QSaveFile {
    fn as_ref(&self) -> &QFileDevice {
        self.upcast()
    }
}

unsafe impl Upcast<QIODevice> for QSaveFile {
    unsafe fn upcast_ptr(this: *const Self) -> *const QIODevice {
        ffi::upcast_qsavefile_qiodevice(this)
    }

    unsafe fn from_base_ptr(base: *const QIODevice) -> *const Self {
        ffi::downcast_qiodevice_qsavefile(base)
    }
}

impl AsRef<QIODevice> for QSaveFile {
    fn as_ref(&self) -> &QIODevice {
        self.upcast()
    }
}

unsafe impl Upcast<QObject> for QSaveFile {
    unsafe fn upcast_ptr(this: *const Self) -> *const QObject {
        ffi::upcast_qsavefile_qobject(this)
    }

    unsafe fn from_base_ptr(base: *const QObject) -> *const Self {
        ffi::downcast_qobject_qsavefile(base)
    }
}

impl AsRef<QObject> for QSaveFile {
    fn as_ref(&self) -> &QObject {
        self.upcast()
    }
}

impl Read for Pin<&mut QSaveFile> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_io_device_mut().read(buf)
    }
}

impl Write for Pin<&mut QSaveFile> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_io_device_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.as_file_device_mut().flush();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let filename = QString::from("myfile.md");
        let file = QSaveFile::new(&filename);
        assert_eq!(file.file_name(), filename);
    }

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QSaveFileProps {
            direct_write_fallback: bool,
            file_name: QString,
        }

        let props = QSaveFileProps {
            direct_write_fallback: true,
            file_name: QString::from("myfile.txt"),
        };

        let mut file = QSaveFile::new_default();
        file.pin_mut()
            .set_direct_write_fallback(props.direct_write_fallback);
        file.pin_mut().set_file_name(&props.file_name);

        let actual_props = QSaveFileProps {
            direct_write_fallback: file.direct_write_fallback(),
            file_name: file.file_name(),
        };

        assert_eq!(actual_props, props);
    }
}

use std::fmt;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;

use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt::QObject;
use cxx_qt_lib::QString;

use crate::qobject::debug_qobject;
use crate::{QFile, QFileDevice, QIODevice};

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++" {
        include!("cxx-qt-io/qtemporaryfile.h");
        type QIODevice = crate::QIODevice;
        type QFileDevice = crate::QFileDevice;
        type QFile = crate::QFile;
    }

    unsafe extern "C++Qt" {
        /// The `QTemporaryFile` class is an I/O device that operates on temporary files.
        ///
        /// Qt Documentation: [QTemporaryFile](https://doc.qt.io/qt-6/qtemporaryfile.html#details)
        #[qobject]
        #[base = QFile]
        type QTemporaryFile;

        /// Returns `true` if the `QTemporaryFile` is in auto remove mode. Auto-remove mode will automatically delete the filename from disk upon destruction. This makes it very easy to create your `QTemporaryFile` object on the stack, fill it with data, read from it, and finally on function return it will automatically clean up after itself.
        ///
        /// Auto-remove is on by default.
        #[rust_name = "auto_remove"]
        fn autoRemove(self: &QTemporaryFile) -> bool;

        /// Returns the file name template.
        ///
        /// The file name template returned by this method, will be relative or absolute depending on the file name template used to construct this object (or passed to [`set_file_template`](QTemporaryFile::set_file_template)) being relative or absolute, respectively.
        #[rust_name = "file_template"]
        fn fileTemplate(self: &QTemporaryFile) -> QString;

        /// Opens a unique temporary file in the file system in [`QIODeviceOpenModeFlag::ReadWrite`](crate::QIODeviceOpenModeFlag::ReadWrite) mode. Returns `true` if the file was successfully opened, or was already open. Otherwise returns `false`.
        ///
        /// If called for the first time, this function will create a unique file name based on [`self.file_template()`](QTemporaryFile::file_template). The file is guaranteed to have been created by this function (that is, it has never existed before).
        ///
        /// If a file is reopened after calling [`close`](QIODevice::close), the same file will be opened again.
        fn open(self: Pin<&mut QTemporaryFile>) -> bool;

        /// Renames the current temporary file to `new_name` and returns `true` if it succeeded.
        ///
        /// This function has an important difference compared to [`QFile::rename`](QFile::rename): it will not perform a copy+delete if the low-level system call to rename the file fails, something that could happen if `new_name` specifies a file in a different volume or filesystem than the temporary file was created on. In other words, `QTemporaryFile` only supports atomic file renaming.
        ///
        /// This functionality is intended to support materializing the destination file with all contents already present, so another process cannot see an incomplete file in the process of being written. The [`QSaveFile`](crate::QSaveFile) class can be used for a similar purpose too, particularly if the destination file is not temporary.
        fn rename(self: Pin<&mut QTemporaryFile>, new_name: &QString) -> bool;

        /// Sets the `QTemporaryFile` into auto-remove mode if `b` is `true`.
        ///
        /// Auto-remove is on by default.
        ///
        /// If you set this property to `false`, ensure the application provides a way to remove the file once it is no longer needed, including passing the responsibility on to another process. Always use [`self.file_name()`](QFileDevice::file_name) to obtain the name and never try to guess the name that `QTemporaryFile` has generated.
        ///
        /// On some systems, if [`file_name`](QFileDevice::file_name) is not called before closing the file, the temporary file may be removed regardless of the state of this property. This behavior should not be relied upon, so application code should either call [`file_name`](QFileDevice::file_name) or leave the auto removal functionality enabled.
        #[rust_name = "set_auto_remove"]
        fn setAutoRemove(self: Pin<&mut QTemporaryFile>, b: bool);

        /// Sets the file name template to `template_name`.
        ///
        /// If the file name (the part after the last directory path separator in [`self.file_template()`](QTemporaryFile::file_template)) doesn't contain `"XXXXXX"`, it will be added automatically.
        ///
        /// `"XXXXXX"` will be replaced with the dynamic part of the file name, which is calculated to be unique.
        //
        /// If `template_name` is a relative path, the path will be relative to the current working directory. You can use [`QDir::temp_path()`](crate::QDir::temp_path) to construct `template_name` if you want use the system's temporary directory. It is important to specify the correct directory if the [`rename`](QTemporaryFile::rename) function will be called, as `QTemporaryFile` can only rename files within the same volume / filesystem as the temporary file itself was created on.
        #[rust_name = "set_file_template"]
        fn setFileTemplate(self: Pin<&mut QTemporaryFile>, template_name: &QString);
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qtemporaryfile_create_native_file"]
        fn qtemporaryfileCreateNativeFile(file: Pin<&mut QFile>) -> *mut QTemporaryFile;
    }

    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/casting.h");

        #[rust_name = "upcast_qtemporaryfile_qobject"]
        unsafe fn upcastPtr(file: *const QTemporaryFile) -> *const QObject;
        #[rust_name = "downcast_qobject_qtemporaryfile"]
        unsafe fn downcastPtr(file: *const QObject) -> *const QTemporaryFile;

        #[rust_name = "upcast_qtemporaryfile_qiodevice"]
        unsafe fn upcastPtr(file: *const QTemporaryFile) -> *const QIODevice;
        #[rust_name = "downcast_qiodevice_qtemporaryfile"]
        unsafe fn downcastPtr(file: *const QIODevice) -> *const QTemporaryFile;

        #[rust_name = "upcast_qtemporaryfile_qfiledevice"]
        unsafe fn upcastPtr(file: *const QTemporaryFile) -> *const QFileDevice;
        #[rust_name = "downcast_qfiledevice_qtemporaryfile"]
        unsafe fn downcastPtr(file: *const QFileDevice) -> *const QTemporaryFile;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qtemporaryfile_init_default"]
        fn make_unique() -> UniquePtr<QTemporaryFile>;
        #[rust_name = "qtemporaryfile_new"]
        fn make_unique(path: &QString) -> UniquePtr<QTemporaryFile>;
    }
}

pub use ffi::QTemporaryFile;

impl fmt::Debug for QTemporaryFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QTemporaryFile {
    /// Constructs a `QTemporaryFile` with `template_name` as the file name template.
    ///
    /// Upon opening the temporary file, `template_name` will be used to create a unique filename.
    ///
    /// If the file name (the part after the last directory path separator in `template_name`) doesn't contain `"XXXXXX"`, it will be added automatically.
    ///
    /// `"XXXXXX"` will be replaced with the dynamic part of the file name, which is calculated to be unique.
    ///
    /// If `template_name` is a relative path, the path will be relative to the current working directory. You can use [`QDir::temp_path()`](crate::QDir::temp_path) to construct `template_name` if you want use the system's temporary directory.
    ///
    /// It is important to specify the correct directory if the [rename](crate::QFile::rename) function will be called, as `QTemporaryFile` can only rename files within the same volume / filesystem as the temporary file itself was created on.
    pub fn new(template_name: &QString) -> UniquePtr<Self> {
        ffi::qtemporaryfile_new(template_name)
    }

    /// Constructs a `QTemporaryFile`.
    ///
    /// The default file name template is determined from the application name as returned by [`QCoreApplication::application_name()`](cxx_qt_lib::QCoreApplication::application_name) (or `"qt_temp"` if the application name is empty), followed by `".XXXXXX"`. The file is stored in the system's temporary directory, as returned by [`QDir::temp_path()`](crate::QDir::temp_path).
    pub fn new_default() -> UniquePtr<Self> {
        ffi::qtemporaryfile_init_default()
    }

    /// If `file` is not already a native file, then a `QTemporaryFile` is created in [`QDir::temp_path()`](crate::QDir::temp_path), the contents of file is copied into it, and a pointer to the temporary file is returned. Does nothing and returns a null pointer if file is already a native file.
    pub fn create_native_file(file: Pin<&mut QFile>) -> UniquePtr<Self> {
        // SAFETY: Qt returns a pointer that is either valid or null and is not owned.
        unsafe { UniquePtr::from_raw(ffi::qtemporaryfile_create_native_file(file)) }
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

    /// Casts this object to `QFile`.
    pub fn as_file(&self) -> &QFile {
        self.upcast()
    }

    /// Mutably casts this object to `QFile`.
    pub fn as_file_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QFile> {
        self.as_mut().upcast_pin()
    }
}

impl Deref for QTemporaryFile {
    type Target = QFile;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

unsafe impl Upcast<QFileDevice> for QTemporaryFile {
    unsafe fn upcast_ptr(this: *const Self) -> *const QFileDevice {
        ffi::upcast_qtemporaryfile_qfiledevice(this)
    }

    unsafe fn from_base_ptr(base: *const QFileDevice) -> *const Self {
        ffi::downcast_qfiledevice_qtemporaryfile(base)
    }
}

unsafe impl Upcast<QIODevice> for QTemporaryFile {
    unsafe fn upcast_ptr(this: *const Self) -> *const QIODevice {
        ffi::upcast_qtemporaryfile_qiodevice(this)
    }

    unsafe fn from_base_ptr(base: *const QIODevice) -> *const Self {
        ffi::downcast_qiodevice_qtemporaryfile(base)
    }
}

unsafe impl Upcast<QObject> for QTemporaryFile {
    unsafe fn upcast_ptr(this: *const Self) -> *const QObject {
        ffi::upcast_qtemporaryfile_qobject(this)
    }

    unsafe fn from_base_ptr(base: *const QObject) -> *const Self {
        ffi::downcast_qobject_qtemporaryfile(base)
    }
}

impl Read for Pin<&mut QTemporaryFile> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_io_device_mut().read(buf)
    }
}

impl Write for Pin<&mut QTemporaryFile> {
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
        let template_name = QString::from("template-XXXXXX");
        let file = QTemporaryFile::new(&template_name);
        assert_eq!(file.file_template(), template_name);
    }

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QTemporaryFileProps {
            auto_remove: bool,
            file_template: QString,
        }

        let props = QTemporaryFileProps {
            auto_remove: false,
            file_template: QString::from("mytemplate-XXXXXX"),
        };

        let mut file = QTemporaryFile::new(&QString::from("template-XXXXXX"));
        file.pin_mut().set_auto_remove(props.auto_remove);
        file.pin_mut().set_file_template(&props.file_template);

        let actual_props = QTemporaryFileProps {
            auto_remove: file.auto_remove(),
            file_template: file.file_template(),
        };

        assert_eq!(actual_props, props);
    }
}

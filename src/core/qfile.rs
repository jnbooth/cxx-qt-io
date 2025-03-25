use crate::adapter::{QIOExt, QIO};
use crate::{QFileDevice, QIODevice};
use cxx::UniquePtr;
use cxx_qt::Upcast;
use cxx_qt_lib::QString;
use std::io::{self, Read, Write};
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        include!("cxx-qt-io/common.h");

        #[doc(hidden)]
        #[rust_name = "qfile_new"]
        fn constructNew(path: &QString) -> UniquePtr<QFile>;

        #[doc(hidden)]
        #[rust_name = "upcast_qfile_qiodevice"]
        unsafe fn upcast(file: *const QFile) -> *const QIODevice;
        #[doc(hidden)]
        #[rust_name = "downcast_qiodevice_qfile"]
        unsafe fn downcast(file: *const QIODevice) -> *const QFile;
    }

    unsafe extern "C++Qt" {
        include!(<QtCore/QFile>);
        type QIODevice = crate::QIODevice;
        type QFileDevice = crate::QFileDevice;

        #[qobject]
        #[base = QFileDevice]
        type QFile;

        /// Copies the file named `file_name()` to `new_nme`.
        ///
        /// This file is closed before it is copied.
        ///
        /// If the copied file is a symbolic link (symlink), the file it refers to is copied, not the link itself. With the exception of permissions, which are copied, no other file metadata is copied.
        ///
        /// Returns `true` if successful; otherwise returns `false`.
        ///
        /// Note that if a file with the name `new_name` already exists, `copy()` returns false. This means `QFile` will not overwrite it.
        fn copy(self: Pin<&mut QFile>, new_name: &QString) -> bool;

        /// Returns `true` if the file specified by `file_name()` exists; otherwise returns `false`.
        fn exists(self: &QFile) -> bool;

        /// Creates a link named `link_name` that points to the file currently specified by `file_name()`. What a link is depends on the underlying filesystem (be it a shortcut on Windows or a symbolic link on Unix). Returns `true` if successful; otherwise returns `false`.
        ///
        /// This function will not overwrite an already existing entity in the file system; in this case, `link()` will return false and set `error()` to return `RenameError`.
        ///
        /// **Note:** To create a valid link on Windows, `link_nme` must have a ``.lnk` file extension.
        fn link(self: Pin<&mut QFile>, link_name: &QString) -> bool;

        /// Moves the file specified by `file_name()` to the trash. Returns `true` if successful, and sets the `file_name()` to the path at which the file can be found within the trash; otherwise returns `false`.
        ///
        /// The time for this function to run is independent of the size of the file being trashed. If this function is called on a directory, it may be proportional to the number of files being trashed. If the current `file_name()` points to a symbolic link, this function will move the link to the trash, possibly breaking it, not the target of the link.
        ///
        /// This function uses the Windows and macOS APIs to perform the trashing on those two operating systems. Elsewhere (Unix systems), this function implements the FreeDesktop.org Trash specification version 1.0.
        ///
        /// **Note:** When using the FreeDesktop.org Trash implementation, this function will fail if it is unable to move the files to the trash location by way of file renames and hardlinks. This condition arises if the file being trashed resides on a volume (mount point) on which the current user does not have permission to create the `.Trash` directory, or with some unusual filesystem types or configurations (such as sub-volumes that aren't themselves mount points).
        ///
        /// **Note:** On systems where the system API doesn't report the location of the file in the trash, `file_name()` will be set to the null string once the file has been moved. On systems that don't have a trash can, this function always returns `false`.
        #[rust_name = "move_to_trash"]
        fn moveToTrash(self: Pin<&mut QFile>) -> bool;

        /// Removes the file specified by `file_name()`. Returns true if successful; otherwise returns false.
        ///
        /// The file is closed before it is removed.
        fn remove(self: Pin<&mut QFile>) -> bool;

        /// Renames the file currently specified by `file_name()` to `new_name`. Returns `true` if successful; otherwise returns `false`.
        ///
        /// If a file with the name `new_name` already exists, `rename()` returns false (i.e., `QFile` will not overwrite it).
        ///
        /// The file is closed before it is renamed.
        ///
        /// If the rename operation fails, Qt will attempt to copy this file's contents to `new_name`, and then remove this file, keeping only `new_name`. If that copy operation fails or this file can't be removed, the destination file `new_name` is removed to restore the old state.
        fn rename(self: Pin<&mut QFile>, new_name: &QString) -> bool;

        /// Sets the `name` of the file. The name can have no path, a relative path, or an absolute path.
        ///
        /// Do not call this function if the file has already been opened.
        ///
        /// If the file name has no path or a relative path, the path used will be the application's current directory path at the time of the `open()` call.
        ///
        /// Note that the directory separator "/" works for all operating systems supported by Qt.
        #[rust_name = "set_file_name"]
        fn setFileName(self: Pin<&mut QFile>, name: &QString);

        /// Returns the absolute path of the file or directory a symlink (or shortcut on Windows) points to, or a an empty string if the object isn't a symbolic link.
        ///
        /// This name may not represent an existing file; it is only a string. `exists()` returns `true` if the symlink points to an existing file.
        #[rust_name = "sym_link_target"]
        fn symLinkTarget(self: &QFile) -> QString;
    }
}

pub use ffi::QFile;

impl QFile {
    pub fn new(path: &QString) -> UniquePtr<Self> {
        ffi::qfile_new(path)
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

impl Upcast<QIODevice> for QFile {
    unsafe fn upcast_ptr(this: *const Self) -> *const QIODevice {
        ffi::upcast_qfile_qiodevice(this)
    }

    unsafe fn from_base_ptr(base: *const QIODevice) -> *const Self {
        ffi::downcast_qiodevice_qfile(base)
    }
}

impl QIO for QFile {
    fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    fn as_io_device_mut(self: Pin<&mut Self>) -> Pin<&mut QIODevice> {
        self.upcast_pin()
    }

    fn flush(self: Pin<&mut Self>) -> bool {
        self.as_file_device_mut().flush()
    }

    fn get_error_kind(&self) -> io::ErrorKind {
        self.as_file_device().get_error_kind()
    }
}

impl Read for Pin<&mut QFile> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QIOExt::read(self.as_mut(), buf)
    }
}

impl Write for Pin<&mut QFile> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        QIOExt::write(self.as_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        QIOExt::flush(self.as_mut())
    }
}

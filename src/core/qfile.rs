use crate::{
    FileDescriptor, QFileDevice, QFileDeviceFileHandleFlags, QIODevice, QIODeviceOpenMode,
};
use cxx::UniquePtr;
use cxx_qt::{QObject, Upcast};
use cxx_qt_lib::{QByteArray, QString};
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-io/qfiledevice.h");
        type QFileDevice = crate::QFileDevice;
        type QFileDeviceFileHandleFlags = crate::QFileDeviceFileHandleFlags;
        include!("cxx-qt-io/qiodevice.h");
        type QIODevice = crate::QIODevice;
        type QIODeviceOpenMode = crate::QIODeviceOpenMode;
    }

    extern "C++" {
        include!("cxx-qt-io/qfile.h");
    }

    unsafe extern "C++Qt" {
        /// The `QFile` class provides an interface for reading from and writing to files.
        ///
        /// Qt Documentation: [QFile](https://doc.qt.io/qt-6/qfile.html#details)
        #[qobject]
        #[base = QFileDevice]
        type QFile;

        /// Copies the file named [`self.file_name()`](QFileDevice::file_name) to `new_name`.
        ///
        /// This file is closed before it is copied.
        ///
        /// If the copied file is a symbolic link (symlink), the file it refers to is copied, not the link itself. With the exception of permissions, which are copied, no other file metadata is copied.
        ///
        /// Returns `true` if successful; otherwise returns `false`.
        ///
        /// Note that if a file with the name `new_name` already exists, this function returns false. This means `QFile` will not overwrite it.
        fn copy(self: Pin<&mut QFile>, new_name: &QString) -> bool;

        /// Returns `true` if the file specified by [`self.file_name()`](QFileDevice::file_name) exists; otherwise returns `false`.
        fn exists(self: &QFile) -> bool;

        /// Creates a link named `link_name` that points to the file currently specified by [`self.file_name()`](QFileDevice::file_name). What a link is depends on the underlying filesystem (be it a shortcut on Windows or a symbolic link on Unix). Returns `true` if successful; otherwise returns `false`.
        ///
        /// This function will not overwrite an already existing entity in the file system; in this case, this function will return false and set [`self.error()`](QFileDevice::error) to return [`QFileDeviceFileError::RenameError`](crate::QFileDeviceFileError::RenameError).
        ///
        /// **Note:** To create a valid link on Windows, `link_name` must have a `.lnk` file extension.
        fn link(self: Pin<&mut QFile>, link_name: &QString) -> bool;

        /// Moves the file specified by [`self.file_name()`](QFileDevice::file_name) to the trash. Returns `true` if successful, and sets [`self.file_name()`](QFileDevice::file_name) to the path at which the file can be found within the trash; otherwise returns `false`.
        ///
        /// The time for this function to run is independent of the size of the file being trashed. If this function is called on a directory, it may be proportional to the number of files being trashed. If the current [`self.file_name()`](QFileDevice::file_name) points to a symbolic link, this function will move the link to the trash, possibly breaking it, not the target of the link.
        ///
        /// This function uses the Windows and macOS APIs to perform the trashing on those two operating systems. Elsewhere (Unix systems), this function implements the [FreeDesktop.org Trash specification version 1.0](https://specifications.freedesktop.org/trash-spec/1.0/).
        ///
        /// **Note:** When using the FreeDesktop.org Trash implementation, this function will fail if it is unable to move the files to the trash location by way of file renames and hardlinks. This condition arises if the file being trashed resides on a volume (mount point) on which the current user does not have permission to create the `.Trash` directory, or with some unusual filesystem types or configurations (such as sub-volumes that aren't themselves mount points).
        ///
        /// **Note:** On systems where the system API doesn't report the location of the file in the trash, [`self.file_name()`](QFileDevice::file_name) will be set to the null string once the file has been moved. On systems that don't have a trash can, this function always returns `false`.
        #[rust_name = "move_to_trash"]
        fn moveToTrash(self: Pin<&mut QFile>) -> bool;

        #[rust_name = "open_int"]
        fn open(
            self: Pin<&mut QFile>,
            fd: i32,
            mode: QIODeviceOpenMode,
            handle_flags: QFileDeviceFileHandleFlags,
        ) -> bool;

        /// Removes the file specified by [`self.file_name()`](QFileDevice::file_name). Returns `true` if successful; otherwise returns `false`.
        ///
        /// The file is closed before it is removed.
        fn remove(self: Pin<&mut QFile>) -> bool;

        /// Renames the file currently specified by [`self.file_name()`](QFileDevice::file_name) to `new_name`. Returns `true` if successful; otherwise returns `false`.
        ///
        /// If a file with the name `new_name` already exists, this function returns `false` (i.e., `QFile` will not overwrite it).
        ///
        /// The file is closed before it is renamed.
        ///
        /// If the rename operation fails, Qt will attempt to copy this file's contents to `new_name`, and then remove this file, keeping only `new_name`. If that copy operation fails or this file can't be removed, the destination file `new_name` is removed to restore the old state.
        fn rename(self: Pin<&mut QFile>, new_name: &QString) -> bool;

        /// Sets the `name` of the file. The name can have no path, a relative path, or an absolute path.
        ///
        /// Do not call this function if the file has already been opened.
        ///
        /// If the file name has no path or a relative path, the path used will be the application's current directory path at the time of the [`open`](QIODevice::open) call.
        ///
        /// Note that the directory separator "/" works for all operating systems supported by Qt.
        #[rust_name = "set_file_name"]
        fn setFileName(self: Pin<&mut QFile>, name: &QString);

        /// Returns the absolute path of the file or directory a symlink (or shortcut on Windows) points to, or a an empty string if the object isn't a symbolic link.
        ///
        /// This name may not represent an existing file; it is only a string. [`self.exists()`](QFile::exists) returns `true` if the symlink points to an existing file.
        #[rust_name = "sym_link_target"]
        fn symLinkTarget(self: &QFile) -> QString;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qfile_decode_name"]
        fn qfileDecodeName(name: &QByteArray) -> QString;

        #[rust_name = "qfile_encode_name"]
        fn qfileEncodeName(name: &QString) -> QByteArray;

        #[cfg(cxxqt_qt_version_at_least_6_9)]
        #[rust_name = "qfile_supports_move_to_trash"]
        fn qfileSupportsMoveToTrash() -> bool;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        include!("cxx-qt-io/common.h");

        #[rust_name = "upcast_qfile_qobject"]
        unsafe fn upcast(file: *const QFile) -> *const QObject;
        #[rust_name = "downcast_qobject_qfile"]
        unsafe fn downcast(file: *const QObject) -> *const QFile;

        #[rust_name = "upcast_qfile_qiodevice"]
        unsafe fn upcast(file: *const QFile) -> *const QIODevice;
        #[rust_name = "downcast_qiodevice_qfile"]
        unsafe fn downcast(file: *const QIODevice) -> *const QFile;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qfile_init_default"]
        fn make_unique() -> UniquePtr<QFile>;
        #[rust_name = "qfile_new"]
        fn make_unique(path: &QString) -> UniquePtr<QFile>;
    }
}

pub use ffi::QFile;

impl QFile {
    /// Constructs a new file object to represent the file with the given `name`.
    pub fn new(name: &QString) -> UniquePtr<Self> {
        ffi::qfile_new(name)
    }

    /// Constructs a `QFile` object.
    pub fn new_default() -> UniquePtr<Self> {
        ffi::qfile_init_default()
    }

    /// This does the reverse of [`encode_name`](QFile::encode_name) using `local_file_name`.
    pub fn decode_name(local_file_name: &QByteArray) -> QString {
        ffi::qfile_decode_name(local_file_name)
    }

    /// Converts `file_name` to an 8-bit encoding that you can use in native APIs. On Windows, the encoding is the one from active Windows (ANSI) codepage. On other platforms, this is UTF-8, for macOS in decomposed form (NFD).
    pub fn encode_name(file_name: &QString) -> QByteArray {
        ffi::qfile_encode_name(file_name)
    }

    /// Opens the existing file descriptor `fd` in the given `mode`. `handle_flags` may be used to specify additional options. Returns `true` if successful; otherwise returns `false`.
    ///
    /// When a `QFile` is opened using this function, behaviour of [`close`](QIODevice::close) is controlled by the [`QFileDeviceFileHandleFlag::AutoCloseHandle`](crate::QFileDeviceFileHandleFlag::AutoCloseHandle) flag. If [`QFileDeviceFileHandleFlag::AutoCloseHandle`](crate::QFileDeviceFileHandleFlag::AutoCloseHandle) is specified, and this function succeeds, then calling [`close`](QIODevice::close) closes the adopted handle. Otherwise, [`close`](QIODevice::close) does not actually close the file, but only flushes it.
    ///
    /// Warning: If `fd` is not a regular file, e.g, it is 0 ([`FileDescriptor::STDIN`]), 1 ([`FileDescriptor::STDOUT`]), or 2 ([`FileDescriptor::STDERR`]), you may not be able to use [`seek`](QIODevice::seek). In those cases, [`size`](QIODevice::size) returns 0. See [`is_sequential`](QIODevice::is_sequential) for more information.
    ///
    /// Warning: Since this function opens the file without specifying the file name, you cannot use this `QFile` with a [`QFileInfo`](https://doc.qt.io/qt-6/qfileinfo.html).
    pub fn open(
        self: Pin<&mut Self>,
        fd: FileDescriptor,
        mode: QIODeviceOpenMode,
        handle_flags: QFileDeviceFileHandleFlags,
    ) -> bool {
        self.open_int(fd.into(), mode, handle_flags)
    }

    /// Returns `true` if Qt supports moving files to a trash (recycle bin) in the current operating system using the [`move_to_trash`](QFile::move_to_trash) function, `false` otherwise. Note that this function returning `true` does not imply [`move_to_trash`](QFile::move_to_trash) will succeed. In particular, this function does not check if the user has disabled the functionality in their settings.
    ///
    /// Introduced in Qt 6.9.
    #[cfg(cxxqt_qt_version_at_least_6_9)]
    pub fn supports_move_to_trash() -> bool {
        ffi::qfile_supports_move_to_trash()
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

impl Deref for QFile {
    type Target = QFileDevice;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl AsRef<QFileDevice> for QFile {
    fn as_ref(&self) -> &QFileDevice {
        self.upcast()
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

impl AsRef<QIODevice> for QFile {
    fn as_ref(&self) -> &QIODevice {
        self.upcast()
    }
}

impl Upcast<QObject> for QFile {
    unsafe fn upcast_ptr(this: *const Self) -> *const QObject {
        ffi::upcast_qfile_qobject(this)
    }

    unsafe fn from_base_ptr(base: *const QObject) -> *const Self {
        ffi::downcast_qobject_qfile(base)
    }
}

impl AsRef<QObject> for QFile {
    fn as_ref(&self) -> &QObject {
        self.upcast()
    }
}

impl Read for Pin<&mut QFile> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_io_device_mut().read(buf)
    }
}

impl Write for Pin<&mut QFile> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_io_device_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.as_file_device_mut().flush();
        Ok(())
    }
}

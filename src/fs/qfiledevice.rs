use std::fmt;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;

use cxx_qt::casting::Upcast;
use cxx_qt::QObject;
use cxx_qt_lib::{QDateTime, QFlags};

use crate::qobject::debug_qobject;
use crate::util::IsNonNull;
use crate::{FileDescriptor, QIODevice};

#[cxx_qt::bridge]
mod ffi {
    /// This enum describes the errors that may be returned by [`QFileDevice::error`].
    #[repr(i32)]
    #[derive(Debug)]
    enum QFileDeviceFileError {
        /// No error occurred.
        NoError,
        /// An error occurred when reading from the file.
        ReadError,
        /// An error occurred when writing to the file.
        WriteError,
        /// A fatal error occurred.
        FatalError,
        /// Out of resources (e.g., too many open files, out of memory, etc.)
        ResourceError,
        /// The file could not be opened.
        OpenError,
        /// The operation was aborted.
        AbortError,
        /// A timeout occurred.
        TimeOutError,
        /// An unspecified error occurred.
        UnspecifiedError,
        /// The file could not be removed.
        RemoveError,
        /// The file could not be renamed.
        RenameError,
        /// The position in the file could not be changed.
        PositionError,
        /// The file could not be resized.
        ResizeError,
        /// The file could not be accessed.
        PermissionsError,
        /// The file could not be copied.
        CopyError,
    }

    /// This enum is used when opening a file to specify additional options which only apply to files and not to a generic `QIODevice`.
    #[repr(i32)]
    enum QFileDeviceFileHandleFlag {
        /// The file handle passed into [`QIODevice::open`] should be closed by [`QIODevice::close`], the default behavior is that [`QIODevice::close`] just flushes the file and the application is responsible for closing the file handle. When opening a file by name, this flag is ignored as Qt always owns the file handle and must close it.
        AutoCloseHandle = 0x0001,
        /// If not explicitly closed, the underlying file handle is left open when the `QFile` object is destroyed.
        DontCloseHandle = 0,
    }

    /// This enum is used by [`QFileDevice::permissions`] function to report the permissions and ownership of a file. The values may be OR-ed together to test multiple permissions and ownership values.
    ///
    /// **Warning:** Because of differences in the platforms supported by Qt, the semantics of [`ReadUser`](QFileDevicePermission::ReadUser), [`WriteUser`](QFileDevicePermission::WriteUser) and [`ExeUser`](QFileDevicePermission::ExeUser) are platform-dependent: On Unix, the rights of the owner of the file are returned and on Windows the rights of the current user are returned. This behavior might change in a future Qt version.
    ///
    /// [Qt Documentation: QFile::Permission](https://doc.qt.io/qt-6/qfiledevice.html#Permission-enum)
    #[repr(i32)]
    enum QFileDevicePermission {
        /// The file is readable by the owner of the file.
        ReadOwner = 0x4000,
        /// The file is writable by the owner of the file.
        WriteOwner = 0x2000,
        /// The file is executable by the owner of the file.
        ExeOwner = 0x1000,
        /// The file is readable by the user.
        ReadUser = 0x0400,
        /// The file is writable by the user.
        WriteUser = 0x0200,
        /// The file is executable by the user.
        ExeUser = 0x0100,
        /// The file is readable by the group.
        ReadGroup = 0x0040,
        /// The file is writable by the group.
        WriteGroup = 0x0020,
        /// The file is executable by the group.
        ExeGroup = 0x0010,
        /// The file is readable by others.
        ReadOther = 0x0004,
        /// The file is writable by others.
        WriteOther = 0x0002,
        /// The file is executable by others.
        ExeOther = 0x0001,
    }

    /// This enum is used by [`QFileDevice::file_time`] and [`QFileDevice::set_file_time`].
    #[repr(i32)]
    enum QFileDeviceFileTime {
        /// When the file was most recently accessed (e.g. read or written to).
        FileAccessTime,
        /// When the file was created (may not be not supported on UNIX).
        FileBirthTime,
        /// When the file's metadata was last changed.
        FileMetadataChangeTime,
        /// When the file was most recently modified.
        FileModificationTime,
    }

    /// This enum describes special options that may be used by [`QFileDevice::map`].
    #[repr(i32)]
    enum QFileDeviceMemoryMapFlag {
        /// No options.
        NoOptions = 0,
        /// The mapped memory will be private, so any modifications will not be visible to other processes and will not be written to disk. Any such modifications will be lost when the memory is unmapped. It is unspecified whether modifications made to the file made after the mapping is created will be visible through the mapped memory.
        MapPrivateOption = 0x0001,
    }

    extern "C++" {
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = cxx_qt_lib::QDateTime;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qtypes.h");
        type qint64 = cxx_qt_lib::qint64;
    }

    extern "C++" {
        include!("cxx-qt-io/qfiledevice.h");
        type QFileDeviceFileError;
        type QFileDeviceFileHandleFlag;
        type QFileDeviceFileTime;
        type QFileDeviceMemoryMapFlag;
        type QFileDeviceMemoryMapFlags = super::QFileDeviceMemoryMapFlags;
        type QFileDevicePermission;
        type QFileDevicePermissions = super::QFileDevicePermissions;

        type QIODevice = crate::QIODevice;
    }

    unsafe extern "C++Qt" {
        /// The `QFileDevice` class provides an interface for reading from and writing to open files.
        ///
        /// Qt Documentation: [QFileDevice](https://doc.qt.io/qt-6/qfiledevice.html#details)
        #[qobject]
        #[base = QIODevice]
        type QFileDevice;

        /// Returns the file error status.
        ///
        /// The I/O device status returns an error code. For example, if [`self.open()`](QIODevice::open) returns `false`, or a read/write operation returns -1, this function can be called to find out the reason why the operation failed.
        fn error(self: &QFileDevice) -> QFileDeviceFileError;

        /// Returns the name of the file. The default implementation in `QFileDevice` returns a null string.
        #[rust_name = "file_name"]
        fn fileName(self: &QFileDevice) -> QString;

        #[doc(hidden)]
        #[rust_name = "file_time_or_invalid"]
        fn fileTime(self: &QFileDevice, time: QFileDeviceFileTime) -> QDateTime;

        /// Flushes any buffered data to the file. Returns `true` if successful; otherwise returns `false`.
        fn flush(self: Pin<&mut QFileDevice>) -> bool;

        #[doc(hidden)]
        #[rust_name = "handle_or_negative"]
        fn handle(self: &QFileDevice) -> i32;

        #[doc(hidden)]
        #[rust_name = "map_qint64"]
        fn map(
            self: Pin<&mut QFileDevice>,
            offset: qint64,
            size: qint64,
            flags: QFileDeviceMemoryMapFlags,
        ) -> *mut u8;

        /// Returns the complete OR-ed together combination of [`QFileDevicePermission`]s for the file.
        fn permissions(self: &QFileDevice) -> QFileDevicePermissions;

        #[doc(hidden)]
        #[rust_name = "resize_qint64"]
        fn resize(self: Pin<&mut QFileDevice>, sz: qint64) -> bool;

        /// Sets the file time specified by `file_time` to `new_date`, returning `true` if successful; otherwise returns `false`.
        #[rust_name = "set_file_time"]
        fn setFileTime(
            self: Pin<&mut QFileDevice>,
            new_date: &QDateTime,
            file_time: QFileDeviceFileTime,
        ) -> bool;

        /// Sets the permissions for the file to the `permissions` specified. Returns `true` if successful, or `false` if the permissions cannot be modified.
        ///
        /// **Warning:** This function does not manipulate ACLs, which may limit its effectiveness.
        #[rust_name = "set_permissions"]
        fn setPermissions(self: Pin<&mut QFileDevice>, permissions: QFileDevicePermissions)
            -> bool;

        /// Unmaps the memory address.
        ///
        /// Returns `true` if the unmap succeeds; `false` otherwise.
        ///
        /// # Safety
        ///
        /// `address` must be a valid pointer obtained from [`self.map()`](QFileDevice::map).
        unsafe fn unmap(self: Pin<&mut QFileDevice>, address: *mut u8) -> bool;

        /// Sets the file's error to [`QFileDeviceFileError::NoError`].
        #[rust_name = "unset_error"]
        fn unsetError(self: Pin<&mut QFileDevice>);
    }

    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/casting.h");

        #[rust_name = "upcast_qfiledevice_qobject"]
        unsafe fn upcastPtr(device: *const QFileDevice) -> *const QObject;
        #[rust_name = "downcast_qobject_qfiledevice"]
        unsafe fn downcastPtr(device: *const QObject) -> *const QFileDevice;
    }
}

pub use ffi::{
    QFileDevice, QFileDeviceFileError, QFileDeviceFileHandleFlag, QFileDeviceFileTime,
    QFileDeviceMemoryMapFlag, QFileDevicePermission,
};

/// [`QFlags`] of [`QFileDevicePermission`].
pub type QFileDevicePermissions = QFlags<QFileDevicePermission>;
unsafe_impl_qflag!(QFileDevicePermission, "QFileDevicePermissions");

/// [`QFlags`] of [`QFileDeviceFileHandleFlag`].
pub type QFileDeviceFileHandleFlags = QFlags<QFileDeviceFileHandleFlag>;
unsafe_impl_qflag!(QFileDeviceFileHandleFlag, "QFileDeviceFileHandleFlags");

/// [`QFlags`] of [`QFileDeviceMemoryMapFlag`].
pub type QFileDeviceMemoryMapFlags = QFlags<QFileDeviceMemoryMapFlag>;
unsafe_impl_qflag!(QFileDeviceMemoryMapFlag, "QFileDeviceMemoryMapFlags");

impl fmt::Debug for QFileDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QFileDevice {
    /// Returns the file time specified by `time`. If the time cannot be determined return `None`.
    pub fn file_time(&self, time: QFileDeviceFileTime) -> Option<QDateTime> {
        self.file_time_or_invalid(time).nonnull()
    }

    /// Returns the file handle of the file.
    ///
    /// This is a small positive integer, suitable for use with C library functions such as [`fdopen()`](https://pubs.opengroup.org/onlinepubs/009695399/functions/fdopen.html) and [`fcntl()`](https://pubs.opengroup.org/onlinepubs/007904975/functions/fcntl.html). On systems that use file descriptors for sockets (i.e. Unix systems, but not Windows) the handle can be used with [`QSocketNotifier`](https://doc.qt.io/qt-6/qsocketnotifier.html) as well.
    ///
    /// If the file is not open, or there is an error, this function returns `None`.
    pub fn handle(&self) -> Option<FileDescriptor> {
        FileDescriptor::from(self.handle_or_negative()).nonnull()
    }

    /// Maps size bytes of the file into memory starting at `offset`. A file should be open for a map to succeed but the file does not need to stay open after the memory has been mapped. When the `QFile` is destroyed or a new file is opened with this object, any maps that have not been unmapped will automatically be unmapped.
    ///
    /// The mapping will have the same open mode as the file (read and/or write), except when using [`QFileDeviceMemoryMapFlag::MapPrivateOption`], in which case it is always possible to write to the mapped memory.
    ///
    /// Any mapping options can be passed through flags.
    ///
    /// Returns a pointer to the memory or a null pointer if there is an error.
    pub fn map(
        self: Pin<&mut Self>,
        offset: i64,
        size: i64,
        flags: QFileDeviceMemoryMapFlags,
    ) -> *mut u8 {
        self.map_qint64(offset.into(), size.into(), flags)
    }

    /// Sets the file size (in bytes) `sz`. Returns `true` if the resize succeeds; `false` otherwise. If `sz` is larger than the file currently is, the new bytes will be set to 0; if `sz` is smaller, the file is simply truncated.
    ///
    /// *Warning:* This function can fail if the file doesn't exist.
    pub fn resize(self: Pin<&mut Self>, sz: i64) -> bool {
        self.resize_qint64(sz.into())
    }

    /// Casts this object to `QIODevice`.
    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    /// Mutably casts this object to `QIODevice`.
    pub fn as_io_device_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QIODevice> {
        self.as_mut().upcast_pin()
    }
}

impl Deref for QFileDevice {
    type Target = QIODevice;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

unsafe impl Upcast<QObject> for QFileDevice {
    unsafe fn upcast_ptr(this: *const Self) -> *const QObject {
        ffi::upcast_qfiledevice_qobject(this)
    }

    unsafe fn from_base_ptr(base: *const QObject) -> *const Self {
        ffi::downcast_qobject_qfiledevice(base)
    }
}

impl Read for Pin<&mut QFileDevice> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_io_device_mut().read(buf)
    }
}

impl Write for Pin<&mut QFileDevice> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_io_device_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.as_mut().flush();
        Ok(())
    }
}

impl From<QFileDeviceFileError> for io::ErrorKind {
    fn from(value: QFileDeviceFileError) -> Self {
        match value {
            QFileDeviceFileError::AbortError => Self::Interrupted,
            QFileDeviceFileError::TimeOutError => Self::TimedOut,
            QFileDeviceFileError::PermissionsError => Self::PermissionDenied,
            _ => Self::Other,
        }
    }
}

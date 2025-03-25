use crate::adapter::{QIOExt, QIO};
use crate::QIODevice;
use cxx_qt::Upcast;
use cxx_qt_lib::{QDateTime, QFlags};
use std::io::{self, Read, Write};
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum FileError {
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

    #[repr(i32)]
    #[derive(Debug)]
    enum FileHandleFlag {
        /// The file handle passed into `open()` should be closed by `close()`, the default behavior is that `close` just flushes the file and the application is responsible for closing the file handle. When opening a file by name, this flag is ignored as Qt always owns the file handle and must close it.
        AutoCloseHandle = 0x0001,
        /// If not explicitly closed, the underlying file handle is left open when the `QFile` object is destroyed.
        DontCloseHandle = 0,
    }

    #[repr(i32)]
    #[derive(Debug)]
    enum FileTime {
        /// When the file was most recently accessed (e.g. read or written to).
        FileAccessTime,
        /// When the file was created (may not be not supported on UNIX).
        FileBirthTime,
        /// When the file's metadata was last changed.
        FileMetadataChangeTime,
        /// When the file was most recently modified.
        FileModificationTime,
    }

    #[repr(i32)]
    #[derive(Debug)]
    enum MemoryMapFlag {
        /// No options.
        NoOptions = 0,
        /// The mapped memory will be private, so any modifications will not be visible to other processes and will not be written to disk. Any such modifications will be lost when the memory is unmapped. It is unspecified whether modifications made to the file made after the mapping is created will be visible through the mapped memory.
        MapPrivateOption = 0x0001,
    }

    #[repr(i32)]
    #[derive(Debug)]
    enum FilePermission {
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

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = cxx_qt_lib::QDateTime;

        include!("cxx-qt-io/qfiledevice.h");
        type FileError;
        type FileHandleFlag;
        #[allow(unused)]
        type FileHandleFlags = super::FileHandleFlags;
        type FileTime;
        type MemoryMapFlag;
        type MemoryMapFlags = super::MemoryMapFlags;
        type FilePermission;
        type FilePermissions = super::FilePermissions;
    }

    unsafe extern "C++Qt" {
        include!(<QtCore/QFileDevice>);

        type QIODevice = crate::QIODevice;

        #[qobject]
        #[base = QIODevice]
        type QFileDevice;

        /// Returns the file error status.
        ///
        /// The I/O device status returns an error code. For example, if `open()` returns `false`, or a read/write operation returns -1, this function can be called to find out the reason why the operation failed.
        fn error(self: &QFileDevice) -> FileError;

        /// Returns the name of the file. The default implementation in `QFileDevice` returns a null string.
        #[rust_name = "file_name"]
        fn fileName(self: &QFileDevice) -> QString;

        /// Returns the file time specified by time. If the time cannot be determined return `QDateTime()` (an invalid date time).
        #[rust_name = "file_time_or_invalid"]
        pub(self) fn fileTime(self: &QFileDevice, time: FileTime) -> QDateTime;

        /// Flushes any buffered data to the file. Returns `true` if successful; otherwise returns `false`.
        fn flush(self: Pin<&mut QFileDevice>) -> bool;

        /// Returns the file handle of the file.
        ///
        /// This is a small positive integer, suitable for use with C library functions such as `fdopen()` and `fcntl()`. On systems that use file descriptors for sockets (i.e. Unix systems, but not Windows) the handle can be used with `QSocketNotifier` as well.
        ///
        /// If the file is not open, or there is an error, `handle()` returns -1.
        #[rust_name = "handle_or_negative"]
        pub(super) fn handle(self: &QFileDevice) -> i32;

        /// Maps size bytes of the file into memory starting at `offset`. A file should be open for a map to succeed but the file does not need to stay open after the memory has been mapped. When the `QFile` is destroyed or a new file is opened with this object, any maps that have not been unmapped will automatically be unmapped.
        ///
        /// The mapping will have the same open mode as the file (read and/or write), except when using `MapPrivateOption`, in which case it is always possible to write to the mapped memory.
        ///
        /// Any mapping options can be passed through flags.
        ///
        /// Returns a pointer to the memory or a null pointer if there is an error.
        fn map(
            self: Pin<&mut QFileDevice>,
            offset: i64,
            size: i64,
            flags: MemoryMapFlags,
        ) -> *mut u8;

        /// Returns the complete OR-ed together combination of `Permission` for the file.
        fn permissions(self: &QFileDevice) -> FilePermissions;

        /// Sets the file size (in bytes) `sz`. Returns `true` if the resize succeeds; `false` otherwise. If `sz` is larger than the file currently is, the new bytes will be set to 0; if `sz` is smaller, the file is simply truncated.
        ///
        /// *Warning:* This function can fail if the file doesn't exist.
        fn resize(self: Pin<&mut QFileDevice>, sz: i64) -> bool;

        /// Sets the file time specified by `file_time` to `new_date`, returning `true` if successful; otherwise returns `false`.
        #[rust_name = "set_file_time"]
        fn setFileTime(
            self: Pin<&mut QFileDevice>,
            new_date: &QDateTime,
            file_time: FileTime,
        ) -> bool;

        /// Sets the permissions for the file to the `permissions` specified. Returns `true` if successful, or `false` if the permissions cannot be modified.
        ///
        /// **Warning:** This function does not manipulate ACLs, which may limit its effectiveness.
        #[rust_name = "set_permissions"]
        fn setPermissions(self: Pin<&mut QFileDevice>, permissions: FilePermissions) -> bool;

        /// Unmaps the memory address.
        ///
        /// Returns `true` if the unmap succeeds; `false` otherwise.
        ///
        /// # Safety
        ///
        /// `address` must be a valid pointer obtained from `map()`.
        unsafe fn unmap(self: Pin<&mut QFileDevice>, address: *mut u8) -> bool;

        /// Sets the file's error to `FileError::NoError`.
        #[rust_name = "unset_error"]
        fn unsetError(self: Pin<&mut QFileDevice>);
    }
}

pub use ffi::{FileError, FileHandleFlag, FilePermission, FileTime, MemoryMapFlag, QFileDevice};

pub type FilePermissions = QFlags<FilePermission>;
unsafe_impl_qflag!(FilePermission, "FilePermissions", i32);
pub type FileHandleFlags = QFlags<FileHandleFlag>;
unsafe_impl_qflag!(FileHandleFlag, "FileHandleFlags", i32);
pub type MemoryMapFlags = QFlags<MemoryMapFlag>;
unsafe_impl_qflag!(MemoryMapFlag, "MemoryMapFlags", i32);

impl QFileDevice {
    /// Returns the file time specified by time. If the time cannot be determined return `None`.
    pub fn file_time(&self, time: FileTime) -> Option<QDateTime> {
        let file_time = self.file_time_or_invalid(time);
        if file_time.is_valid() {
            Some(file_time)
        } else {
            None
        }
    }

    /// Returns the file handle of the file.
    ///
    /// This is a small positive integer, suitable for use with C library functions such as `fdopen()` and `fcntl()`. On systems that use file descriptors for sockets (i.e. Unix systems, but not Windows) the handle can be used with `QSocketNotifier` as well.
    ///
    /// If the file is not open, or there is an error, `handle()` returns `None`.
    pub fn handle(&self) -> Option<i32> {
        let handle = self.handle_or_negative();
        if handle == -1 {
            None
        } else {
            Some(handle)
        }
    }
}

impl QIO for QFileDevice {
    fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    fn as_io_device_mut(self: Pin<&mut Self>) -> Pin<&mut QIODevice> {
        self.upcast_pin()
    }

    fn flush(self: Pin<&mut Self>) -> bool {
        self.flush()
    }

    fn get_error_kind(&self) -> io::ErrorKind {
        match self.error() {
            FileError::AbortError => io::ErrorKind::Interrupted,
            FileError::ResourceError => io::ErrorKind::OutOfMemory,
            FileError::TimeOutError => io::ErrorKind::TimedOut,
            FileError::PermissionsError => io::ErrorKind::PermissionDenied,
            _ => io::ErrorKind::Other,
        }
    }
}

impl Read for Pin<&mut QFileDevice> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QIOExt::read(self.as_mut(), buf)
    }
}

impl Write for Pin<&mut QFileDevice> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        QIOExt::write(self.as_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        QIOExt::flush(self.as_mut())
    }
}

use crate::qio::{QIOExt, QIO};
use crate::util::MSecs;
use crate::QFile;
use cxx_qt::{Downcast, Upcast};
use cxx_qt_lib::{QByteArray, QFlags};
use std::ffi::{c_char, CStr};
use std::io::{self, Read, Write};
use std::pin::Pin;
use std::ptr;
use std::time::Duration;

#[cxx_qt::bridge]
mod ffi {
    /// This enum is used with [`QIODevice::open`] to describe the mode in which a device is opened. It is also returned by [`QIODevice::open_mode`].
    ///
    /// Certain flags, such as [`Unbuffered`](QIODeviceOpenModeFlag::Unbufferred) and [`Truncate`](QIODeviceOpenModeFlag::Truncate), are meaningless when used with some subclasses. Some of these restrictions are implied by the type of device that is represented by a subclass. In other cases, the restriction may be due to the implementation, or may be imposed by the underlying platform; for example, [`QTcpSocket`](crate::QTcpSocket) does not support [`Unbuffered`](QIODeviceOpenModeFlag::Unbufferred) mode, and limitations in the native API prevent [`QFile`](crate::QFile) from supporting [`Unbuffered`](QIODeviceOpenModeFlag::Unbufferred) on Windows.
    #[repr(i32)]
    enum QIODeviceOpenModeFlag {
        /// The device is not open.
        NotOpen = 0x0000,
        /// The device is open for reading.
        ReadOnly = 0x0001,
        /// The device is open for writing. Note that, for file-system subclasses (e.g. [`QFile`](crate::QFile)), this mode implies [`Truncate`](QIODeviceOpenModeFlag::Truncate) unless combined with [`ReadOnly`](QIODeviceOpenModeFlag::ReadOnly), [`Append`](QIODeviceOpenModeFlag::Append) or [`NewOnly`](QIODeviceOpenModeFlag::NewOnly).
        WriteOnly = 0x0002,
        /// The device is open for reading and writing.
        ReadWrite = 0x003,
        /// The device is opened in append mode so that all data is written to the end of the file.
        Append = 0x0004,
        /// If possible, the device is truncated before it is opened. All earlier contents of the device are lost.
        Truncate = 0x0008,
        /// When reading, the end-of-line terminators are translated to '\n'. When writing, the end-of-line terminators are translated to the local encoding, for example '\r\n' for Win32.
        Text = 0x0010,
        /// Any buffer in the device is bypassed.
        Unbuffered = 0x0020,
        /// Fail if the file to be opened already exists. Create and open the file only if it does not exist. There is a guarantee from the operating system that you are the only one creating and opening the file. Note that this mode implies [`WriteOnly`](QIODeviceOpenModeFlag::WriteOnly), and combining it with [`ReadWrite`](QIODeviceOpenModeFlag::ReadWrite) is allowed. This flag currently only affects [`QFile`](crate::QFile). Other classes might use this flag in the future, but until then using this flag with any classes other than [`QFile`](crate::QFile) may result in undefined behavior.
        NewOnly = 0x0040,
        /// Fail if the file to be opened does not exist. This flag must be specified alongside `ReadOnly`, `WriteOnly`, or `ReadWrite`. Note that using this flag with `ReadOnly` alone is redundant, as `ReadOnly` already fails when the file does not exist. This flag currently only affects `QFile`. Other classes might use this flag in the future, but until then using this flag with any classes other than `QFile` may result in undefined behavior.
        ExistingOnly = 0x0080,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++" {
        include!("cxx-qt-io/qiodevice.h");
        type QIODeviceOpenModeFlag;
        type QIODeviceOpenMode = super::QIODeviceOpenMode;
    }

    unsafe extern "C++Qt" {
        /// The `QIODevice` class is the base interface class of all I/O devices in Qt.
        ///
        /// Qt Documentation: [QIODevice](https://doc.qt.io/qt-6/qiodevice.html#details)
        #[qobject]
        type QIODevice;

        /// Returns `true` if the current read and write position is at the end of the device (i.e. there is no more data available for reading on the device); otherwise returns `false`.
        ///
        /// For some devices, `at_end()` can return `true` even though there is more data to read. This special case only applies to devices that generate data in direct response to you calling `read()` (e.g., `/dev` or `/proc` files on Unix and macOS, or console input / `stdin` on all platforms).
        #[rust_name = "at_end"]
        fn atEnd(self: &QIODevice) -> bool;

        /// Returns the number of bytes that are available for reading. This function is commonly used with sequential devices to determine the number of bytes to allocate in a buffer before reading.
        #[rust_name = "bytes_available"]
        fn bytesAvailable(self: &QIODevice) -> i64;

        /// For buffered devices, this function returns the number of bytes waiting to be written. For devices with no buffer, this function returns 0.
        #[rust_name = "bytes_to_write"]
        fn bytesToWrite(self: &QIODevice) -> i64;

        /// Returns `true` if a complete line of data can be read from the device; otherwise returns `false`.
        ///
        /// Note that unbuffered devices, which have no way of determining what can be read, always return `false`.
        #[rust_name = "can_read_line"]
        fn canReadLine(self: &QIODevice) -> bool;

        /// First emits `about_to_close()`, then closes the device and sets its OpenMode to `NotOpen`. The error string is also reset.
        fn close(self: Pin<&mut QIODevice>);

        /// Completes a read transaction.
        ///
        /// For sequential devices, all data recorded in the internal buffer during the transaction will be discarded.
        #[rust_name = "commit_transaction"]
        fn commitTransaction(self: Pin<&mut QIODevice>);

        /// Returns the index of the current read channel.
        #[rust_name = "current_read_channel"]
        fn currentReadChannel(self: &QIODevice) -> i32;

        /// Returns the index of the current write channel.
        #[rust_name = "current_write_channel"]
        fn currentWriteChannel(self: &QIODevice) -> i32;

        /// Returns a human-readable description of the last device error that occurred.
        #[rust_name = "error_string"]
        fn errorString(self: &QIODevice) -> QString;

        /// Reads one character from the device and stores it in `c`. If c is a null pointer, the character is discarded. Returns `true` on success; otherwise returns `false`.
        ///
        /// # Safety
        /// `c` must be valid or null.
        #[rust_name = "get_char_unsafe"]
        unsafe fn getChar(self: Pin<&mut QIODevice>, c: *mut c_char) -> bool;

        /// # Safety
        ///
        /// [`QIODeviceModeFlag::NewOnly`] may cause undefined behavior if `self` is not a `QFile`.
        #[rust_name = "open_unsafe"]
        pub(self) unsafe fn open(self: Pin<&mut QIODevice>, mode: QIODeviceOpenMode) -> bool;

        /// Returns the mode in which the device has been opened; i.e. [`QIODeviceOpenModeFlag::ReadOnly`] or [`QIODeviceOpenModeFlag::WriteOnly`].
        #[rust_name = "open_mode"]
        fn openMode(self: &QIODevice) -> QIODeviceOpenMode;

        /// Reads at most `max_size` bytes from the device into `data`, without side effects (i.e., if you call [`read_unsafe`](QIODevice::read) after [`peek_unsafe`](QIODevice::peek), you will get the same data). Returns the number of bytes read. If an error occurs, such as when attempting to peek a device opened in [`QIODeviceOpenModeFlag::WriteOnly`] mode, this function returns -1.
        ///
        /// 0 is returned when no more data is available for reading.
        ///
        /// # Safety
        ///
        /// `data` must be valid and `max_size` must be no greater than the maximum length of
        /// the value stored at `data`.
        #[rust_name = "peek_unsafe"]
        unsafe fn peek(self: Pin<&mut QIODevice>, data: *mut c_char, max_size: i64) -> i64;

        /// Peeks at most `max_size` bytes from the device, returning the data peeked as a `QByteArray`.
        ///
        /// This function has no way of reporting errors; returning an empty `QByteArray` can mean either that no data was currently available for peeking, or that an error occurred.
        ///
        /// # Safety
        ///
        /// `data` must be valid and `max_size` must be no greater than the maximum length of
        /// the value stored at `data`.
        #[rust_name = "peek_to_array"]
        fn peek(self: Pin<&mut QIODevice>, max_size: i64) -> QByteArray;

        /// For random-access devices, this function returns the position that data is written to or read from. For sequential devices or closed devices, where there is no concept of a "current position", 0 is returned.
        fn pos(self: &QIODevice) -> i64;

        /// Writes the character `c` to the device. Returns `true` on success; otherwise returns `false`.
        #[rust_name = "put_char"]
        fn putChar(self: Pin<&mut QIODevice>, c: c_char) -> bool;

        /// Reads at most `max_size` bytes from the device into `data`, and returns the number of bytes read. If an error occurs, such as when attempting to read from a device opened in [`QIODeviceOpenModeFlag::WriteOnly`] mode, this function returns -1.
        ///
        /// 0 is returned when no more data is available for reading. However, reading past the end of the stream is considered an error, so this function returns -1 in those cases (that is, reading on a closed socket or after a process has died).
        ///
        /// # Safety
        ///
        /// `data` must be valid and `max_size` must be no greater than the maximum length of
        /// the value stored at `data`.
        #[rust_name = "read_unsafe"]
        unsafe fn read(self: Pin<&mut QIODevice>, data: *mut c_char, max_size: i64) -> i64;

        /// Reads at most `max_size` bytes from the device, and returns the data read as a `QByteArray`.
        ///
        /// This function has no way of reporting errors; returning an empty `QByteArray` can mean either that no data was currently available for reading, or that an error occurred.
        #[rust_name = "read_to_array"]
        fn read(self: Pin<&mut QIODevice>, max_size: i64) -> QByteArray;

        /// Reads all remaining data from the device, and returns it as a byte array.
        ///
        /// This function has no way of reporting errors; returning an empty `QByteArray` can mean either that no data was currently available for reading, or that an error occurred. This function also has no way of indicating that more data may have been available and couldn't be read.
        #[rust_name = "read_all"]
        fn readAll(self: Pin<&mut QIODevice>) -> QByteArray;

        /// Returns the number of available read channels if the device is open; otherwise returns 0.
        #[rust_name = "read_channel_count"]
        fn readChannelCount(self: &QIODevice) -> i32;

        /// This function reads a line of ASCII characters from the device, up to a maximum of `max_size` - 1 bytes, stores the characters in data, and returns the number of bytes read. If a line could not be read but no error occurred, this function returns 0. If an error occurs, this function returns the length of what could be read, or -1 if nothing was read.
        ///
        /// A terminating `'\0'` byte is always appended to data, so `max_size` must be larger than 1.
        ///
        /// Data is read until either of the following conditions are met:
        ///
        /// * The first `'\n'` character is read.
        /// * `max_size - 1` bytes are read.
        /// * The end of the device data is detected.
        ///
        /// The newline character (`'\n'`) is included in the buffer. If a newline is not encountered before maxSize - 1 bytes are read, a newline will not be inserted into the buffer. On windows newline characters are replaced with `'\n'`.
        ///
        /// Note that on sequential devices, data may not be immediately available, which may result in a partial line being returned. By calling [`can_read_line`](QIODevice::can_read_line) before reading, you can check whether a complete line (including the newline character) can be read.
        ///
        /// # Safety
        ///
        /// `data` must be valid for reads for `max_size` many bytes.
        #[rust_name = "read_line_unsafe"]
        unsafe fn readLine(self: Pin<&mut QIODevice>, data: *mut c_char, max_size: i64) -> i64;

        /// Reads a line from the device, but no more than `max_size` characters, and returns the result as a byte array.
        ///
        /// The resulting line can have trailing end-of-line characters (`"\n"` or `"\r\n"`), so calling [`QByteArray::trimmed`] may be necessary.
        ///
        /// This function has no way of reporting errors; returning an empty `QByteArray` can mean either that no data was currently available for reading, or that an error occurred.
        #[rust_name = "read_line_to_array"]
        fn readLine(self: Pin<&mut QIODevice>, max_size: i64) -> QByteArray;

        /// Seeks to the start of input for random-access devices. Returns `true` on success; otherwise returns `false` (for example, if the device is not open).
        fn reset(self: Pin<&mut QIODevice>) -> bool;

        /// Rolls back a read transaction.
        ///
        /// Restores the input stream to the point of the [`start_transaction`](QIODevice::start_transaction) call. This function is commonly used to rollback the transaction when an incomplete read was detected prior to committing the transaction.
        #[rust_name = "rollback_transaction"]
        fn rollbackTransaction(self: Pin<&mut QIODevice>);

        /// For random-access devices, this function sets the current position to `pos`, returning `true` on success, or `false` if an error occurred. For sequential devices, the default behavior is to produce a warning and return `false`.
        fn seek(self: Pin<&mut QIODevice>, pos: i64) -> bool;

        /// Sets the current read channel of the device to the given channel. The current input channel is used by the functions [`read`](QIODevice::read), [`read_all`](QIODevice::read_all), [`read_line`](QIODevice::read_line), and [`get_char`](QIODevice::get_char). It also determines which channel triggers the device to emit [`ready_read`](QIODevice::ready_read).
        #[rust_name = "set_current_read_channel"]
        fn setCurrentReadChannel(self: Pin<&mut QIODevice>, channel: i32);

        /// Sets the current write channel of the device to the given channel. The current output channel is used by the functions [`write`](QIODevice::write) and [`put_char`](QIODevice::put_char). It also determines which channel triggers the device to emit [`bytes_written`](QIODevice::bytes_written).
        #[rust_name = "set_current_write_channel"]
        fn setCurrentWriteChannel(self: Pin<&mut QIODevice>, channel: i32);

        /// If `enabled` is `true`, this function sets the [`QIODeviceOpenModeFlag::Text`] flag on the device; otherwise the [`QIODeviceOpenModeFlag::Text`] flag is removed. This feature is useful for classes that provide custom end-of-line handling on a `QIODevice`.
        ///
        /// The IO device should be opened before calling this function.
        #[rust_name = "set_text_mode_enabled"]
        fn setTextModeEnabled(self: Pin<&mut QIODevice>, enabled: bool);

        /// For open random-access devices, this function returns the size of the device. For open sequential devices, [`self.bytes_available()`](QIODevice::bytes_available) is returned.
        ///
        /// If the device is closed, the size returned will not reflect the actual size of the device.
        fn size(self: &QIODevice) -> i64;

        /// Skips up to `max_size` bytes from the device. Returns the number of bytes actually skipped, or -1 on error.
        ///
        /// This function does not wait and only discards the data that is already available for reading.
        ///
        /// If the device is opened in text mode, end-of-line terminators are translated to `'\n'` symbols and count as a single byte identically to the [`read`](QIODevice::read) and [`peek`](QIODevice::peek) behavior.
        ///
        /// This function works for all devices, including sequential ones that cannot [`seek`](QIODevice::seek). It is optimized to skip unwanted data after a [`peek`](QIODevice::peek) call.
        ///
        /// For random-access devices, this function can be used to seek forward from the current position. Negative `max_size` values are not allowed.
        fn skip(self: Pin<&mut QIODevice>, max_size: i64) -> i64;

        /// Starts a new read transaction on the device.
        ///
        /// Defines a restorable point within the sequence of read operations. For sequential devices, read data will be duplicated internally to allow recovery in case of incomplete reads. For random-access devices, this function saves the current position. Call [`commit_transaction`](QIODevice::commit_transaction) or [`rollback_transaction`](QIODevice::rollback_transaction) to finish the transaction.
        #[rust_name = "start_transaction"]
        fn startTransaction(self: Pin<&mut QIODevice>);

        /// Puts the character `c` back into the device, and decrements the current position unless the position is 0. This function is usually called to "undo" a `get_char()` operation, such as when writing a backtracking parser.
        ///
        /// # Safety
        ///
        /// `c` must be the value that was retrieved by [`get_char`](QIODevice::get_char) immediately prior
        /// and there must be no transaction in progress.
        #[rust_name = "unget_char"]
        unsafe fn ungetChar(self: Pin<&mut QIODevice>, c: c_char);

        #[rust_name = "wait_for_bytes_written_msecs"]
        pub(self) fn waitForBytesWritten(self: Pin<&mut QIODevice>, msecs: i32) -> bool;

        #[rust_name = "wait_for_ready_read_msecs"]
        pub(self) fn waitForReadyRead(self: Pin<&mut QIODevice>, msecs: i32) -> bool;

        /// Writes at most `max_size` bytes of data from data to the device. Returns the number of bytes that were actually written, or -1 if an error occurred.
        ///
        /// # Safety
        ///
        /// `data` must be valid for reads for `max_size` many bytes.
        #[rust_name = "write_unsafe"]
        unsafe fn write(self: Pin<&mut QIODevice>, data: *const c_char, max_size: i64) -> i64;

        /// Writes the content of `data` to the device. Returns the number of bytes that were actually written, or -1 if an error occurred.
        #[rust_name = "write_array"]
        fn write(self: Pin<&mut QIODevice>, data: &QByteArray) -> i64;

        /// Writes data from a zero-terminated string of 8-bit characters to the device. Returns the number of bytes that were actually written, or -1 if an error occurred.
        ///
        /// # Safety
        ///
        /// `data` must be a zero-terminated string of 8-bit characters.
        #[rust_name = "write_all_unsafe"]
        unsafe fn write(self: Pin<&mut QIODevice>, data: *const c_char) -> i64;

        /// Returns the number of available write channels if the device is open; otherwise returns 0.
        #[rust_name = "write_channel_count"]
        fn writeChannelCount(self: &QIODevice) -> i32;

        /// This signal is emitted when the device is about to close. Connect this signal if you have operations that need to be performed before the device closes (e.g., if you have data in a separate buffer that needs to be written to the device).
        #[qsignal]
        #[rust_name = "about_to_close"]
        fn aboutToClose(self: Pin<&mut QIODevice>);

        /// This signal is emitted every time a payload of data has been written to the device's current write channel. The bytes argument is set to the number of bytes that were written in this payload.
        ///
        /// This signal is not emitted recursively; if you reenter the event loop or call [`wait_for_bytes_written`](QIODevice::wait_for_bytes_written) inside a slot connected to this signal, the signal will not be reemitted (although [`wait_for_bytes_written`](QIODevice::wait_for_bytes_written) may still return true).
        #[qsignal]
        #[rust_name = "bytes_written"]
        fn bytesWritten(self: Pin<&mut QIODevice>, bytes: i64);

        /// This signal is emitted every time a payload of data has been written to the device. The bytes argument is set to the number of bytes that were written in this payload, while channel is the channel they were written to. Unlike [`bytes_written`](QIODevice::bytes_written), it is emitted regardless of the current write channel.
        ///
        /// This signal can be emitted recursively - even for the same channel.
        #[qsignal]
        #[rust_name = "channel_bytes_written"]
        fn channelBytesWritten(self: Pin<&mut QIODevice>, channel: i32, bytes: i64);

        /// This signal is emitted when new data is available for reading from the device. The `channel` argument is set to the index of the read channel on which the data has arrived. Unlike [`ready_read`](QIODevice::ready_read), it is emitted regardless of the current read channel.
        ///
        /// This signal can be emitted recursively - even for the same channel.
        #[qsignal]
        #[rust_name = "channel_ready_read"]
        fn channelReadyRead(self: Pin<&mut QIODevice>, channel: i32);

        /// This signal is emitted when the input (reading) stream is closed in this device. It is emitted as soon as the closing is detected, which means that there might still be data available for reading with [`read`](QIODevice::read).
        #[qsignal]
        #[rust_name = "read_channel_finished"]
        fn readChannelFinished(self: Pin<&mut QIODevice>);

        /// This signal is emitted once every time new data is available for reading from the device's current read channel. It will only be emitted again once new data is available, such as when a new payload of network data has arrived on your network socket, or when a new block of data has been appended to your device.
        ///
        /// This signal is not emitted recursively; if you reenter the event loop or call [`wait_for_ready_read`](QIODevice::wait_for_ready_read) inside a slot connected to this signal, the signal will not be reemitted (although [`wait_for_ready_read`](QIODevice::wait_for_ready_read) may still return `true`).
        #[qsignal]
        #[rust_name = "ready_read"]
        fn readyRead(self: Pin<&mut QIODevice>);
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qiodevice_is_open"]
        fn qiodeviceIsOpen(device: &QIODevice) -> bool;

        #[rust_name = "qiodevice_is_readable"]
        fn qiodeviceIsReadable(device: &QIODevice) -> bool;

        #[rust_name = "qiodevice_is_sequential"]
        fn qiodeviceIsSequential(device: &QIODevice) -> bool;

        #[rust_name = "qiodevice_is_text_mode_enabled"]
        fn qiodeviceIsTextModeEnabled(device: &QIODevice) -> bool;

        #[rust_name = "qiodevice_is_transaction_started"]
        fn qiodeviceIsTransactionStarted(device: &QIODevice) -> bool;

        #[rust_name = "qiodevice_is_writable"]
        fn qiodeviceIsWritable(device: &QIODevice) -> bool;
    }
}

pub use ffi::{QIODevice, QIODeviceOpenModeFlag};

/// [`QFlags`] of [`QIODeviceOpenModeFlag`].
pub type QIODeviceOpenMode = QFlags<QIODeviceOpenModeFlag>;
unsafe_impl_qflag!(QIODeviceOpenModeFlag, "QIODeviceOpenMode");

#[allow(clippy::cast_possible_wrap)]
impl QIODevice {
    /// Reads one character from the device and discards it. Returns `true` on success; otherwise returns `false`.
    pub fn discard_char(self: Pin<&mut Self>) -> bool {
        // SAFETY: `ptr::null_mut()` is null.
        unsafe { self.get_char_unsafe(ptr::null_mut()) }
    }

    /// Reads one character from the device and stores it in `c`. Returns `true` on success; otherwise returns `false`.
    pub fn get_char(self: Pin<&mut Self>, c: &mut c_char) -> bool {
        // SAFETY: `c` is valid.
        unsafe { self.get_char_unsafe(c) }
    }

    /// Returns `true` if the device is open; otherwise returns `false`. A device is open if it can be read from and/or written to. By default, this function returns `false` if [`self.open_mode()`](QIODevice::open_mode) returns [`QIODeviceOpenModeFlag::NotOpen`].
    pub fn is_open(&self) -> bool {
        ffi::qiodevice_is_open(self)
    }

    /// Returns `true` if data can be read from the device; otherwise returns `false`. Use [`bytes_available`](QIODevice::bytes_available) to determine how many bytes can be read.
    pub fn is_readable(&self) -> bool {
        ffi::qiodevice_is_readable(self)
    }

    /// Returns `true` if this device is sequential; otherwise returns `false`.
    ///
    /// Sequential devices, as opposed to a random-access devices, have no concept of a start, an end, a size, or a current position, and they do not support seeking. You can only read from the device when it reports that data is available. The most common example of a sequential device is a network socket. On Unix, special files such as `/dev/zero` and fifo pipes are sequential.
    ///
    /// Regular files, on the other hand, do support random access. They have both a size and a current position, and they also support seeking backwards and forwards in the data stream. Regular files are non-sequential.
    pub fn is_sequential(&self) -> bool {
        ffi::qiodevice_is_sequential(self)
    }

    /// Returns `true` if the [`QIODeviceOpenModeFlag::Text`] flag is enabled; otherwise returns `false`.
    pub fn is_text_mode_enabled(&self) -> bool {
        ffi::qiodevice_is_text_mode_enabled(self)
    }

    /// Returns `true` if a transaction is in progress on the device, otherwise `false`.
    pub fn is_transaction_started(&self) -> bool {
        ffi::qiodevice_is_transaction_started(self)
    }

    /// Returns `true` if data can be written to the device; otherwise returns `false`.
    pub fn is_writable(&self) -> bool {
        ffi::qiodevice_is_writable(self)
    }

    /// Opens the device and sets [`self.open_mode()`](QIODevice::open_mode) to `mode`. Returns `true` if successful; otherwise returns `false`. This function should be called from any reimplementations of it or other functions that open the device.
    pub fn open(self: Pin<&mut QIODevice>, mut mode: QIODeviceOpenMode) -> bool {
        if mode.test_flag(QIODeviceOpenModeFlag::NewOnly) && (*self).downcast::<QFile>().is_none() {
            mode.set_flag(QIODeviceOpenModeFlag::NewOnly, false);
        }
        // SAFETY: `mode` does not contain `NewOnly` unless `self` is a `QFile`.
        unsafe { self.open_unsafe(mode) }
    }

    /// Reads at most `data.len()` bytes from the device into `data`, without side effects (i.e., if you call [`read`](QIODevice::read) after [`peek`](QIODevice::peek), you will get the same data). Returns the number of bytes read. If an error occurs, such as when attempting to peek a device opened in [`QIODeviceOpenModeFlag::WriteOnly`] mode, this function returns -1.
    ///
    /// 0 is returned when no more data is available for reading.
    pub fn peek(self: Pin<&mut Self>, data: &mut [c_char]) -> i64 {
        // SAFETY: `data.as_mut_ptr()` is valid up to `data.len()`.
        unsafe { self.peek_unsafe(data.as_mut_ptr(), data.len() as i64) }
    }

    /// Reads at most `data.len()` bytes from the device into `data`, and returns the number of bytes read. If an error occurs, such as when attempting to read from a device opened in [`QIODeviceOpenModeFlag::WriteOnly`] mode, this function returns -1.
    ///
    /// 0 is returned when no more data is available for reading. However, reading past the end of the stream is considered an error, so this function returns -1 in those cases (that is, reading on a closed socket or after a process has died).
    pub fn read(self: Pin<&mut Self>, data: &mut [c_char]) -> i64 {
        // SAFETY: `data.as_mut_ptr()` is valid up to `data.len()`.
        unsafe { self.read_unsafe(data.as_mut_ptr(), data.len() as i64) }
    }

    /// This function reads a line of ASCII characters from the device, up to a maximum of `max_size` - 1 bytes, stores the characters in data, and returns the number of bytes read. If a line could not be read but no error occurred, this function returns 0. If an error occurs, this function returns the length of what could be read, or -1 if nothing was read.
    ///
    /// A terminating `'\0'` byte is always appended to data, so `data.len()` must be larger than 1.
    ///
    /// Data is read until either of the following conditions are met:
    ///
    /// * The first `'\n'` character is read.
    /// * `max_size - 1` bytes are read.
    /// * The end of the device data is detected.
    ///
    /// The newline character (`'\n'`) is included in the buffer. If a newline is not encountered before maxSize - 1 bytes are read, a newline will not be inserted into the buffer. On windows newline characters are replaced with `'\n'`.
    ///
    /// Note that on sequential devices, data may not be immediately available, which may result in a partial line being returned. By calling [`can_read_line`](QIODevice::can_read_line) before reading, you can check whether a complete line (including the newline character) can be read.
    pub fn read_line(self: Pin<&mut Self>, data: &mut [c_char]) -> i64 {
        if data.len() < 2 {
            return 0;
        }
        // SAFETY: `data.as_mut_ptr()` is valid up to `data.len()` and at least 2 bytes long.
        unsafe { self.read_line_unsafe(data.as_mut_ptr(), data.len() as i64) }
    }

    /// Reads a line from the device and returns the result as a byte array.
    ///
    /// The resulting line can have trailing end-of-line characters (`"\n"` or `"\r\n"`), so calling [`QByteArray::trimmed`] may be necessary.
    ///
    /// This function has no way of reporting errors; returning an empty `QByteArray` can mean either that no data was currently available for reading, or that an error occurred.
    pub fn read_line_to_end(self: Pin<&mut Self>) -> QByteArray {
        self.read_line_to_array(0)
    }

    /// For buffered devices, this function waits until a payload of buffered written data has been written to the device and the [`bytes_written`](QIODevice::bytes_written) signal has been emitted, or until `duration` has passed. If `duration` is `None`, this function will not time out. For unbuffered devices, it returns immediately.
    ///
    /// Returns `true` if a payload of data was written to the device; otherwise returns `false` (i.e. if the operation timed out, or if an error occurred).
    ///
    /// This function can operate without an event loop. It is useful when writing non-GUI applications and when performing I/O operations in a non-GUI thread.
    ///
    /// If called from within a slot connected to the [`bytes_written`](QIODevice::bytes_written) signal, [`bytes_written`](QIODevice::bytes_written) will not be reemitted.
    pub fn wait_for_bytes_written(self: Pin<&mut QIODevice>, duration: Option<Duration>) -> bool {
        self.wait_for_bytes_written_msecs(duration.msecs())
    }

    /// Blocks until new data is available for reading and the [`ready_read`](QIODevice::ready_read) signal has been emitted, or until `duration` has passed. If `duration` is `None`, this function will not time out.
    ///
    /// Returns `true` if new data is available for reading; otherwise returns `false` (if the operation timed out or if an error occurred).
    ///
    /// This function can operate without an event loop. It is useful when writing non-GUI applications and when performing I/O operations in a non-GUI thread.
    ///
    /// If called from within a slot connected to the [`ready_read`](QIODevice::ready_read) signal, [`ready_read`](QIODevice::ready_read) will not be reemitted.
    pub fn wait_for_ready_read(self: Pin<&mut QIODevice>, duration: Option<Duration>) -> bool {
        self.wait_for_ready_read_msecs(duration.msecs())
    }

    /// Writes at most `data.len()` bytes of data from `data` to the device. Returns the number of bytes that were actually written, or -1 if an error occurred.
    pub fn write(self: Pin<&mut Self>, data: &[c_char]) -> i64 {
        // SAFETY: `data.ptr()` is valid up to `data.len()`.
        unsafe { self.write_unsafe(data.as_ptr(), data.len() as i64) }
    }

    /// Writes data from a zero-terminated string of 8-bit characters to the device. Returns the number of bytes that were actually written, or -1 if an error occurred.
    pub fn write_all(self: Pin<&mut Self>, data: &CStr) -> i64 {
        // SAFETY: `data` is a zero-terminated string of 8-bit characters.
        unsafe { self.write_all_unsafe(data.as_ptr()) }
    }
}

impl Upcast<QIODevice> for QIODevice {
    unsafe fn upcast_ptr(this: *const Self) -> *const Self {
        this
    }
    unsafe fn from_base_ptr(base: *const Self) -> *const Self {
        base
    }

    fn upcast(&self) -> &Self {
        self
    }

    fn upcast_mut(&mut self) -> &mut Self {
        self
    }

    fn upcast_pin(self: Pin<&mut Self>) -> Pin<&mut Self> {
        self
    }
}

impl QIO for QIODevice {}

impl Read for Pin<&mut QIODevice> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QIOExt::read(self.as_mut(), buf)
    }
}

impl Write for Pin<&mut QIODevice> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        QIOExt::write(self.as_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

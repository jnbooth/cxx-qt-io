use std::fmt;

use cxx_qt_lib::qintptr;

use crate::util::IsNonNull;
#[cfg(all(unix, feature = "fs"))]
use crate::FileDescriptor;

/// A [`qintptr`](https://doc.qt.io/qt-6/qttypes.html#qintptr-typedef) that references a native socket descriptor.
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketDescriptor(isize);

impl IsNonNull for SocketDescriptor {
    fn is_nonnull(value: &Self) -> bool {
        value.0 != -1
    }
}

impl fmt::Debug for SocketDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for SocketDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<isize> for SocketDescriptor {
    fn from(value: isize) -> Self {
        Self(value)
    }
}

impl From<SocketDescriptor> for isize {
    fn from(value: SocketDescriptor) -> Self {
        value.0
    }
}

impl From<qintptr> for SocketDescriptor {
    fn from(value: qintptr) -> Self {
        Self(value.into())
    }
}

impl From<SocketDescriptor> for qintptr {
    fn from(value: SocketDescriptor) -> Self {
        value.0.into()
    }
}

/// On Unix platforms, file descriptors are used for sockets.
#[cfg(all(unix, feature = "fs"))]
impl From<FileDescriptor> for SocketDescriptor {
    fn from(value: FileDescriptor) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self(i32::from(value) as isize)
    }
}
/// On Unix platforms, file descriptors are used for sockets.
#[cfg(all(unix, feature = "fs"))]
impl From<SocketDescriptor> for FileDescriptor {
    fn from(value: SocketDescriptor) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        Self::from(isize::from(value) as i32)
    }
}

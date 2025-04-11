use cxx_qt_lib::qintptr;

use crate::util::IsNonNull;

/// A [`qintptr`](https://doc.qt.io/qt-6/qttypes.html#qintptr-typedef) that references a native socket descriptor.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketDescriptor(isize);

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

impl IsNonNull for SocketDescriptor {
    fn is_nonnull(value: &Self) -> bool {
        value.0 != -1
    }
}

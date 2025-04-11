use crate::util::IsNonNull;
use std::fmt::{self, Debug, Display, Formatter};

/// An `int` that references a native file descriptor.
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileDescriptor(i32);

impl FileDescriptor {
    /// 0
    pub const STDIN: Self = Self(0);
    /// 1
    pub const STDOUT: Self = Self(1);
    /// 2
    pub const STDERR: Self = Self(2);
}

impl Debug for FileDescriptor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for FileDescriptor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl From<i32> for FileDescriptor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<FileDescriptor> for i32 {
    fn from(value: FileDescriptor) -> Self {
        value.0
    }
}

impl IsNonNull for FileDescriptor {
    fn is_nonnull(value: &Self) -> bool {
        value.0 != -1
    }
}

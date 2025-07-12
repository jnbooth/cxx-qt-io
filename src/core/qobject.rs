#![allow(dead_code)]
use std::ffi::CStr;
use std::fmt;
use std::ptr;

use cxx_qt::casting::Upcast;
use cxx_qt::QObject;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qobject.h");
        type QObject = cxx_qt::QObject;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qobject_class_name"]
        fn qobjectClassName(obj: &QObject) -> *const c_char;

        #[rust_name = "qobject_thread_eq"]
        fn qobjectThreadEq(lhs: &QObject, rhs: &QObject) -> bool;
    }
}

fn qobject_class_name<T>(obj: &T) -> &CStr
where
    T: Upcast<QObject>,
{
    let name = ffi::qobject_class_name(obj.upcast());
    // SAFETY: Qt guarantees zero-termination and obj's meta-object lives at least as long as obj.
    unsafe { CStr::from_ptr(name) }
}

pub fn debug_qobject<T>(f: &mut fmt::Formatter, obj: &T) -> fmt::Result
where
    T: Upcast<QObject>,
{
    #[inline(never)]
    fn inner(f: &mut fmt::Formatter, obj: &QObject) -> fmt::Result {
        // Should always be valid UTF-8, but just in case.
        f.debug_tuple(&qobject_class_name(obj).to_string_lossy())
            .field(&ptr::from_ref(obj))
            .finish()
    }
    inner(f, obj.upcast())
}

/// Returns `true` if both objects are in the same thread.
#[inline(always)]
pub(crate) fn in_same_thread<T, U>(lhs: &T, rhs: &U) -> bool
where
    T: Upcast<QObject>,
    U: Upcast<QObject>,
{
    ffi::qobject_thread_eq(lhs.upcast(), rhs.upcast())
}

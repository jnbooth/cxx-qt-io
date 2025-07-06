#![allow(dead_code)]
use std::pin::Pin;
use std::ptr;
use std::time::Duration;

use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QByteArray, QDate, QDateTime, QString, QTime, QVariant};

#[inline(always)]
pub(crate) fn upcast_mut<From, To>(pointer: *mut From) -> *mut To
where
    From: Upcast<To>,
{
    // SAFETY: Provided by Upcast's contract.
    unsafe { From::upcast_ptr(pointer) }.cast_mut()
}

/// Unwraps a`Pin<&mut T>` into a mutable pointer. This function should only be used
/// to directly pass the resulting pointer to an extern Qt function.
///
/// # Safety
///
/// This function is unsafe. You must guarantee that you will continue to
/// treat the `&mut T` as pinned after you call this function, so that
/// the invariants on the `Pin` type can be upheld. If the code using the
/// resulting `&mut T` does not continue to maintain the pinning invariants that
/// is a violation of the API contract and may lead to undefined behavior in
/// later (safe) operations.
///
/// Note that you must be able to guarantee that the data pointed to by `&mut T`
/// will be treated as pinned all the way until its `drop` handler is complete!
///
/// *For more information, see the [`pin` module docs][std::pin]*
#[inline(always)]
pub(crate) unsafe fn unpin_for_qt<T>(pin: Pin<&mut T>) -> *mut T {
    unsafe { ptr::from_mut(Pin::into_inner_unchecked(pin)) }
}

pub(crate) trait MSecs: Sized {
    fn msecs(self) -> i32;
}

impl MSecs for Duration {
    fn msecs(self) -> i32 {
        self.as_millis().try_into().unwrap_or(i32::MAX)
    }
}

impl MSecs for Option<Duration> {
    fn msecs(self) -> i32 {
        match self {
            Some(duration) => duration.msecs(),
            None => -1,
        }
    }
}

pub(crate) trait IsNonNull: Sized {
    fn is_nonnull(value: &Self) -> bool;

    fn nonnull(self) -> Option<Self> {
        if Self::is_nonnull(&self) {
            Some(self)
        } else {
            None
        }
    }

    fn nonnull_or<E>(self, err: E) -> Result<Self, E> {
        if Self::is_nonnull(&self) {
            Ok(self)
        } else {
            Err(err)
        }
    }
}

impl IsNonNull for QByteArray {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

impl IsNonNull for QDate {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl IsNonNull for QDateTime {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl IsNonNull for QString {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

impl IsNonNull for QTime {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl IsNonNull for QVariant {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

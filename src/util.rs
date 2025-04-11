use std::time::Duration;

use cxx_qt_lib::{QByteArray, QDate, QDateTime, QString, QTime, QVariant};

#[allow(dead_code)]
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

#[allow(dead_code)]
pub(crate) trait IsNonNull: Sized {
    fn is_nonnull(value: &Self) -> bool;

    fn nonnull(self) -> Option<Self> {
        if Self::is_nonnull(&self) {
            Some(self)
        } else {
            None
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

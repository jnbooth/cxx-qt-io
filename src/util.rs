use std::time::Duration;

use cxx_qt_lib::{QByteArray, QDate, QDateTime, QString, QTime};

#[allow(dead_code)]
pub(crate) trait MSecs: Sized {
    fn msecs(self) -> i32;
}

impl MSecs for Option<Duration> {
    fn msecs(self) -> i32 {
        match self {
            Some(duration) => duration.as_millis().try_into().unwrap_or(i32::MAX),
            None => -1,
        }
    }
}

#[allow(dead_code)]
pub(crate) trait NonNull: Sized {
    fn is_nonnull(value: &Self) -> bool;

    fn nonnull(self) -> Option<Self> {
        if Self::is_nonnull(&self) {
            Some(self)
        } else {
            None
        }
    }
}

impl NonNull for QByteArray {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

impl NonNull for QDate {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl NonNull for QDateTime {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl NonNull for QString {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

impl NonNull for QTime {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

use cxx_qt_lib::{QDate, QDateTime, QTime};

#[allow(dead_code)]
pub(crate) trait Valid: Sized {
    fn is_valid(value: &Self) -> bool;

    fn valid(self) -> Option<Self> {
        if Self::is_valid(&self) {
            Some(self)
        } else {
            None
        }
    }
}

impl Valid for QDate {
    fn is_valid(value: &Self) -> bool {
        value.is_valid()
    }
}

impl Valid for QDateTime {
    fn is_valid(value: &Self) -> bool {
        value.is_valid()
    }
}

impl Valid for QTime {
    fn is_valid(value: &Self) -> bool {
        value.is_valid()
    }
}

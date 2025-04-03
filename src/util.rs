use cxx_qt_lib::{QByteArray, QDate, QDateTime, QString, QTime};

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

impl Valid for QByteArray {
    fn is_valid(value: &Self) -> bool {
        !value.is_null()
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

impl Valid for QString {
    fn is_valid(value: &Self) -> bool {
        !value.is_null()
    }
}

impl Valid for QTime {
    fn is_valid(value: &Self) -> bool {
        value.is_valid()
    }
}

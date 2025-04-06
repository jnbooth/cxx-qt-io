#[cfg(any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_13))]
mod v5_13;
#[cfg(any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_13))]
pub use v5_13::QSslAlternativeNameEntryType;

#[cfg(all(cxxqt_qt_version_at_least_5_0, not(cxxqt_qt_version_at_least_5_13)))]
mod v5_0;
#[cfg(all(cxxqt_qt_version_at_least_5_0, not(cxxqt_qt_version_at_least_5_13)))]
pub use v5_0::QSslAlternativeNameEntryType;

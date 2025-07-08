#[cfg(cxxqt_qt_version_at_least_6_2)]
mod v6_2;
#[cfg(cxxqt_qt_version_at_least_6_2)]
pub use v6_2::QSslImplementedClass;

#[cfg(not(cxxqt_qt_version_at_least_6_2))]
mod v6_1;
#[cfg(not(cxxqt_qt_version_at_least_6_2))]
pub use v6_1::QSslImplementedClass;

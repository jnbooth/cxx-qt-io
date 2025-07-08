#[cfg(cxxqt_qt_version_at_least_6_8)]
mod v6_8;
#[cfg(cxxqt_qt_version_at_least_6_8)]
pub use v6_8::QNetworkRequestAttribute;

#[cfg(all(cxxqt_qt_version_at_least_6_5, not(cxxqt_qt_version_at_least_6_8)))]
mod v6_5;
#[cfg(all(cxxqt_qt_version_at_least_6_5, not(cxxqt_qt_version_at_least_6_8)))]
pub use v6_5::QNetworkRequestAttribute;

#[cfg(all(cxxqt_qt_version_at_least_6_3, not(cxxqt_qt_version_at_least_6_5)))]
mod v6_3;
#[cfg(all(cxxqt_qt_version_at_least_6_3, not(cxxqt_qt_version_at_least_6_5)))]
pub use v6_3::QNetworkRequestAttribute;

#[cfg(not(cxxqt_qt_version_at_least_6_3))]
mod v6_1;
#[cfg(not(cxxqt_qt_version_at_least_6_3))]
pub use v6_1::QNetworkRequestAttribute;

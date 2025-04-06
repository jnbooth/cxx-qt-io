#[cfg(any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_4))]
mod v5_6;
#[cfg(any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_4))]
pub use v5_6::QNetworkReplyNetworkError;

#[cfg(all(cxxqt_qt_version_at_least_5_0, not(cxxqt_qt_version_at_least_5_4)))]
mod v5_0;
#[cfg(all(cxxqt_qt_version_at_least_5_0, not(cxxqt_qt_version_at_least_5_4)))]
pub use v5_0::QNetworkReplyNetworkError;

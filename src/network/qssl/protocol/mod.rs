#[cfg(cxxqt_qt_version_at_least_6_3)]
mod v6_3;
#[cfg(cxxqt_qt_version_at_least_6_3)]
pub use v6_3::QSslSslProtocol;

#[cfg(all(
    any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_12),
    not(cxxqt_qt_version_at_least_6_3)
))]
mod v5_12;
#[cfg(all(
    any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_12),
    not(cxxqt_qt_version_at_least_6_3)
))]
pub use v5_12::QSslSslProtocol;

#[cfg(all(cxxqt_qt_version_at_least_5_0, not(cxxqt_qt_version_at_least_5_12)))]
mod v5_0;
#[cfg(all(cxxqt_qt_version_at_least_5_0, not(cxxqt_qt_version_at_least_5_12)))]
pub use v5_0::QSslSslProtocol;

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

#[cfg(all(cxxqt_qt_version_at_least_6_0, not(cxxqt_qt_version_at_least_6_3)))]
mod v6_0;
#[cfg(all(cxxqt_qt_version_at_least_6_0, not(cxxqt_qt_version_at_least_6_3)))]
pub use v6_0::QNetworkRequestAttribute;

#[cfg(cxxqt_qt_version_at_least_5_15)]
mod v5_15;
#[cfg(cxxqt_qt_version_at_least_5_15)]
pub use v5_15::QNetworkRequestAttribute;

#[cfg(all(cxxqt_qt_version_at_least_5_14, not(cxxqt_qt_version_at_least_5_15)))]
mod v5_14;
#[cfg(all(cxxqt_qt_version_at_least_5_14, not(cxxqt_qt_version_at_least_5_15)))]
pub use v5_14::QNetworkRequestAttribute;

#[cfg(all(cxxqt_qt_version_at_least_5_11, not(cxxqt_qt_version_at_least_5_14)))]
mod v5_11;
#[cfg(all(cxxqt_qt_version_at_least_5_11, not(cxxqt_qt_version_at_least_5_14)))]
pub use v5_11::QNetworkRequestAttribute;

#[cfg(all(cxxqt_qt_version_at_least_5_9, not(cxxqt_qt_version_at_least_5_11)))]
mod v5_9;
#[cfg(all(cxxqt_qt_version_at_least_5_9, not(cxxqt_qt_version_at_least_5_11)))]
pub use v5_9::QNetworkRequestAttribute;

#[cfg(all(cxxqt_qt_version_at_least_5_6, not(cxxqt_qt_version_at_least_5_9)))]
mod v5_6;
#[cfg(all(cxxqt_qt_version_at_least_5_6, not(cxxqt_qt_version_at_least_5_9)))]
pub use v5_6::QNetworkRequestAttribute;

#[cfg(all(cxxqt_qt_version_at_least_5_5, not(cxxqt_qt_version_at_least_5_6)))]
mod v5_5;
#[cfg(all(cxxqt_qt_version_at_least_5_5, not(cxxqt_qt_version_at_least_5_6)))]
pub use v5_5::QNetworkRequestAttribute;

#[cfg(all(cxxqt_qt_version_at_least_5_3, not(cxxqt_qt_version_at_least_5_5)))]
mod v5_3;
#[cfg(all(cxxqt_qt_version_at_least_5_3, not(cxxqt_qt_version_at_least_5_5)))]
pub use v5_3::QNetworkRequestAttribute;

#[cfg(all(cxxqt_qt_version_at_least_5_0, not(cxxqt_qt_version_at_least_5_3)))]
mod v5_0;
#[cfg(all(cxxqt_qt_version_at_least_5_0, not(cxxqt_qt_version_at_least_5_3)))]
pub use v5_0::QNetworkRequestAttribute;

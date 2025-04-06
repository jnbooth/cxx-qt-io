#[cfg(cxxqt_qt_version_at_least_6_0)]
mod v6_0;
#[cfg(cxxqt_qt_version_at_least_6_0)]
pub use v6_0::QCryptographicHashAlgorithm;

#[cfg(cxxqt_qt_version_at_least_5_9)]
mod v5_9;
#[cfg(cxxqt_qt_version_at_least_5_9)]
pub use v5_9::CryptographicHashAlgorithm;

#[cfg(all(not(cxxqt_qt_version_at_least_5_9), cxxqt_qt_version_at_least_5_1))]
mod v5_1;
#[cfg(all(not(cxxqt_qt_version_at_least_5_9), cxxqt_qt_version_at_least_5_1))]
pub use v5_1::CryptographicHashAlgorithm;

#[cfg(all(not(cxxqt_qt_version_at_least_5_1), cxxqt_qt_version_at_least_5_0))]
mod v5_0;
#[cfg(all(not(cxxqt_qt_version_at_least_5_1), cxxqt_qt_version_at_least_5_0))]
pub use v5_0::CryptographicHashAlgorithm;

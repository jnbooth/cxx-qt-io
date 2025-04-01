#[cfg(cxxqt_qt_version_at_least_6_0)]
mod cryptographic_hash_algorithm_6_0;
#[cfg(cxxqt_qt_version_at_least_6_0)]
pub use cryptographic_hash_algorithm_6_0::CryptographicHashAlgorithm;

#[cfg(cxxqt_qt_version_at_least_5_9)]
mod cryptographic_hash_algorithm_5_9;
#[cfg(cxxqt_qt_version_at_least_5_9)]
pub use cryptographic_hash_algorithm_5_9::CryptographicHashAlgorithm;

#[cfg(all(not(cxxqt_qt_version_at_least_5_9), cxxqt_qt_version_at_least_5_1))]
mod cryptographic_hash_algorithm_5_1;
#[cfg(all(not(cxxqt_qt_version_at_least_5_9), cxxqt_qt_version_at_least_5_1))]
pub use cryptographic_hash_algorithm_5_1::CryptographicHashAlgorithm;

#[cfg(all(not(cxxqt_qt_version_at_least_5_1), cxxqt_qt_version_at_least_5_0))]
mod cryptographic_hash_algorithm;
#[cfg(all(not(cxxqt_qt_version_at_least_5_1), cxxqt_qt_version_at_least_5_0))]
pub use cryptographic_hash_algorithm::CryptographicHashAlgorithm;

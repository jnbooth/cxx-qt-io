use std::fmt;

use cxx::{ExternType, type_id};
use cxx_qt_lib::QString;

use crate::util::IsNonNull;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslellipticcurve.h");
    }

    unsafe extern "C++" {
        type QSslEllipticCurve = super::QSslEllipticCurve;

        #[doc(hidden)]
        #[Self = "QSslEllipticCurve"]
        #[rust_name = "from_long_name_or_null"]
        fn fromLongName(name: &QString) -> QSslEllipticCurve;

        #[doc(hidden)]
        #[Self = "QSslEllipticCurve"]
        #[rust_name = "from_short_name_or_null"]
        fn fromShortName(name: &QString) -> QSslEllipticCurve;

        /// Returns `true` if this elliptic curve is one of the named curves that can be used in the key exchange when using an elliptic curve cipher with TLS; `false` otherwise.
        #[rust_name = "is_tls_named_curve"]
        fn isTlsNamedCurve(&self) -> bool;

        /// Returns `true` if this elliptic curve is a valid curve, `false` otherwise.
        #[rust_name = "is_valid"]
        fn isValid(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "long_name_or_empty"]
        fn longName(&self) -> QString;

        #[doc(hidden)]
        #[rust_name = "short_name_or_empty"]
        fn shortName(&self) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslellipticcurve_init_default"]
        fn construct() -> QSslEllipticCurve;
    }
}

/// Represents an elliptic curve for use by elliptic-curve cipher algorithms.
///
/// Qt Documentation: [QSslEllipticCurve](https://doc.qt.io/qt-6/qsslellipticcurve.html#details)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QSslEllipticCurve {
    id: i32,
}

impl Default for QSslEllipticCurve {
    /// Constructs an invalid elliptic curve.
    fn default() -> Self {
        ffi::qsslellipticcurve_init_default()
    }
}

impl IsNonNull for QSslEllipticCurve {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl fmt::Debug for QSslEllipticCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QSslEllipticCurve")
            .field("is_tls_named_curve", &self.is_tls_named_curve())
            .field("is_valid", &self.is_valid())
            .field("long_name", &self.long_name())
            .field("short_name", &self.short_name())
            .finish()
    }
}

impl fmt::Display for QSslEllipticCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.short_name_or_empty().fmt(f)
    }
}

impl QSslEllipticCurve {
    /// Returns a `QSslEllipticCurve` instance representing the named curve `name`. The `name` is a long name for the curve, whose exact spelling depends on the SSL implementation.
    ///
    /// Returns an error if the given `name` is not supported.
    ///
    /// **Note:** The OpenSSL implementation of this function treats the name case-sensitively.
    pub fn from_long_name(name: &QString) -> Result<Self, QSslEllipticCurveError> {
        Self::from_long_name_or_null(name).nonnull_or(QSslEllipticCurveError(()))
    }

    /// Returns a `QSslEllipticCurve` instance representing the named curve `name`. The `name` is the conventional short name for the curve, as represented by RFC 4492 (for instance secp521r1), or as NIST short names (for instance P-256). The actual set of recognized names depends on the SSL implementation.
    ///
    /// Returns an error if the given `name` is not supported.
    ///
    /// **Note:** The OpenSSL implementation of this function treats the name case-sensitively.
    pub fn from_short_name(name: &QString) -> Result<Self, QSslEllipticCurveError> {
        Self::from_short_name_or_null(name).nonnull_or(QSslEllipticCurveError(()))
    }

    /// Returns the conventional long name for this curve. If this curve is invalid, returns `None`.
    pub fn long_name(&self) -> Option<QString> {
        self.long_name_or_empty().nonnull()
    }

    /// Returns the conventional short name for this curve. If this curve is invalid, returns `None`.
    pub fn short_name(&self) -> Option<QString> {
        self.short_name_or_empty().nonnull()
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QSslEllipticCurve {
    type Id = type_id!("QSslEllipticCurve");
    type Kind = cxx::kind::Trivial;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct QSslEllipticCurveError(pub(crate) ());

impl fmt::Display for QSslEllipticCurveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("the supplied name is not a supported elliptic curve")
    }
}

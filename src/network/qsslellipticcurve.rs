use std::fmt::{self, Debug, Formatter};

use cxx::{type_id, ExternType};

use crate::util::Valid;
use cxx_qt_lib::QString;

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

        /// Returns `true` if this elliptic curve is one of the named curves that can be used in the key exchange when using an elliptic curve cipher with TLS; `false` otherwise.
        #[rust_name = "is_tls_named_curve"]
        fn isTlsNamedCurve(&self) -> bool;

        /// Returns `true` if this elliptic curve is a valid curve, `false` otherwise.
        #[rust_name = "is_valid"]
        fn isValid(&self) -> bool;

        /// Returns the conventional long name for this curve. If this curve is invalid, returns an empty string.
        #[rust_name = "long_name"]
        fn longName(&self) -> QString;

        /// Returns the conventional short name for this curve. If this curve is invalid, returns an empty string.
        #[rust_name = "short_name"]
        fn shortName(&self) -> QString;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qsslellipticcurve_from_long_name"]
        fn qsslellipticcurveFromLongName(name: &QString) -> QSslEllipticCurve;

        #[rust_name = "qsslellipticcurve_from_short_name"]
        fn qsslellipticcurveFromShortName(name: &QString) -> QSslEllipticCurve;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslellipticcurve_init_default"]
        fn construct() -> QSslEllipticCurve;

        #[rust_name = "qsslellipticcurve_to_debug_qstring"]
        fn toDebugQString(value: &QSslEllipticCurve) -> QString;
    }
}

impl Valid for QSslEllipticCurve {
    fn is_valid(value: &Self) -> bool {
        value.is_valid()
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

impl Debug for QSslEllipticCurve {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qsslellipticcurve_to_debug_qstring(self))
    }
}

impl QSslEllipticCurve {
    /// Returns a `QSslEllipticCurve` instance representing the named curve `name`. The `name` is a long name for the curve, whose exact spelling depends on the SSL implementation.
    ///
    /// If the given `name` is not supported, returns `None`.
    ///
    /// **Note:** The OpenSSL implementation of this function treats the name case-sensitively.
    pub fn from_long_name(name: &QString) -> Option<Self> {
        ffi::qsslellipticcurve_from_long_name(name).valid()
    }

    /// Returns a `QSslEllipticCurve` instance representing the named curve `name`. The `name` is the conventional short name for the curve, as represented by RFC 4492 (for instance secp521r1), or as NIST short names (for instance P-256). The actual set of recognized names depends on the SSL implementation.
    ///
    /// If the given `name` is not supported, returns `None`.
    ///
    /// **Note:** The OpenSSL implementation of this function treats the name case-sensitively.
    pub fn from_short_name(name: &QString) -> Option<Self> {
        ffi::qsslellipticcurve_from_short_name(name).valid()
    }
}

unsafe impl ExternType for QSslEllipticCurve {
    type Id = type_id!("QSslEllipticCurve");
    type Kind = cxx::kind::Trivial;
}

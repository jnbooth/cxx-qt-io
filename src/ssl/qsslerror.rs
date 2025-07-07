use std::fmt;
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};

use crate::util::IsNonNull;
use crate::QSslCertificate;

#[cxx::bridge]
mod ffi {
    /// Describes all recognized errors that can occur during an SSL handshake.
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum QSslErrorSslError {
        NoError,
        UnableToGetIssuerCertificate,
        UnableToDecryptCertificateSignature,
        UnableToDecodeIssuerPublicKey,
        CertificateSignatureFailed,
        CertificateNotYetValid,
        CertificateExpired,
        InvalidNotBeforeField,
        InvalidNotAfterField,
        SelfSignedCertificate,
        SelfSignedCertificateInChain,
        UnableToGetLocalIssuerCertificate,
        UnableToVerifyFirstCertificate,
        CertificateRevoked,
        InvalidCaCertificate,
        PathLengthExceeded,
        InvalidPurpose,
        CertificateUntrusted,
        CertificateRejected,
        SubjectIssuerMismatch, // hostname mismatch?
        AuthorityIssuerSerialNumberMismatch,
        NoPeerCertificate,
        HostNameMismatch,
        NoSslSupport,
        CertificateBlacklisted,
        CertificateStatusUnknown,
        OcspNoResponseFound,
        OcspMalformedRequest,
        OcspMalformedResponse,
        OcspInternalError,
        OcspTryLater,
        OcspSigRequred,
        OcspUnauthorized,
        OcspResponseCannotBeTrusted,
        OcspResponseCertIdUnknown,
        OcspResponseExpired,
        OcspStatusUnknown,
        UnspecifiedError = -1,
    }

    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-io/qsslcertificate.h");
        type QSslCertificate = crate::QSslCertificate;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslerror.h");
        type QSslErrorSslError;
    }

    unsafe extern "C++" {
        type QSslError = super::QSslError;

        #[doc(hidden)]
        #[rust_name = "certificate_or_null"]
        fn certificate(&self) -> QSslCertificate;

        /// Returns the type of the error.
        fn error(&self) -> QSslErrorSslError;

        /// Returns a short localized human-readable description of the error.
        #[rust_name = "error_string"]
        fn errorString(&self) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslerror_drop"]
        fn drop(extension: &mut QSslError);

        #[rust_name = "qsslerror_init_default"]
        fn construct() -> QSslError;
        #[rust_name = "qsslerror_init_error"]
        fn construct(error: QSslErrorSslError) -> QSslError;
        #[rust_name = "qsslerror_init_certificate"]
        fn construct(error: QSslErrorSslError, certificate: &QSslCertificate) -> QSslError;
        #[rust_name = "qsslerror_clone"]
        fn construct(other: &QSslError) -> QSslError;

        #[rust_name = "qsslerror_eq"]
        fn operatorEq(a: &QSslError, b: &QSslError) -> bool;
    }
}

pub use ffi::QSslErrorSslError;

/// The `QSslError` class provides an SSL error.
///
/// Qt Documentation: [QSslError](https://doc.qt.io/qt-6/qsslerror.html#details)
#[repr(C)]
pub struct QSslError {
    _space: MaybeUninit<usize>,
}

impl Clone for QSslError {
    fn clone(&self) -> Self {
        ffi::qsslerror_clone(self)
    }
}

impl Drop for QSslError {
    fn drop(&mut self) {
        ffi::qsslerror_drop(self);
    }
}

impl Default for QSslError {
    /// Constructs a `QSslError` object with no error and default certificate.
    fn default() -> Self {
        ffi::qsslerror_init_default()
    }
}

impl PartialEq for QSslError {
    fn eq(&self, other: &Self) -> bool {
        ffi::qsslerror_eq(self, other)
    }
}

impl Eq for QSslError {}

impl fmt::Debug for QSslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error().fmt(f)
    }
}

impl fmt::Display for QSslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error_string().fmt(f)
    }
}

impl QSslError {
    /// Constructs a `QSslError` object. The two arguments specify the `error` that occurred, and which `certificate` the error relates to.
    ///
    /// To create a `QSslError` that applies to any certificate. use `QSslError::From<QSslErrorSslError>`.
    pub fn new(error: QSslErrorSslError, certificate: &QSslCertificate) -> Self {
        ffi::qsslerror_init_certificate(error, certificate)
    }

    /// Returns the certificate associated with this error, or `None` if the error does not relate to any certificate.
    pub fn certificate(&self) -> Option<QSslCertificate> {
        self.certificate_or_null().nonnull()
    }
}

impl From<QSslErrorSslError> for QSslError {
    fn from(value: QSslErrorSslError) -> Self {
        ffi::qsslerror_init_error(value)
    }
}

unsafe impl ExternType for QSslError {
    type Id = type_id!("QSslError");
    type Kind = cxx::kind::Trivial;
}

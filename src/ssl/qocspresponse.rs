use std::fmt;
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    /// Describes the Online Certificate Status.
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum QOcspCertificateStatus {
        /// The certificate is not revoked, but this does not necessarily mean that the certificate was ever issued or that the time at which the response was produced is within the certificate's validity interval.
        Good,
        /// This state indicates that the certificate has been revoked (either permanently or temporarily - on hold).
        Revoked,
        /// This state indicates that the responder doesn't know about the certificate being requested.
        Unknown,
    }

    /// Describes the reason for revocation.
    ///
    /// This enumeration describes revocation reasons, defined in [RFC 5280, section 5.3.1](https://datatracker.ietf.org/doc/html/rfc5280#section-5.3.1).
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum QOcspRevocationReason {
        Unspecified,
        KeyCompromise,
        CACompromise,
        AffiliationChanged,
        Superseded,
        CessationOfOperation,
        CertificateHold,
        RemoveFromCRL,
        None = -1,
    }

    extern "C++" {
        include!("cxx-qt-io/qsslcertificate.h");
        type QSslCertificate = crate::QSslCertificate;
    }

    extern "C++" {
        include!("cxx-qt-io/qocspresponse.h");
        type QOcspCertificateStatus;
        type QOcspRevocationReason;
    }

    unsafe extern "C++" {
        type QOcspResponse = super::QOcspResponse;

        /// Returns the certificate status.
        #[rust_name = "certificate_status"]
        fn certificateStatus(&self) -> QOcspCertificateStatus;

        /// This function returns a certificate used to sign OCSP response.
        fn responder(&self) -> QSslCertificate;

        /// Returns the reason for revocation.
        #[rust_name = "revocation_reason"]
        fn revocationReason(&self) -> QOcspRevocationReason;

        /// This function returns a certificate, for which this response was issued.
        fn subject(&self) -> QSslCertificate;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qocspresponse_drop"]
        fn drop(extension: &mut QOcspResponse);

        #[rust_name = "qocspresponse_init_default"]
        fn construct() -> QOcspResponse;
        #[rust_name = "qocspresponse_clone"]
        fn construct(other: &QOcspResponse) -> QOcspResponse;
    }
}

pub use ffi::{QOcspCertificateStatus, QOcspRevocationReason};

/// This class represents Online Certificate Status Protocol response.
///
/// Qt Documentation: [QOcspResponse](https://doc.qt.io/qt-6/qocspresponse.html#details)
#[repr(C)]
pub struct QOcspResponse {
    _space: MaybeUninit<usize>,
}

impl Clone for QOcspResponse {
    fn clone(&self) -> Self {
        ffi::qocspresponse_clone(self)
    }
}

impl Drop for QOcspResponse {
    fn drop(&mut self) {
        ffi::qocspresponse_drop(self);
    }
}

impl Default for QOcspResponse {
    /// Creates a new response with status [`QOcspCertificateStatus::Unknown`] and revocation reason [`QOcspRevocationReason::None`].
    fn default() -> Self {
        ffi::qocspresponse_init_default()
    }
}

impl fmt::Debug for QOcspResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QOcspResponse")
            .field("certificate_status", &self.certificate_status())
            .field("responder", &self.responder())
            .field("revocation_reason", &self.revocation_reason())
            .field("subject", &self.subject())
            .finish()
    }
}

unsafe impl ExternType for QOcspResponse {
    type Id = type_id!("QOcspResponse");
    type Kind = cxx::kind::Trivial;
}

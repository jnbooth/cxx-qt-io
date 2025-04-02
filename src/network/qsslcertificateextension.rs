use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslcertificateextension.h");
    }

    unsafe extern "C++" {
        type QSslCertificateExtension = super::QSslCertificateExtension;

        /// Returns the criticality of the extension.
        #[rust_name = "is_critical"]
        fn isCritical(&self) -> bool;

        /// Returns `true` if this extension is supported. In this case, supported simply means that the structure of the `QVariant` returned by the `value()` accessor will remain unchanged between versions. Unsupported extensions can be freely used, however there is no guarantee that the returned data will have the same structure between versions.
        #[rust_name = "is_supported"]
        fn isSupported(&self) -> bool;

        /// Returns the name of the extension. If no name is known for the extension then the OID will be returned.
        fn name(&self) -> QString;

        /// Returns the ASN.1 OID of this extension.
        fn oid(&self) -> QString;

        /// Returns the value of the extension. The structure of the value returned depends on the extension type.
        fn value(&self) -> QVariant;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslcertificateextension_drop"]
        fn drop(extension: &mut QSslCertificateExtension);

        #[rust_name = "qsslcertificateextension_init_default"]
        fn construct() -> QSslCertificateExtension;
        #[rust_name = "qsslcertificateextension_clone"]
        fn construct(other: &QSslCertificateExtension) -> QSslCertificateExtension;
    }
}

/// The `QSslCertificateExtension` class provides an API for accessing the extensions of an X509 certificate.
///
/// Qt Documentation: [QSslCertificateExtension](https://doc.qt.io/qt-6/qsslcertificateextension.html#details)
#[repr(C)]
pub struct QSslCertificateExtension {
    _space: MaybeUninit<usize>,
}

impl Clone for QSslCertificateExtension {
    fn clone(&self) -> Self {
        ffi::qsslcertificateextension_clone(self)
    }
}

impl Drop for QSslCertificateExtension {
    fn drop(&mut self) {
        ffi::qsslcertificateextension_drop(self);
    }
}

impl Default for QSslCertificateExtension {
    fn default() -> Self {
        ffi::qsslcertificateextension_init_default()
    }
}

unsafe impl ExternType for QSslCertificateExtension {
    type Id = type_id!("QSslCertificateExtension");
    type Kind = cxx::kind::Trivial;
}

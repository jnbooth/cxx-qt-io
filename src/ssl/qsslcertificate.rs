use std::fmt;
use std::iter::FusedIterator;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr;

use cxx::{type_id, ExternType};
use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QByteArray, QDateTime, QList, QString, QStringList};

use crate::util::{unpin_for_qt, upcast_mut, IsNonNull};
use crate::{
    DecodeSslKeyError, QCryptographicHashAlgorithm, QIODevice, QSslAlternativeNameEntryType,
    QSslEncodingFormat, QSslError, QSslKey,
};

#[cxx::bridge]
mod ffi {
    /// The syntax used to interpret the meaning of the pattern.
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum QSslCertificatePatternSyntax {
        /// A rich Perl-like pattern matching syntax.
        RegularExpression,
        /// This provides a simple pattern matching syntax similar to that used by shells (command interpreters) for "file globbing". See [`QRegularExpression::from_wildcard`](https://doc.qt.io/qt-6/qregularexpression.html#fromWildcard).
        Wildcard,
        /// The pattern is a fixed string. This is equivalent to using the RegularExpression pattern on a string in which all metacharacters are escaped using [`QRegularExpression::escape`](https://doc.qt.io/qt-6/qregularexpression.html#escape). This is the default.
        FixedString,
    }

    /// Describes keys that you can pass to [`QSslCertificate::issuer_info`] or [`QSslCertificate::subject_info`] to get information about the certificate issuer or subject.
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum QSslCertificateSubjectInfo {
        /// "O" The name of the organization.
        Organization,
        /// "CN" The common name; most often this is used to store the host name.
        CommonName,
        /// "L" The locality.
        LocalityName,
        /// "OU" The organizational unit name.
        OrganizationalUnitName,
        /// "C" The country.
        CountryName,
        /// "ST" The state or province.
        StateOrProvinceName,
        /// The distinguished name qualifier.
        DistinguishedNameQualifier,
        /// The certificate's serial number.
        SerialNumber,
        /// The email address associated with the certificate.
        EmailAddress,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = cxx_qt_lib::QDateTime;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;
        include!("cxx-qt-lib/qtypes.h");
        type qsizetype = cxx_qt_lib::qsizetype;
        include!("cxx-qt-lib/qlist.h");
        type QList_QByteArray = cxx_qt_lib::QList<QByteArray>;

        include!("cxx-qt-io/qcryptographichash.h");
        type QCryptographicHashAlgorithm = crate::QCryptographicHashAlgorithm;
        include!("cxx-qt-io/qiodevice.h");
        type QIODevice = crate::QIODevice;
        include!("cxx-qt-io/qssl.h");
        type QSslEncodingFormat = crate::QSslEncodingFormat;
        include!("cxx-qt-io/qsslkey.h");
        type QSslKey = crate::QSslKey;
        include!("cxx-qt-io/qlist.h");
        type QList_QSslCertificate = cxx_qt_lib::QList<QSslCertificate>;
        type QList_QSslCertificateExtension = cxx_qt_lib::QList<crate::QSslCertificateExtension>;
        type QList_QSslError = cxx_qt_lib::QList<crate::QSslError>;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslcertificate.h");
        type QSslCertificatePatternSyntax;
        type QSslCertificateSubjectInfo;
        type QSslAlternativeNameEntryType = crate::QSslAlternativeNameEntryType;
    }

    unsafe extern "C++" {
        type SubjectAlternativeNamesKeys = super::SubjectAlternativeNamesKeys;

        #[doc(hidden)]
        fn advance(&mut self) -> bool;
        /// # Safety
        ///
        /// You must first call [advance](Self::advance) and ensure it returns `true`.
        #[doc(hidden)]
        unsafe fn key(&self) -> &QSslAlternativeNameEntryType;
    }

    unsafe extern "C++" {
        type SubjectAlternativeNamesIter = super::SubjectAlternativeNamesIter;

        #[doc(hidden)]
        fn advance(&mut self) -> bool;
        /// # Safety
        ///
        /// You must first call [advance](Self::advance) and ensure it returns `true`.
        #[doc(hidden)]
        unsafe fn key(&self) -> &QSslAlternativeNameEntryType;
        /// # Safety
        ///
        /// You must first call [advance](Self::advance) and ensure it returns `true`.
        #[doc(hidden)]
        unsafe fn value(&self) -> &QString;
    }

    unsafe extern "C++" {
        type SubjectAlternativeNamesValues = super::SubjectAlternativeNamesValues;

        #[doc(hidden)]
        fn advance(&mut self) -> bool;
        /// # Safety
        ///
        /// You must first call [advance](Self::advance) and ensure it returns `true`.
        #[doc(hidden)]
        unsafe fn value(&self) -> &QString;
    }

    unsafe extern "C++" {
        type SubjectAlternativeNamesMap = super::SubjectAlternativeNamesMap;

        /// Returns `true` if the multi map contains an item with key `key`; otherwise returns `false`.
        fn contains(&self, key: &QSslAlternativeNameEntryType) -> bool;

        #[doc(hidden)]
        #[rust_name = "count_entries_qsizetype"]
        fn count(&self, key: &QSslAlternativeNameEntryType, value: &QString) -> qsizetype;

        #[doc(hidden)]
        #[rust_name = "count_values_qsizetype"]
        fn count(&self, key: &QSslAlternativeNameEntryType) -> qsizetype;

        /// Returns `true` if the multi map contains no items; otherwise returns `false`.
        #[rust_name = "is_empty"]
        fn isEmpty(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "len_qsizetype"]
        fn size(&self) -> qsizetype;
    }

    unsafe extern "C++" {
        type QSslCertificate = super::QSslCertificate;

        /// Clears the contents of this certificate, making it a null certificate.
        fn clear(&mut self);

        /// Returns a cryptographic digest of this certificate using `algorithm`.
        fn digest(&self, algorithm: QCryptographicHashAlgorithm) -> QByteArray;

        #[doc(hidden)]
        #[rust_name = "effective_date_or_null"]
        fn effectiveDate(&self) -> QDateTime;

        #[doc(hidden)]
        #[rust_name = "expiry_date_or_null"]
        fn expiryDate(&self) -> QDateTime;

        /// Returns a list containing the X509 extensions of this certificate.
        fn extensions(&self) -> QList_QSslCertificateExtension;

        /// Returns `true` if this certificate is blacklisted; otherwise returns `false`.
        #[rust_name = "is_blacklisted"]
        fn isBlacklisted(&self) -> bool;

        /// Returns `true` if this is a null certificate (i.e., a certificate with no contents); otherwise returns `false`.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        /// Returns `true` if this certificate is self signed; otherwise returns `false`.
        ///
        /// A certificate is considered self-signed its issuer and subject are identical.
        #[rust_name = "is_self_signed"]
        fn isSelfSigned(&self) -> bool;

        /// Returns a name that describes the issuer. It returns the `CommonName` if available, otherwise falls back to the first `Organization` or the first `OrganizationalUnitName`.
        #[rust_name = "issuer_display_name"]
        fn issuerDisplayName(&self) -> QString;

        #[doc(hidden)]
        #[rust_name = "issuer_info_by_subject"]
        fn issuerInfo(&self, subject: QSslCertificateSubjectInfo) -> QStringList;
        #[doc(hidden)]
        #[rust_name = "issuer_info_by_attribute"]
        fn issuerInfo(&self, attribute: &QByteArray) -> QStringList;

        /// Returns a list of the attributes that have values in the issuer information of this certificate. The information associated with a given attribute can be accessed using the [`subject_info`](QSslCertificate::subject_info) method. Note that this list may include the OIDs for any elements that are not known by the SSL backend.
        #[rust_name = "issuer_info_attributes"]
        fn issuerInfoAttributes(&self) -> QList_QByteArray;

        /// Returns the certificate subject's public key.
        #[rust_name = "public_key"]
        fn publicKey(&self) -> QSslKey;

        /// Returns the certificate's serial number string in hexadecimal format.
        #[rust_name = "serial_number"]
        fn serialNumber(&self) -> QByteArray;

        /// Returns the list of alternative subject names for this certificate. The alternative names typically contain host names, optionally with wildcards, that are valid for this certificate.
        ///
        /// These names are tested against the connected peer's host name, if either the subject information for [`QSslCertificateSubjectInfo::CommonName`] doesn't define a valid host name, or the subject info name doesn't match the peer's host name.
        #[rust_name = "subject_alternative_names"]
        fn subjectAlternativeNames(&self) -> SubjectAlternativeNamesMap;

        /// Returns a name that describes the subject. It returns the `CommonName` if available, otherwise falls back to the first `Organization` or the first `OrganizationalUnitName`.
        #[rust_name = "subject_display_name"]
        fn subjectDisplayName(&self) -> QString;

        #[doc(hidden)]
        #[rust_name = "subject_info_by_subject"]
        fn subjectInfo(&self, subject: QSslCertificateSubjectInfo) -> QStringList;
        #[doc(hidden)]
        #[rust_name = "subject_info_by_attribute"]
        fn subjectInfo(&self, attribute: &QByteArray) -> QStringList;

        /// Returns a list of the attributes that have values in the subject information of this certificate. The information associated with a given attribute can be accessed using the [`subject_info`](QSslCertificate::subject_info) method. Note that this list may include the OIDs for any elements that are not known by the SSL backend.
        #[rust_name = "subject_info_attributes"]
        fn subjectInfoAttributes(&self) -> QList_QByteArray;

        /// Returns this certificate converted to a DER (binary) encoded representation.
        #[rust_name = "to_der"]
        fn toDer(&self) -> QByteArray;

        /// Returns this certificate converted to a PEM (Base64) encoded representation.
        #[rust_name = "to_pem"]
        fn toPem(&self) -> QByteArray;

        /// Returns this certificate converted to a human-readable text representation.
        #[rust_name = "to_text"]
        fn toText(&self) -> QString;

        /// Returns the certificate's version string.
        fn version(&self) -> QByteArray;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "subjectalternativenamesmap_find"]
        fn qmultimapFind(
            map: &SubjectAlternativeNamesMap,
            key: &QSslAlternativeNameEntryType,
        ) -> SubjectAlternativeNamesValues;

        #[rust_name = "subjectalternativenamesmap_keys"]
        fn qmultimapKeys(map: &SubjectAlternativeNamesMap) -> SubjectAlternativeNamesKeys;

        #[rust_name = "subjectalternativenamesmap_iter"]
        fn qmultimapIter(map: &SubjectAlternativeNamesMap) -> SubjectAlternativeNamesIter;

        #[rust_name = "qsslcertificate_from_data"]
        fn qsslcertificateFromData(
            data: &QByteArray,
            format: QSslEncodingFormat,
        ) -> QList_QSslCertificate;

        /// # Safety
        ///
        /// `device` must be valid.
        #[rust_name = "qsslcertificate_from_device"]
        unsafe fn qsslcertificateFromDevice(
            device: *mut QIODevice,
            format: QSslEncodingFormat,
        ) -> QList_QSslCertificate;

        #[rust_name = "qsslcertificate_from_path"]
        fn qsslcertificateFromPath(
            path: &QString,
            format: QSslEncodingFormat,
            syntax: QSslCertificatePatternSyntax,
        ) -> QList_QSslCertificate;

        /// # Safety
        ///
        /// All pointers must be valid.
        #[rust_name = "qsslcertificate_import_pkcs_12"]
        unsafe fn qsslcertificateImportPkcs12(
            device: *mut QIODevice,
            key: *mut QSslKey,
            certificate: *mut QSslCertificate,
            ca_certificates: *mut QList_QSslCertificate,
            pass_phrase: &QByteArray,
        ) -> bool;

        #[rust_name = "qsslcertificate_verify"]
        fn qsslcertificateVerify(
            certificates: &QList_QSslCertificate,
            host_name: &QString,
        ) -> QList_QSslError;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        include!("cxx-qt-io/qbytearray.h");

        #[rust_name = "qbytearray_to_base64"]
        fn qbytearrayToBase64(bytes: &QByteArray) -> QByteArray;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslcertificate_drop"]
        fn drop(certificate: &mut QSslCertificate);

        #[rust_name = "qsslcertificate_init_device"]
        unsafe fn construct(device: *mut QIODevice, format: QSslEncodingFormat) -> QSslCertificate;
        #[rust_name = "qsslcertificate_init_data"]
        fn construct(data: &QByteArray, format: QSslEncodingFormat) -> QSslCertificate;
        #[rust_name = "qsslcertificate_clone"]
        fn construct(other: &QSslCertificate) -> QSslCertificate;

        #[rust_name = "qsslcertificate_eq"]
        fn operatorEq(a: &QSslCertificate, b: &QSslCertificate) -> bool;
    }
}

pub use ffi::{QSslCertificatePatternSyntax, QSslCertificateSubjectInfo};

/// Parameter for [`QSslCertificate`] functions that reference certificate information.
///
/// Functions that accept `SslCertificateSubjectInfoOrAttribute` are overloaded to accept either [`QSslCertificateSubjectInfo`] or [`&QByteArray`](cxx_qt_lib::QByteArray). You do not need to use this type directly.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SslCertificateSubjectInfoOrAttribute<'a> {
    /// Specify by subject info type.
    Subject(QSslCertificateSubjectInfo),
    /// Specify by certificate attribute.
    Attribute(&'a QByteArray),
}

impl From<QSslCertificateSubjectInfo> for SslCertificateSubjectInfoOrAttribute<'static> {
    fn from(value: QSslCertificateSubjectInfo) -> Self {
        Self::Subject(value)
    }
}

impl<'a> From<&'a QByteArray> for SslCertificateSubjectInfoOrAttribute<'a> {
    fn from(value: &'a QByteArray) -> Self {
        Self::Attribute(value)
    }
}

/// The `QSslCertificate` class provides a convenient API for an X509 certificate.
///
/// Qt Documentation: [QSslCertificate](https://doc.qt.io/qt-6/qsslcertificate.html#details)
#[repr(C)]
pub struct QSslCertificate {
    _space: MaybeUninit<usize>,
}

impl Clone for QSslCertificate {
    fn clone(&self) -> Self {
        ffi::qsslcertificate_clone(self)
    }
}

impl Drop for QSslCertificate {
    fn drop(&mut self) {
        ffi::qsslcertificate_drop(self);
    }
}

impl PartialEq for QSslCertificate {
    fn eq(&self, other: &Self) -> bool {
        ffi::qsslcertificate_eq(self, other)
    }
}

impl Eq for QSslCertificate {}

impl fmt::Debug for QSslCertificate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QSslCertificate")
            .field("version", &self.version())
            .field("serial_number", &self.serial_number())
            .field(
                "digest",
                &ffi::qbytearray_to_base64(&self.digest(QCryptographicHashAlgorithm::Md5)),
            )
            .field("issuer", &self.issuer_display_name())
            .field("subject", &self.subject_display_name())
            .field(
                "subject_alternative_names",
                &self.subject_alternative_names(),
            )
            .field("effective_date", &self.effective_date_or_null())
            .field("expiry_date", &self.expiry_date_or_null())
            .finish()
    }
}

impl fmt::Display for QSslCertificate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_text().fmt(f)
    }
}

impl IsNonNull for QSslCertificate {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

impl QSslCertificate {
    /// Imports a PKCS#12 (pfx) file from the specified device. A PKCS#12 file is a bundle that can contain a number of certificates and keys. This method reads a single `key`, its `certificate` and any associated `ca_certificates` from the bundle. If a `pass_phrase` is specified then this will be used to decrypt the bundle. Returns `true` if the PKCS#12 file was successfully loaded.
    ///
    /// **Note:** The `device` must be open and ready to be read from.
    pub fn import_pkcs_12<T>(
        device: Pin<&mut T>,
        key: &QSslKey,
        certificate: &QSslCertificate,
        ca_certificates: &QList<Self>,
        pass_phrase: Option<&QByteArray>,
    ) -> bool
    where
        T: Upcast<QIODevice>,
    {
        // Reduce monomorphization
        unsafe fn inner(
            device: *mut QIODevice,
            key: &QSslKey,
            certificate: &QSslCertificate,
            ca_certificates: &QList<QSslCertificate>,
            pass_phrase: Option<&QByteArray>,
        ) -> bool {
            let key_mut = ptr::from_ref(key).cast_mut();
            let certificate_mut = ptr::from_ref(certificate).cast_mut();
            let ca_certificates_mut = ptr::from_ref(ca_certificates).cast_mut();
            // SAFETY: from outer
            unsafe {
                match pass_phrase {
                    Some(pass_phrase) => ffi::qsslcertificate_import_pkcs_12(
                        device,
                        key_mut,
                        certificate_mut,
                        ca_certificates_mut,
                        pass_phrase,
                    ),
                    None => ffi::qsslcertificate_import_pkcs_12(
                        device,
                        key_mut,
                        certificate_mut,
                        ca_certificates_mut,
                        &QByteArray::default(),
                    ),
                }
            }
        }
        // SAFETY: Qt knows what it's doing.
        unsafe {
            inner(
                upcast_mut(unpin_for_qt(device)),
                key,
                certificate,
                ca_certificates,
                pass_phrase,
            )
        }
    }

    /// Verifies a certificate chain. The chain to be verified is passed in the `certificate_chain` parameter. The first certificate in the list should be the leaf certificate of the chain to be verified. If `host_name` is specified then the certificate is also checked to see if it is valid for the specified host name.
    ///
    /// Note that the root (CA) certificate should not be included in the list to be verified, this will be looked up automatically using the CA list specified in the default [`QSslConfiguration`](crate::QSslConfiguration), and, in addition, if possible, CA certificates loaded on demand on Unix and Windows.
    pub fn verify(
        certificate_chain: &QList<QSslCertificate>,
        host_name: Option<&QString>,
    ) -> QList<QSslError> {
        match host_name {
            Some(host_name) => ffi::qsslcertificate_verify(certificate_chain, host_name),
            None => ffi::qsslcertificate_verify(certificate_chain, &QString::default()),
        }
    }

    /// Constructs a `QSslCertificate` by parsing the `format` encoded `data` and using the first available certificate found.
    ///
    /// Returns an error if `data` did not contain a certificate or the certificate was not loaded successfully.
    pub fn first_from_data(
        data: &QByteArray,
        format: QSslEncodingFormat,
    ) -> Result<Self, DecodeSslKeyError> {
        ffi::qsslcertificate_init_data(data, format).nonnull_or(DecodeSslKeyError(()))
    }

    /// Constructs a `QSslCertificate` by reading `format` encoded data from `device` and using the first certificate found.
    ///
    /// Returns an error if `device` did not contain a certificate or the certificate was not loaded successfully.
    pub fn first_from_device<T>(
        device: Pin<&mut T>,
        format: QSslEncodingFormat,
    ) -> Result<Self, DecodeSslKeyError>
    where
        T: Upcast<QIODevice>,
    {
        // SAFETY: `unpin_for_qt(device)` is passed directly to Qt.
        unsafe { ffi::qsslcertificate_init_device(upcast_mut(unpin_for_qt(device)), format) }
            .nonnull_or(DecodeSslKeyError(()))
    }

    /// Searches for and parses all certificates in `data` that are encoded in the specified `format` and returns them in a list of certificates.
    pub fn from_data(data: &QByteArray, format: QSslEncodingFormat) -> QList<Self> {
        ffi::qsslcertificate_from_data(data, format)
    }

    /// Searches for and parses all certificates in `device` that are encoded in the specified `format` and returns them in a list of certificates.
    pub fn from_device<T>(device: Pin<&mut T>, format: QSslEncodingFormat) -> QList<Self>
    where
        T: Upcast<QIODevice>,
    {
        // SAFETY: `unpin_for_qt(device)` is passed directly to Qt.
        unsafe { ffi::qsslcertificate_from_device(upcast_mut(unpin_for_qt(device)), format) }
    }

    /// Searches all files in the `path` for certificates encoded in the specified `format` and returns them in a list. `path` must be a file or a pattern matching one or more files, as specified by `syntax`.
    pub fn from_path<T>(
        path: &QString,
        format: QSslEncodingFormat,
        syntax: QSslCertificatePatternSyntax,
    ) -> QList<Self>
    where
        T: Upcast<QIODevice>,
    {
        ffi::qsslcertificate_from_path(path, format, syntax)
    }

    /// Returns the date-time that the certificate becomes valid, or `None` if this is a null certificate.
    pub fn effective_date(&self) -> Option<QDateTime> {
        self.effective_date_or_null().nonnull()
    }

    /// Returns the date-time that the certificate expires, or `None` if this is a null certificate.
    pub fn expiry_date(&self) -> Option<QDateTime> {
        self.expiry_date_or_null().nonnull()
    }

    /// Returns the issuer information for the `subject`, or an empty list if there is no information for `subject` in the certificate. There can be more than one entry of each type.
    pub fn issuer_info<'a, T>(&self, subject: T) -> QStringList
    where
        T: Into<SslCertificateSubjectInfoOrAttribute<'a>>,
    {
        match subject.into() {
            SslCertificateSubjectInfoOrAttribute::Attribute(attribute) => {
                self.issuer_info_by_attribute(attribute)
            }
            SslCertificateSubjectInfoOrAttribute::Subject(subject) => {
                self.issuer_info_by_subject(subject)
            }
        }
    }

    /// Returns the information for the `subject`, or an empty list if there is no information for `subject` in the certificate. There can be more than one entry of each type.
    pub fn subject_info<'a, T>(&self, subject: T) -> QStringList
    where
        T: Into<SslCertificateSubjectInfoOrAttribute<'a>>,
    {
        match subject.into() {
            SslCertificateSubjectInfoOrAttribute::Attribute(attribute) => {
                self.subject_info_by_attribute(attribute)
            }
            SslCertificateSubjectInfoOrAttribute::Subject(subject) => {
                self.subject_info_by_subject(subject)
            }
        }
    }
}

impl TryFrom<&QByteArray> for QSslCertificate {
    type Error = DecodeSslKeyError;

    /// Constructs a `QSslCertificate` by parsing the PEM-encoded `data` and using the first available certificate found.
    ///
    /// Returns an error if `data` did not contain a certificate or the certificate was not loaded successfully.
    fn try_from(data: &QByteArray) -> Result<Self, Self::Error> {
        Self::first_from_data(data, QSslEncodingFormat::Pem)
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QSslCertificate {
    type Id = type_id!("QSslCertificate");
    type Kind = cxx::kind::Trivial;
}

impl SubjectAlternativeNamesMap {
    /// Returns the number of items with key `key` and value `value`.
    pub fn count_entries(&self, key: QSslAlternativeNameEntryType, value: &QString) -> isize {
        self.count_entries_qsizetype(&key, value).into()
    }

    /// Returns the number of items with key `key`.
    pub fn count_values(&self, key: QSslAlternativeNameEntryType) -> isize {
        self.count_values_qsizetype(&key).into()
    }

    pub fn find(&self, key: QSslAlternativeNameEntryType) -> SubjectAlternativeNamesValues {
        ffi::subjectalternativenamesmap_find(self, &key)
    }

    /// Returns the number of items.
    pub fn len(&self) -> isize {
        self.len_qsizetype().into()
    }

    /// Returns an iterator containing all the keys in the multi map in ascending order. Keys that occur multiple times in the multi map also occur multiple times in the list.
    pub fn keys(&self) -> SubjectAlternativeNamesKeys {
        ffi::subjectalternativenamesmap_keys(self)
    }
}

impl IntoIterator for &SubjectAlternativeNamesMap {
    type Item = (QSslAlternativeNameEntryType, QString);

    type IntoIter = SubjectAlternativeNamesIter;

    fn into_iter(self) -> Self::IntoIter {
        ffi::subjectalternativenamesmap_iter(self)
    }
}

#[repr(C)]
pub struct SubjectAlternativeNamesKeys {
    _iter: MaybeUninit<usize>,
    _end: MaybeUninit<usize>,
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for SubjectAlternativeNamesKeys {
    type Id = type_id!("SubjectAlternativeNamesKeys");
    type Kind = cxx::kind::Trivial;
}

impl Iterator for SubjectAlternativeNamesKeys {
    type Item = QSslAlternativeNameEntryType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.advance() {
            // SAFETY: `self.advance()` returned true, so the iterator is valid.
            Some(unsafe { *self.key() })
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct SubjectAlternativeNamesIter {
    _iter: MaybeUninit<usize>,
    _end: MaybeUninit<usize>,
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for SubjectAlternativeNamesIter {
    type Id = type_id!("SubjectAlternativeNamesIter");
    type Kind = cxx::kind::Trivial;
}

impl Iterator for SubjectAlternativeNamesIter {
    type Item = (QSslAlternativeNameEntryType, QString);

    fn next(&mut self) -> Option<Self::Item> {
        if self.advance() {
            // SAFETY: `self.advance()` returned true, so the iterator is valid.
            Some(unsafe { (*self.key(), self.value().clone()) })
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct SubjectAlternativeNamesValues {
    _iter: MaybeUninit<usize>,
    _end: MaybeUninit<usize>,
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for SubjectAlternativeNamesValues {
    type Id = type_id!("SubjectAlternativeNamesValues");
    type Kind = cxx::kind::Trivial;
}

impl Iterator for SubjectAlternativeNamesValues {
    type Item = QString;

    fn next(&mut self) -> Option<Self::Item> {
        if self.advance() {
            // SAFETY: `self.advance()` returned true, so the iterator is valid.
            Some(unsafe { self.value().clone() })
        } else {
            None
        }
    }
}

impl FusedIterator for SubjectAlternativeNamesValues {}

#[repr(C)]
pub struct SubjectAlternativeNamesMap {
    _space: MaybeUninit<usize>,
}

impl SubjectAlternativeNamesMap {
    pub fn iter(&self) -> SubjectAlternativeNamesIter {
        ffi::subjectalternativenamesmap_iter(self)
    }
}

impl fmt::Debug for SubjectAlternativeNamesMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for SubjectAlternativeNamesMap {
    type Id = type_id!("SubjectAlternativeNamesMap");
    type Kind = cxx::kind::Trivial;
}

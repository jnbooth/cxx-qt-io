use std::fmt;
use std::mem::MaybeUninit;

use cxx::{ExternType, type_id};
use cxx_qt_lib::QString;

use crate::QSslSslProtocol;
use crate::util::IsNonNull;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[namespace = "rust::cxxqtio1"]
    extern "C++" {
        include!("cxx-qt-io/qssl.h");
        type QSslSslProtocol = crate::QSslSslProtocol;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslcipher.h");
    }

    unsafe extern "C++" {
        type QSslCipher = super::QSslCipher;

        /// Returns the cipher's authentication method.
        #[rust_name = "authentication_method"]
        fn authenticationMethod(&self) -> QString;

        /// Returns the cipher's encryption method.
        #[rust_name = "encryption_method"]
        fn encryptionMethod(&self) -> QString;

        /// Returns `true` if this is a null cipher; otherwise returns `false`.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        /// Returns the name of the cipher, or an empty `QString` if this is a null cipher.
        fn name(&self) -> QString;

        /// Returns the cipher's protocol type, or [`QSslSslProtocol::UnknownProtocol`] if `QSslCipher` is unable to determine the protocol ([`self.protocol_string()`](QSslCipher::protocol_string) may contain more information).
        fn protocol(&self) -> QSslSslProtocol;

        /// Returns the cipher's protocol as a `QString`.
        #[rust_name = "protocol_string"]
        fn protocolString(&self) -> QString;

        /// Returns the number of bits supported by the cipher.
        #[rust_name = "supported_bits"]
        fn supportedBits(&self) -> i32;

        /// Returns the number of bits used by the cipher.
        #[rust_name = "used_bits"]
        fn usedBits(&self) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslcipher_drop"]
        fn drop(extension: &mut QSslCipher);

        #[rust_name = "qsslcipher_init_default"]
        fn construct() -> QSslCipher;
        #[rust_name = "qsslcipher_init_name"]
        fn construct(name: &QString) -> QSslCipher;
        #[rust_name = "qsslcipher_init_protocol"]
        fn construct(name: &QString, protocol: QSslSslProtocol) -> QSslCipher;
        #[rust_name = "qsslcipher_clone"]
        fn construct(other: &QSslCipher) -> QSslCipher;

        #[rust_name = "qsslcipher_eq"]
        fn operatorEq(a: &QSslCipher, b: &QSslCipher) -> bool;
    }
}

impl IsNonNull for QSslCipher {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

/// The `QSslCipher` class represents an SSL cryptographic cipher.
///
/// Qt Documentation: [QSslCipher](https://doc.qt.io/qt-6/qsslcipher.html#details)
#[repr(C)]
pub struct QSslCipher {
    _space: MaybeUninit<usize>,
}

impl Clone for QSslCipher {
    fn clone(&self) -> Self {
        ffi::qsslcipher_clone(self)
    }
}

impl Drop for QSslCipher {
    fn drop(&mut self) {
        ffi::qsslcipher_drop(self);
    }
}

impl Default for QSslCipher {
    /// Constructs an empty `QSslCipher` object.
    fn default() -> Self {
        ffi::qsslcipher_init_default()
    }
}

impl PartialEq for QSslCipher {
    fn eq(&self, other: &Self) -> bool {
        ffi::qsslcipher_eq(self, other)
    }
}

impl Eq for QSslCipher {}

impl fmt::Debug for QSslCipher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QSslCipher")
            .field("name", &self.name())
            .field("bits", &self.used_bits())
            .field("protocol_string", &self.protocol_string())
            .finish()
    }
}

impl QSslCipher {
    /// Constructs a `QSslCipher` object for the cipher determined by `name` and `protocol`. The constructor accepts only supported ciphers (i.e., the name and protocol (if supplied) must identify a cipher in the list of ciphers returned by [`QSslConfiguration::supported_ciphers()`](crate::QSslConfiguration::supported_ciphers)).
    ///
    /// To construct a cipher without specifying a protocol, use `QSslCipher::from(name)`.
    ///
    /// Returns an error if `name` and `protocol` do not correctly identify a supported cipher.
    pub fn new(name: &QString, protocol: QSslSslProtocol) -> Result<Self, QSslCipherError> {
        ffi::qsslcipher_init_protocol(name, protocol).nonnull_or(QSslCipherError(()))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct QSslCipherError(pub(crate) ());

impl fmt::Display for QSslCipherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SSL name does not correctly identify a supported cipher")
    }
}

impl TryFrom<&QString> for QSslCipher {
    type Error = QSslCipherError;

    /// Constructs a `QSslCipher` object for the cipher determined by `name` and `protocol`. The constructor accepts only supported ciphers (i.e., the name and protocol (if supplied) must identify a cipher in the list of ciphers returned by [`QSslConfiguration::supported_ciphers()`](crate::QSslConfiguration::supported_ciphers)).
    ///
    /// To specify a protocol as well as a name, use [`QSslCipher::new`].
    ///
    /// Returns an error if `name` and `protocol` do not correctly identify a supported cipher.
    fn try_from(value: &QString) -> Result<Self, Self::Error> {
        ffi::qsslcipher_init_name(value).nonnull_or(QSslCipherError(()))
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QSslCipher {
    type Id = type_id!("QSslCipher");
    type Kind = cxx::kind::Trivial;
}

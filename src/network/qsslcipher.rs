use std::fmt::{self, Debug, Formatter};
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};
use cxx_qt_lib::QString;

use crate::util::Valid;
use crate::QSslSslProtocol;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
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

        /// Returns the cipher's protocol type, or `UnknownProtocol` if `QSslCipher` is unable to determine the protocol (`protocol_string()` may contain more information).
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

        #[rust_name = "qsslcipher_to_debug_qstring"]
        fn toDebugQString(value: &QSslCipher) -> QString;
    }
}

impl Valid for QSslCipher {
    fn is_valid(value: &Self) -> bool {
        !value.is_null()
    }
}

/// This class represents Online Certificate Status Protocol response.
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

impl Debug for QSslCipher {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qsslcipher_to_debug_qstring(self))
    }
}

impl QSslCipher {
    /// Constructs a `QSslCipher` object for the cipher determined by `name` and optional `protocol`. The constructor accepts only supported ciphers (i.e., the name and protocol (if supplied) must identify a cipher in the list of ciphers returned by `QSslSocket::supported_ciphers()`).
    ///
    /// Returns `None` if `name` and `protocol` do not correctly identify a supported cipher.
    pub fn new(name: &QString, protocol: Option<QSslSslProtocol>) -> Option<Self> {
        match protocol {
            Some(protocol) => ffi::qsslcipher_init_protocol(name, protocol),
            None => ffi::qsslcipher_init_name(name),
        }
        .valid()
    }
}

unsafe impl ExternType for QSslCipher {
    type Id = type_id!("QSslCipher");
    type Kind = cxx::kind::Trivial;
}

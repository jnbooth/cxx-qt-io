use std::any::Any;
use std::fmt;
use std::mem::MaybeUninit;
use std::pin::Pin;

use cxx::{ExternType, type_id};
use cxx_qt::casting::Upcast;
use cxx_qt_lib::QByteArray;

use crate::util::{IsNonNull, unpin_for_qt, upcast_mut};
use crate::{QIODevice, QSslEncodingFormat, QSslKeyAlgorithm, QSslKeyType};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;

        include!("cxx-qt-io/qiodevice.h");
        type QIODevice = crate::QIODevice;
    }

    #[namespace = "rust::cxxqtio1"]
    extern "C++" {
        include!("cxx-qt-io/qssl.h");
        type QSslKeyAlgorithm = crate::QSslKeyAlgorithm;
        type QSslEncodingFormat = crate::QSslEncodingFormat;
        type QSslKeyType = crate::QSslKeyType;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslkey.h");
    }

    unsafe extern "C++" {
        type QSslKey = super::QSslKey;

        /// Returns the key algorithm.
        fn algorithm(&self) -> QSslKeyAlgorithm;

        /// Clears the contents of this key, making it a null key.
        fn clear(&mut self);

        /// Returns `true` if this is a null key; otherwise `false`.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "len_or_negative"]
        fn length(&self) -> i32;

        /// Returns the type of the key (i.e., [`QSslKeyType::PublicKey`] or [`QSslKeyType::PrivateKey`]).
        #[cxx_name = "type"]
        fn key_type(&self) -> QSslKeyType;

        /// Returns the key in PEM encoding. The result is encrypted with `pass_phrase` if the key is a private key.
        #[rust_name = "to_pem_encrypted"]
        fn toPem(&self, pass_phrase: &QByteArray) -> QByteArray;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qsslkey_to_der"]
        fn qsslkeyToDer(key: &QSslKey) -> QByteArray;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslkey_drop"]
        fn drop(datagram: &mut QSslKey);

        #[rust_name = "qsslkey_init_default"]
        fn construct() -> QSslKey;
        #[rust_name = "qsslkey_init_device"]
        unsafe fn construct(
            device: *mut QIODevice,
            algorithm: QSslKeyAlgorithm,
            encoding: QSslEncodingFormat,
            key_type: QSslKeyType,
            pass_phrase: &QByteArray,
        ) -> QSslKey;
        #[rust_name = "qsslkey_init_data"]
        fn construct(
            data: &QByteArray,
            algorithm: QSslKeyAlgorithm,
            encoding: QSslEncodingFormat,
            key_type: QSslKeyType,
            pass_phrase: &QByteArray,
        ) -> QSslKey;
        #[rust_name = "qsslkey_clone"]
        fn construct(other: &QSslKey) -> QSslKey;

        #[rust_name = "qsslkey_eq"]
        fn operatorEq(a: &QSslKey, b: &QSslKey) -> bool;
    }
}

/// The `QSslKey` class provides an interface for private and public keys.
///
/// Qt Documentation: [QSslKey](https://doc.qt.io/qt-6/qsslkey.html#details)
#[repr(C)]
pub struct QSslKey {
    _space: MaybeUninit<usize>,
}

impl Clone for QSslKey {
    fn clone(&self) -> Self {
        ffi::qsslkey_clone(self)
    }
}

impl Default for QSslKey {
    /// Constructs a null key.
    fn default() -> Self {
        ffi::qsslkey_init_default()
    }
}

impl Drop for QSslKey {
    fn drop(&mut self) {
        ffi::qsslkey_drop(self);
    }
}

impl PartialEq for QSslKey {
    fn eq(&self, other: &Self) -> bool {
        ffi::qsslkey_eq(self, other)
    }
}

impl Eq for QSslKey {}

impl fmt::Debug for QSslKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QSslKey")
            .field("type", &self.type_id())
            .field("algorithm", &self.algorithm())
            .field("len", &self.len_or_negative())
            .finish()
    }
}

impl IsNonNull for QSslKey {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

impl QSslKey {
    fn into_result(self) -> Result<Self, DecodeSslKeyError> {
        if self.is_null() {
            Err(DecodeSslKeyError(()))
        } else {
            Ok(self)
        }
    }

    /// Constructs a `QSslKey` by reading and decoding data from a `device` using a specified `algorithm` and `encoding` format. `key_type` specifies whether the key is public or private.
    ///
    /// If the key is encrypted then `pass_phrase` is used to decrypt it.
    ///
    /// Returns an error if the device did not provide a valid key.
    pub fn from_device<T>(
        device: Pin<&mut T>,
        algorithm: QSslKeyAlgorithm,
        encoding: QSslEncodingFormat,
        key_type: QSslKeyType,
        pass_phrase: &QByteArray,
    ) -> Result<Self, DecodeSslKeyError>
    where
        T: Upcast<QIODevice>,
    {
        // SAFETY: `unpin_for_qt(device)` is passed directly to Qt.
        unsafe {
            ffi::qsslkey_init_device(
                upcast_mut(unpin_for_qt(device)),
                algorithm,
                encoding,
                key_type,
                pass_phrase,
            )
        }
        .into_result()
    }

    /// Constructs a `QSslKey` by decoding the string in the byte array `encoded` using a specified `algorithm` and `encoding` format. `key_type` specifies whether the key is public or private.
    ///
    /// If the key is encrypted then `pass_phrase` is used to decrypt it.
    ///
    /// Returns an error if `encoded` did not contain a valid key.
    pub fn from_data(
        encoded: &QByteArray,
        algorithm: QSslKeyAlgorithm,
        encoding: QSslEncodingFormat,
        key_type: QSslKeyType,
        pass_phrase: &QByteArray,
    ) -> Result<Self, DecodeSslKeyError> {
        ffi::qsslkey_init_data(encoded, algorithm, encoding, key_type, pass_phrase).into_result()
    }

    /// Returns the length of the key in bits, or `None` if the key is null.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> Option<i32> {
        let len = self.len_or_negative();
        if len == -1 { None } else { Some(len) }
    }

    /// Returns the key in DER encoding.
    pub fn to_der(&self) -> QByteArray {
        ffi::qsslkey_to_der(self)
    }

    /// Returns the key in PEM encoding.
    pub fn to_pem(&self) -> QByteArray {
        self.to_pem_encrypted(&QByteArray::default())
    }
}

impl TryFrom<&QByteArray> for QSslKey {
    type Error = DecodeSslKeyError;

    /// Constructs a `QSslKey` by decoding the string in the byte array `encoded` using the RSA algorithm and PEM encoding format as a private key.
    ///
    /// Returns an error if `encoded` did not contain a valid key.
    fn try_from(encoded: &QByteArray) -> Result<Self, Self::Error> {
        Self::from_data(
            encoded,
            QSslKeyAlgorithm::Rsa,
            QSslEncodingFormat::Pem,
            QSslKeyType::PrivateKey,
            &QByteArray::default(),
        )
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QSslKey {
    type Id = type_id!("QSslKey");
    type Kind = cxx::kind::Trivial;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DecodeSslKeyError(pub(crate) ());

impl fmt::Display for DecodeSslKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unable to decode file data to SSL key or certificate")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonnull() {
        let key_data = include_bytes!("../../tests/local.key");
        let key = QSslKey::try_from(&QByteArray::from(key_data)).expect("unable to parse key data");
        assert_nonnull!(key, QSslKey::default());
    }
}

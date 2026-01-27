use std::fmt;
use std::mem::MaybeUninit;
use std::pin::Pin;

use cxx::{ExternType, type_id};
use cxx_qt::casting::Upcast;
use cxx_qt_lib::QByteArray;

use crate::util::{IsNonNull, unpin_for_qt, upcast_mut};
use crate::{QIODevice, QSslEncodingFormat};

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    #[namespace = "rust::cxxqtio1"]
    enum QSslDiffieHellmanParametersError {
        /// No error occurred.
        NoError,
        /// The given input data could not be used to construct a [`QSslDiffieHellmanParameters`] object.
        InvalidInputDataError,
        /// The Diffie-Hellman parameters are unsafe and should not be used.
        UnsafeParametersError,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-io/qiodevice.h");
        type QIODevice = crate::QIODevice;
    }

    #[namespace = "rust::cxxqtio1"]
    extern "C++" {
        include!("cxx-qt-io/qssl.h");
        type QSslEncodingFormat = crate::QSslEncodingFormat;
    }

    #[namespace = "rust::cxxqtio1"]
    extern "C++" {
        include!("cxx-qt-io/qssldiffiehellmanparameters.h");
        type QSslDiffieHellmanParametersError;
    }

    unsafe extern "C++" {
        type QSslDiffieHellmanParameters = super::QSslDiffieHellmanParameters;

        /// Returns the default `QSslDiffieHellmanParameters` used by [`QSslSocket`](crate::QSslSocket).
        #[Self = "QSslDiffieHellmanParameters"]
        #[rust_name = "default_parameters"]
        fn defaultParameters() -> QSslDiffieHellmanParameters;

        #[doc(hidden)]
        #[Self = "QSslDiffieHellmanParameters"]
        #[rust_name = "from_encoded_device_unsafe"]
        unsafe fn fromEncoded(
            device: *mut QIODevice,
            encoding: QSslEncodingFormat,
        ) -> QSslDiffieHellmanParameters;

        #[doc(hidden)]
        #[Self = "QSslDiffieHellmanParameters"]
        #[rust_name = "from_encoded_bytes"]
        fn fromEncoded(
            bytes: &QByteArray,
            encoding: QSslEncodingFormat,
        ) -> QSslDiffieHellmanParameters;

        /// Returns the error that caused the `QSslDiffieHellmanParameters` object to be invalid.
        fn error(&self) -> QSslDiffieHellmanParametersError;

        /// Returns a human-readable description of the error that caused the QSslDiffieHellmanParameters object to be invalid.
        #[rust_name = "error_string"]
        fn errorString(&self) -> QString;

        /// Returns `true` if this is a an empty `QSslDiffieHellmanParameters` instance.
        ///
        /// Setting an empty `QSslDiffieHellmanParameters` instance on a [`QSslSocket`](crate::QSslSocket)-based server will disable Diffie-Hellman key exchange.
        #[rust_name = "is_empty"]
        fn isEmpty(&self) -> bool;

        /// Returns `true` if this is a valid `QSslDiffieHellmanParameters`; otherwise `false`.
        ///
        /// This method should be used after constructing a `QSslDiffieHellmanParameters` object to determine its validity.
        ///
        /// If a `QSslDiffieHellmanParameters` object is not valid, you can use the [`error`](QSslDiffieHellmanParameters::error) method to determine what error prevented the object from being constructed.
        #[rust_name = "is_valid"]
        fn isValid(&self) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qssldiffiehellmanparameters_drop"]
        fn drop(extension: &mut QSslDiffieHellmanParameters);

        #[rust_name = "qssldiffiehellmanparameters_init_default"]
        fn construct() -> QSslDiffieHellmanParameters;
        #[rust_name = "qssldiffiehellmanparameters_clone"]
        fn construct(other: &QSslDiffieHellmanParameters) -> QSslDiffieHellmanParameters;

        #[rust_name = "qssldiffiehellmanparameters_eq"]
        fn operatorEq(a: &QSslDiffieHellmanParameters, b: &QSslDiffieHellmanParameters) -> bool;

        #[rust_name = "qssldiffiehellmanparameters_to_debug_qstring"]
        fn toDebugQString(value: &QSslDiffieHellmanParameters) -> QString;
    }
}

pub use ffi::QSslDiffieHellmanParametersError;

/// The `QSslDiffieHellmanParameters` class provides an interface for Diffie-Hellman parameters for servers.
///
/// Qt Documentation: [QSslDiffieHellmanParameters](https://doc.qt.io/qt-6/qssldiffiehellmanparameters.html#details)
#[repr(C)]
pub struct QSslDiffieHellmanParameters {
    _space: MaybeUninit<usize>,
}

impl Clone for QSslDiffieHellmanParameters {
    fn clone(&self) -> Self {
        ffi::qssldiffiehellmanparameters_clone(self)
    }
}

impl Drop for QSslDiffieHellmanParameters {
    fn drop(&mut self) {
        ffi::qssldiffiehellmanparameters_drop(self);
    }
}

impl Default for QSslDiffieHellmanParameters {
    /// Constructs an empty `QSslDiffieHellmanParameters` object.
    fn default() -> Self {
        ffi::qssldiffiehellmanparameters_init_default()
    }
}

impl PartialEq for QSslDiffieHellmanParameters {
    fn eq(&self, other: &Self) -> bool {
        ffi::qssldiffiehellmanparameters_eq(self, other)
    }
}

impl Eq for QSslDiffieHellmanParameters {}

impl fmt::Debug for QSslDiffieHellmanParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const TRIM_UNTIL: usize = "QSslDiffieHellmanParameters(".len();
        let debug = ffi::qssldiffiehellmanparameters_to_debug_qstring(self);
        let data = debug.as_slice();
        String::from_utf16_lossy(&data[TRIM_UNTIL..data.len() - 1]).fmt(f)
    }
}

impl IsNonNull for QSslDiffieHellmanParameters {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl QSslDiffieHellmanParameters {
    fn into_result(self) -> Result<Self, QSslDiffieHellmanParametersError> {
        let error = self.error();
        if error == QSslDiffieHellmanParametersError::NoError {
            Ok(self)
        } else {
            Err(error)
        }
    }

    /// Attempts to construct a `QSslDiffieHellmanParameters` object by reading from `device` in either PEM or DER form as specified by `encoding`.
    pub fn from_encoded_device<T>(
        device: Pin<&mut T>,
        encoding: QSslEncodingFormat,
    ) -> Result<Self, QSslDiffieHellmanParametersError>
    where
        T: Upcast<QIODevice>,
    {
        // SAFETY: `unpin_for_qt(device)` is passed directly to Qt.
        unsafe { Self::from_encoded_device_unsafe(upcast_mut(unpin_for_qt(device)), encoding) }
            .into_result()
    }

    /// Attempts to construct a `QSslDiffieHellmanParameters` object using the byte array `encoded` in either PEM or DER form as specified by `encoding`.
    pub fn from_encoded_data(
        encoded: &QByteArray,
        encoding: QSslEncodingFormat,
    ) -> Result<Self, QSslDiffieHellmanParametersError> {
        Self::from_encoded_bytes(encoded, encoding).into_result()
    }
}

impl TryFrom<&QByteArray> for QSslDiffieHellmanParameters {
    type Error = QSslDiffieHellmanParametersError;

    fn try_from(value: &QByteArray) -> Result<Self, Self::Error> {
        Self::from_encoded_data(value, QSslEncodingFormat::Pem)
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QSslDiffieHellmanParameters {
    type Id = type_id!("QSslDiffieHellmanParameters");
    type Kind = cxx::kind::Trivial;
}

impl fmt::Display for QSslDiffieHellmanParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonnull() {
        assert!(crate::util::IsNonNull::is_nonnull(
            &QSslDiffieHellmanParameters::default_parameters()
        ));
    }
}

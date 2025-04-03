use std::fmt::{self, Debug, Formatter};
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::ptr;

use cxx::{type_id, ExternType};
use cxx_qt::Upcast;
use cxx_qt_lib::QByteArray;

use crate::util::Valid;
use crate::{QIODevice, QSslEncodingFormat};

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum QSslDiffieHellmanParametersError {
        /// No error occurred.
        NoError,
        /// The given input data could not be used to construct a `QSslDiffieHellmanParameters` object.
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
        include!("cxx-qt-io/qssl.h");
        type QSslEncodingFormat = crate::QSslEncodingFormat;
    }

    extern "C++" {
        include!("cxx-qt-io/qssldiffiehellmanparameters.h");
        type QSslDiffieHellmanParametersError;
    }

    unsafe extern "C++" {
        type QSslDiffieHellmanParameters = super::QSslDiffieHellmanParameters;

        /// Returns the error that caused the `QSslDiffieHellmanParameters` object to be invalid.
        fn error(&self) -> QSslDiffieHellmanParametersError;

        /// Returns a human-readable description of the error that caused the QSslDiffieHellmanParameters object to be invalid.
        #[rust_name = "error_string"]
        fn errorString(&self) -> QString;

        /// Returns `true` if this is a an empty `QSslDiffieHellmanParameters` instance.
        ///
        /// Setting an empty `QSslDiffieHellmanParameters` instance on a `QSslSocket`-based server will disable Diffie-Hellman key exchange.
        #[rust_name = "is_empty"]
        fn isEmpty(&self) -> bool;

        /// Returns `true` if this is a valid `QSslDiffieHellmanParameters`; otherwise `false`.
        ///
        /// This method should be used after constructing a `QSslDiffieHellmanParameters` object to determine its validity.
        ///
        /// If a `QSslDiffieHellmanParameters` object is not valid, you can use the `error()` method to determine what error prevented the object from being constructed.
        #[rust_name = "is_valid"]
        fn isValid(&self) -> bool;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qssldiffiehellmanparameters_default_parameters"]
        fn qssldiffiehellmanparametersDefaultParameters() -> QSslDiffieHellmanParameters;

        #[rust_name = "qssldiffiehellmanparameters_from_encoded_device"]
        unsafe fn qssldiffiehellmanparametersFromEncoded(
            device: *mut QIODevice,
            encoding: QSslEncodingFormat,
        ) -> QSslDiffieHellmanParameters;

        #[rust_name = "qssldiffiehellmanparameters_from_encoded_qbytearray"]
        fn qssldiffiehellmanparametersFromEncoded(
            encoded: &QByteArray,
            encoding: QSslEncodingFormat,
        ) -> QSslDiffieHellmanParameters;

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

impl Valid for QSslDiffieHellmanParameters {
    fn is_valid(value: &Self) -> bool {
        value.is_valid()
    }
}

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

impl Debug for QSslDiffieHellmanParameters {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            ffi::qssldiffiehellmanparameters_to_debug_qstring(self)
        )
    }
}

impl QSslDiffieHellmanParameters {
    fn into_result(self) -> Result<Self, QSslDiffieHellmanParametersError> {
        if self.is_valid() {
            Ok(self)
        } else {
            Err(self.error())
        }
    }

    /// Returns the default `QSslDiffieHellmanParameters` used by `QSslSocket`.
    pub fn default_parameters() -> Self {
        ffi::qssldiffiehellmanparameters_default_parameters()
    }

    /// Attempts to construct a `QSslDiffieHellmanParameters` object by reading from `device` in either PEM or DER form as specified by `encoding`.
    pub fn from_encoded_device<T>(
        device: Pin<&mut T>,
        encoding: QSslEncodingFormat,
    ) -> Result<Self, QSslDiffieHellmanParametersError>
    where
        T: Upcast<QIODevice>,
    {
        let device = device.upcast_pin();
        unsafe {
            ffi::qssldiffiehellmanparameters_from_encoded_device(
                ptr::from_ref(&*device).cast_mut(),
                encoding,
            )
        }
        .into_result()
    }

    /// Attempts to construct a `QSslDiffieHellmanParameters` object using the byte array `encoded` in either PEM or DER form as specified by `encoding`.
    pub fn from_encoded_data(
        data: &QByteArray,
        encoding: QSslEncodingFormat,
    ) -> Result<Self, QSslDiffieHellmanParametersError> {
        ffi::qssldiffiehellmanparameters_from_encoded_qbytearray(data, encoding).into_result()
    }
}

impl TryFrom<&QByteArray> for QSslDiffieHellmanParameters {
    type Error = QSslDiffieHellmanParametersError;

    fn try_from(value: &QByteArray) -> Result<Self, Self::Error> {
        ffi::qssldiffiehellmanparameters_from_encoded_qbytearray(value, QSslEncodingFormat::Pem)
            .into_result()
    }
}

unsafe impl ExternType for QSslDiffieHellmanParameters {
    type Id = type_id!("QSslDiffieHellmanParameters");
    type Kind = cxx::kind::Trivial;
}

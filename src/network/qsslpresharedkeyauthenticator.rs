use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
    }

    extern "C++" {
        include!("cxx-qt-io/qsslpresharedkeyauthenticator.h");
    }

    unsafe extern "C++" {
        type QSslPreSharedKeyAuthenticator = super::QSslPreSharedKeyAuthenticator;

        /// Returns the PSK client identity.
        fn identity(&self) -> QByteArray;

        /// Returns the PSK identity hint as provided by the server. The interpretation of this hint is left to the application.
        #[rust_name = "identity_hint"]
        fn identityHint(&self) -> QByteArray;

        /// Returns the maximum length, in bytes, of the PSK client identity.
        ///
        /// **Note:** it is possible to set an identity whose length is greater than `maximum_identity_length()`; in this case, only the first `maximum_identity_length()` bytes will be actually sent to the server.
        #[rust_name = "maximum_identity_length"]
        fn maximumIdentityLength(&self) -> i32;

        /// Returns the maximum length, in bytes, of the pre shared key.
        ///
        /// **Note:** it is possible to set a key whose length is greater than the `maximum_pre_shared_key_length()`; in this case, only the first maximumPreSharedKeyLength() bytes will be actually sent to the server.
        #[rust_name = "maximum_pre_shared_key_length"]
        fn maximumPreSharedKeyLength(&self) -> i32;

        /// Returns the pre shared key.
        #[rust_name = "pre_shared_key"]
        fn preSharedKey(&self) -> QByteArray;

        /// Sets the PSK client identity (to be advised to the server) to `identity`.
        ///
        /// **Note:** it is possible to set an identity whose length is greater than `maximum_identity_length()`; in this case, only the first `maximum_identity_length()` bytes will be actually sent to the server.
        #[rust_name = "set_identity"]
        fn setIdentity(&mut self, identity: &QByteArray);

        /// Sets the pre shared key to `pre_shared_key`.
        ///
        /// **Note:** it is possible to set a key whose length is greater than the `maximum_pre_shared_key_length()` in this case, only the first `maximum_pre_shared_key_length()` bytes will be actually sent to the server.
        #[rust_name = "set_pre_shared_key"]
        fn setPreSharedKey(&mut self, pre_shared_key: &QByteArray);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qsslpresharedkeyauthenticator_drop"]
        fn drop(extension: &mut QSslPreSharedKeyAuthenticator);

        #[rust_name = "qsslpresharedkeyauthenticator_init_default"]
        fn construct() -> QSslPreSharedKeyAuthenticator;
        #[rust_name = "qsslpresharedkeyauthenticator_clone"]
        fn construct(other: &QSslPreSharedKeyAuthenticator) -> QSslPreSharedKeyAuthenticator;

        #[rust_name = "qsslpresharedkeyauthenticator_eq"]
        fn operatorEq(a: &QSslPreSharedKeyAuthenticator, b: &QSslPreSharedKeyAuthenticator)
            -> bool;
    }
}

/// The `QSslPreSharedKeyAuthenticator` class provides authentication data for pre shared keys (PSK) ciphersuites.
///
/// Qt Documentation: [QSslPreSharedKeyAuthenticator](https://doc.qt.io/qt-6/qsslpresharedkeyauthenticator.html#details)
#[repr(C)]
pub struct QSslPreSharedKeyAuthenticator {
    _space: MaybeUninit<usize>,
}

impl Clone for QSslPreSharedKeyAuthenticator {
    fn clone(&self) -> Self {
        ffi::qsslpresharedkeyauthenticator_clone(self)
    }
}

impl Drop for QSslPreSharedKeyAuthenticator {
    fn drop(&mut self) {
        ffi::qsslpresharedkeyauthenticator_drop(self);
    }
}

impl Default for QSslPreSharedKeyAuthenticator {
    /// Constructs a `QSslPreSharedKeyAuthenticator` object with no error and default certificate.
    fn default() -> Self {
        ffi::qsslpresharedkeyauthenticator_init_default()
    }
}

impl PartialEq for QSslPreSharedKeyAuthenticator {
    fn eq(&self, other: &Self) -> bool {
        ffi::qsslpresharedkeyauthenticator_eq(self, other)
    }
}

impl Eq for QSslPreSharedKeyAuthenticator {}

unsafe impl ExternType for QSslPreSharedKeyAuthenticator {
    type Id = type_id!("QSslPreSharedKeyAuthenticator");
    type Kind = cxx::kind::Trivial;
}

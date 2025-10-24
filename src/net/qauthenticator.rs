use std::pin::Pin;

use cxx::UniquePtr;
use cxx_qt_lib::{QString, QVariant};

use crate::util::IsNonNull;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
        include!("cxx-qt-lib/qhash.h");
        type QHash_QString_QVariant = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_QString_QVariant>;
    }

    extern "C++" {
        include!("cxx-qt-io/qauthenticator.h");
    }

    unsafe extern "C++" {
        /// The `QAuthenticator` class provides an authentication object.
        ///
        /// Qt Documentation: [QAuthenticator](https://doc.qt.io/qt-6/qauthenticator.html#details)
        type QAuthenticator;

        /// Returns `true` if the object has not been initialized. Returns `false` if non-const member functions have been called, or the content was constructed or copied from another initialized `QAuthenticator` object.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "option_or_invalid"]
        fn option(&self, opt: &QString) -> QVariant;

        /// Returns all incoming options set in this `QAuthenticator` object by parsing the server reply.
        fn options(&self) -> QHash_QString_QVariant;

        /// Returns the password used for authentication.
        fn password(&self) -> QString;

        /// Returns the realm requiring authentication.
        fn realm(&self) -> QString;

        #[doc(hidden)]
        #[rust_name = "set_option_variant"]
        fn setOption(self: Pin<&mut Self>, opt: &QString, value: &QVariant);

        /// Sets the `password` used for authentication.
        #[rust_name = "set_password"]
        fn setPassword(self: Pin<&mut Self>, password: &QString);

        /// Sets the `user` used for authentication.
        #[rust_name = "set_user"]
        fn setUser(self: Pin<&mut Self>, user: &QString);

        /// Returns the user used for authentication.
        fn user(&self) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qauthenticator_init_default"]
        fn make_unique() -> UniquePtr<QAuthenticator>;
        #[rust_name = "qauthenticator_eq"]
        fn operatorEq(a: &QAuthenticator, b: &QAuthenticator) -> bool;
    }
}

pub use ffi::QAuthenticator;

impl PartialEq for QAuthenticator {
    fn eq(&self, other: &Self) -> bool {
        ffi::qauthenticator_eq(self, other)
    }
}

impl Eq for QAuthenticator {}

impl IsNonNull for QAuthenticator {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

impl QAuthenticator {
    /// Constructs an empty authentication object.
    pub fn new() -> UniquePtr<Self> {
        ffi::qauthenticator_init_default()
    }

    /// Sets the outgoing option `opt` to value `value`.
    pub fn set_option<T>(self: Pin<&mut Self>, opt: &QString, value: T)
    where
        T: Into<QVariant>,
    {
        self.set_option_variant(opt, &value.into());
    }

    /// Returns the value related to option `opt` if it was set by the server. Returns `None` if option `opt` isn't found.
    pub fn option(&self, opt: &QString) -> Option<QVariant> {
        self.option_or_invalid(opt).nonnull()
    }
}

#[cfg(test)]
mod tests {
    use cxx_qt_lib::QString;

    use super::*;

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QAuthenticatorProps {
            password: QString,
            user: QString,
        }

        let props = QAuthenticatorProps {
            password: QString::from("mypassword"),
            user: QString::from("myuser"),
        };

        let mut authenticator = QAuthenticator::new();
        authenticator.pin_mut().set_password(&props.password);
        authenticator.pin_mut().set_user(&props.user);

        let actual_props = QAuthenticatorProps {
            password: authenticator.password(),
            user: authenticator.user(),
        };

        assert_eq!(actual_props, props);
    }

    #[test]
    fn nonnull() {
        let mut auth = QAuthenticator::new();
        auth.pin_mut().set_password(&QString::from("password"));
        assert_nonnull!(auth, QAuthenticator::new());
    }
}

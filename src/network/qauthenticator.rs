use std::pin::Pin;

use cxx_qt_lib::{QString, QVariant};

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

    unsafe extern "C++" {
        include!(<QtNetwork/QAuthenticator>);

        /// The `QAuthenticator` class provides an authentication object.
        ///
        /// Qt Documentation: [QAuthenticator](https://doc.qt.io/qt-6/qauthenticator.html#details)
        type QAuthenticator;

        /// Returns `true` if the object has not been initialized. Returns `false` if non-const member functions have been called, or the content was constructed or copied from another initialized `QAuthenticator` object.
        #[rust_name = "is_null"]
        fn isNull(self: &QAuthenticator) -> bool;

        /// Returns the value related to option `opt` if it was set by the server. If option `opt` isn't found, an invalid `QVariant` will be returned.
        fn option(self: &QAuthenticator, opt: &QString) -> QVariant;

        /// Returns all incoming options set in this `QAuthenticator` object by parsing the server reply.
        fn options(self: &QAuthenticator) -> QHash_QString_QVariant;

        /// Returns the password used for authentication.
        fn password(self: &QAuthenticator) -> QString;

        /// Returns the realm requiring authentication.
        fn realm(self: &QAuthenticator) -> QString;

        #[doc(hidden)]
        #[rust_name = "set_option_variant"]
        fn setOption(self: Pin<&mut QAuthenticator>, opt: &QString, value: &QVariant);

        /// Sets the `password` used for authentication.
        #[rust_name = "set_password"]
        fn setPassword(self: Pin<&mut QAuthenticator>, password: &QString);

        /// Sets the `user` used for authentication.
        #[rust_name = "set_user"]
        fn setUser(self: Pin<&mut QAuthenticator>, user: &QString);

        /// Returns the user used for authentication.
        fn user(self: &QAuthenticator) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qauthenticator_eq"]
        fn operatorEq(a: &QAuthenticator, b: &QAuthenticator) -> bool;
    }
}

pub use ffi::QAuthenticator;

use crate::util::NonNull;

impl QAuthenticator {
    /// Sets the outgoing option `opt` to value `value`.
    pub fn set_option<T>(self: Pin<&mut Self>, opt: &QString, value: T)
    where
        T: Into<QVariant>,
    {
        self.set_option_variant(opt, &value.into());
    }
}

impl PartialEq for QAuthenticator {
    fn eq(&self, other: &Self) -> bool {
        ffi::qauthenticator_eq(self, other)
    }
}

impl Eq for QAuthenticator {}

impl NonNull for QAuthenticator {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

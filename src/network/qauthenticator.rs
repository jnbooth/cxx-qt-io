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

        /// Sets the outgoing option `opt` to value `value`.
        #[rust_name = "set_option"]
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
}

pub use ffi::QAuthenticator;

use cxx::{type_id, ExternType};
use cxx_qt_lib::{QByteArray, QDateTime, QList};
use std::fmt::{self, Debug, Formatter};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    /// This enum is used with the `to_raw_form()` function to declare which form of a cookie shall be returned.
    ///
    /// Note that only the Full form of the cookie can be parsed back into its original contents.
    #[repr(i32)]
    #[derive(Debug)]
    enum CookieRawForm {
        /// makes `to_raw_form()` return only the `"NAME=VALUE"` part of the cookie, as suitable for sending back to a server in a client request's `"Cookie:"` header. Multiple cookies are separated by a semi-colon in the `"Cookie:"` header field.
        NameAndValueOnly,
        /// makes `to_raw_form()` return the full cookie contents, as suitable for sending to a client in a server's `"Set-Cookie:"` header.
        Full,
    }

    #[cfg(cxxqt_qt_version_at_least_6_1)]
    #[repr(i32)]
    #[derive(Debug)]
    enum SameSitePolicy {
        /// `SameSite` is not set. Can be interpreted as None or Lax by the browser.
        Default,
        /// Cookies can be sent in all contexts. This used to be default, but recent browsers made Lax default, and will now require the cookie to be both secure and to set `SameSite=None`.
        None,
        /// Cookies are sent on first party requests and GET requests initiated by third party website. This is the default in modern browsers (since mid 2020).
        Lax,
        /// Cookies will only be sent in a first-party context.
        Strict,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = cxx_qt_lib::QDateTime;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-io/qlist.h");
        type QList_QNetworkCookie = cxx_qt_lib::QList<QNetworkCookie>;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkcookie.h");
        type CookieRawForm;
        #[cfg(cxxqt_qt_version_at_least_6_1)]
        type SameSitePolicy;
    }

    unsafe extern "C++" {
        type QNetworkCookie = super::QNetworkCookie;

        /// Returns the path associated with this cookie. This corresponds to the "path" field of the cookie string.
        fn path(self: &QNetworkCookie) -> QString;

        /// Returns the `SameSite` option if specified in the cookie string, `SameSitePolicy::Default` if not present.
        #[cfg(cxxqt_qt_version_at_least_6_1)]
        #[rust_name = "same_site_policy"]
        fn sameSitePolicy(self: &QNetworkCookie) -> SameSitePolicy;

        /// Sets the domain associated with this cookie to be `domain`.
        #[rust_name = "set_domain"]
        fn setDomain(self: &mut QNetworkCookie, domain: &QString);

        #[doc(hidden)]
        #[rust_name = "set_expiration_date_or_null"]
        fn setExpirationDate(self: &mut QNetworkCookie, date: &QDateTime);

        /// Sets this cookie's `HttpOnly` flag to `enable`.
        #[rust_name = "set_http_only"]
        fn setHttpOnly(self: &mut QNetworkCookie, enable: bool);

        /// Sets the name of this cookie to be `cookie_name`. Note that setting a cookie name to an empty `QByteArray` will make this cookie invalid.
        #[rust_name = "set_name"]
        fn setName(self: &mut QNetworkCookie, cookie_name: &QByteArray);

        /// Sets the path associated with this cookie to be `path`.
        #[rust_name = "set_path"]
        fn setPath(self: &mut QNetworkCookie, path: &QString);

        /// Sets the `SameSite` option of this cookie to `same_site`.
        #[cfg(cxxqt_qt_version_at_least_6_1)]
        #[rust_name = "set_same_site_policy"]
        fn setSameSitePolicy(self: &mut QNetworkCookie, policy: SameSitePolicy);

        /// Sets the secure flag of this cookie to `enable`.
        ///
        /// Secure cookies may contain private information and should not be resent over unencrypted connections.
        #[rust_name = "set_secure"]
        fn setSecure(self: &mut QNetworkCookie, enable: bool);

        /// Sets the value of this cookie to be `value`.
        #[rust_name = "set_value"]
        fn setValue(self: &mut QNetworkCookie, value: &QByteArray);

        /// Returns the raw form of this `QNetworkCookie`. The `QByteArray` returned by this function is suitable for an HTTP header, either in a server response (the `Set-Cookie` header) or the client request (the `Cookie` header). You can choose from one of two formats, using form.
        #[rust_name = "to_raw_form"]
        fn toRawForm(self: &QNetworkCookie, form: CookieRawForm) -> QByteArray;

        /// Returns this cookies value, as specified in the cookie string. Note that a cookie is still valid if its value is empty.
        ///
        /// Cookie name-value pairs are considered opaque to the application: that is, their values don't mean anything.
        fn value(self: &QNetworkCookie) -> QByteArray;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qnetworkcookie_parse_cookies"]
        fn qnetworkcookieParseCookies(cookie_string: &[u8]) -> QList_QNetworkCookie;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qnetworkcookie_drop"]
        fn drop(headers: &mut QNetworkCookie);

        #[doc(hidden)]
        #[rust_name = "qnetworkcookie_init_default"]
        fn construct() -> QNetworkCookie;

        #[doc(hidden)]
        #[rust_name = "qnetworkcookie_init_name_val"]
        fn construct(name: &QByteArray, value: &QByteArray) -> QNetworkCookie;
        #[doc(hidden)]
        #[rust_name = "qnetworkcookie_clone"]
        fn construct(other: &QNetworkCookie) -> QNetworkCookie;
        #[doc(hidden)]
        #[rust_name = "qnetworkcookie_eq"]
        fn operatorEq(a: &QNetworkCookie, b: &QNetworkCookie) -> bool;
        #[doc(hidden)]
        #[rust_name = "qnetworkcookie_to_debug_qstring"]
        fn toDebugQString(value: &QNetworkCookie) -> QString;
    }
}

pub use ffi::CookieRawForm;

#[cfg(cxxqt_qt_version_at_least_6_1)]
pub use ffi::SameSitePolicy;

/// The `QNetworkCookie` class holds one network cookie.
///
/// Qt Documentation: [QNetworkCookie](https://doc.qt.io/qt-6/qnetworkcookie.html#details)
#[repr(C)]
pub struct QNetworkCookie {
    _space: MaybeUninit<usize>,
}

impl Clone for QNetworkCookie {
    fn clone(&self) -> Self {
        ffi::qnetworkcookie_clone(self)
    }
}

impl Default for QNetworkCookie {
    /// Creates an invalid cookie.
    fn default() -> Self {
        ffi::qnetworkcookie_init_default()
    }
}

impl Drop for QNetworkCookie {
    fn drop(&mut self) {
        ffi::qnetworkcookie_drop(self);
    }
}

impl PartialEq for QNetworkCookie {
    fn eq(&self, other: &Self) -> bool {
        ffi::qnetworkcookie_eq(self, other)
    }
}

impl Eq for QNetworkCookie {}

impl Debug for QNetworkCookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ffi::qnetworkcookie_to_debug_qstring(self))
    }
}

impl QNetworkCookie {
    /// Parses the cookie string `cookie_string` as received from a server response in the `"Set-Cookie:"` header. If there's a parsing error, this function returns an empty list.
    ///
    /// Since the HTTP header can set more than one cookie at the same time, this function returns a `QList<QNetworkCookie>`, one for each cookie that is parsed.
    pub fn parse_cookies<T: AsRef<[u8]>>(cookie_string: &T) -> QList<Self> {
        ffi::qnetworkcookie_parse_cookies(cookie_string.as_ref())
    }

    /// Create a new `QNetworkCookie` object, initializing the cookie name to name and its value to value.
    ///
    /// A cookie is only valid if it has a name. However, the value is opaque to the application and being empty may have significance to the remote server.
    pub fn new(name: &QByteArray, value: &QByteArray) -> Self {
        ffi::qnetworkcookie_init_name_val(name, value)
    }

    /// Sets the expiration date of this cookie to date. Setting an expiration date of `None` to this cookie will mean it's a session cookie.
    pub fn set_expiration_date(&mut self, date: Option<&QDateTime>) {
        match date {
            Some(date) => self.set_expiration_date_or_null(date),
            None => self.set_expiration_date_or_null(&QDateTime::default()),
        };
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkCookie {
    type Id = type_id!("QNetworkCookie");
    type Kind = cxx::kind::Trivial;
}

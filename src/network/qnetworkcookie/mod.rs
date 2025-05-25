#[cfg(cxxqt_qt_version_at_least_6_1)]
mod v6_1;
#[cfg(cxxqt_qt_version_at_least_6_1)]
pub use v6_1::QNetworkCookieSameSite;

use cxx::{type_id, ExternType};
use cxx_qt_lib::{QByteArray, QDateTime, QList};
use std::fmt;
use std::mem::MaybeUninit;

use crate::util::IsNonNull;

#[cxx::bridge]
mod ffi {
    /// This enum is used with [`QNetworkCookie::to_raw_form`] to declare which form of a cookie shall be returned.
    ///
    /// Note that only the [`Full`](QNetworkCookieRawForm::Full) form of the cookie can be parsed back into its original contents.
    #[repr(i32)]
    enum QNetworkCookieRawForm {
        /// Makes [`QNetworkCookie::to_raw_form`] return only the `"NAME=VALUE"` part of the cookie, as suitable for sending back to a server in a client request's `Cookie` header. Multiple cookies are separated by a semi-colon in the `Cookie` header field.
        NameAndValueOnly,
        /// Makes [`QNetworkCookie::to_raw_form`] return the full cookie contents, as suitable for sending to a client in a server's `Set-Cookie` header.
        Full,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = cxx_qt_lib::QDateTime;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-io/qlist.h");
        type QList_QNetworkCookie = cxx_qt_lib::QList<QNetworkCookie>;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkcookie.h");
        type QNetworkCookieRawForm;
        #[cfg(cxxqt_qt_version_at_least_6_1)]
        type QNetworkCookieSameSite = super::QNetworkCookieSameSite;
    }

    unsafe extern "C++" {
        type QNetworkCookie = super::QNetworkCookie;

        /// Returns the domain this cookie is associated with. This corresponds to the "domain" field of the cookie string.
        ///
        /// Note that the domain here may start with a dot, which is not a valid hostname. However, it means this cookie matches all hostnames ending with that domain name.
        fn domain(&self) -> QString;

        #[doc(hidden)]
        #[rust_name = "expiration_date_or_null"]
        fn expirationDate(&self) -> QDateTime;

        /// Returns `true` if this cookie has the same identifier tuple as `other`. The identifier tuple is composed of the name, domain and path.
        #[rust_name = "has_same_identifier"]
        fn hasSameIdentifier(&self, other: &QNetworkCookie) -> bool;

        /// Returns `true` if the "HttpOnly" flag is enabled for this cookie.
        ///
        /// A cookie that is "HttpOnly" is only set and retrieved by the network requests and replies; i.e., the HTTP protocol. It is not accessible from scripts running on browsers.
        #[rust_name = "is_http_only"]
        fn isHttpOnly(&self) -> bool;

        /// Returns `true` if the "secure" option was specified in the cookie string, `false` otherwise.
        ///
        /// Secure cookies may contain private information and should not be resent over unencrypted connections.
        #[rust_name = "is_secure"]
        fn isSecure(&self) -> bool;

        /// Returns `true` if this cookie is a session cookie. A session cookie is a cookie which has no expiration date, which means it should be discarded when the application's concept of session is over (usually, when the application exits).
        #[rust_name = "is_session_cookie"]
        fn isSessionCookie(&self) -> bool;

        /// Returns the name of this cookie. The only mandatory field of a cookie is its name, without which it is not considered valid.
        fn name(&self) -> QByteArray;

        /// This functions normalizes the path and domain of the cookie if they were previously empty. The `url` parameter is used to determine the correct domain and path.
        fn normalize(&mut self, url: &QUrl);

        /// Returns the path associated with this cookie. This corresponds to the "path" field of the cookie string.
        fn path(&self) -> QString;

        /// Returns the `SameSite` option if specified in the cookie string, [`QNetworkCookieSameSite::Default`] if not present.
        ///
        /// Introduced in Qt 6.1.
        #[cfg(cxxqt_qt_version_at_least_6_1)]
        #[rust_name = "same_site_policy"]
        fn sameSitePolicy(&self) -> QNetworkCookieSameSite;

        /// Sets the domain associated with this cookie to be `domain`.
        #[rust_name = "set_domain"]
        fn setDomain(&mut self, domain: &QString);

        #[doc(hidden)]
        #[rust_name = "set_expiration_date_or_null"]
        fn setExpirationDate(&mut self, date: &QDateTime);

        /// Sets this cookie's `HttpOnly` flag to `enable`.
        #[rust_name = "set_http_only"]
        fn setHttpOnly(&mut self, enable: bool);

        /// Sets the name of this cookie to be `cookie_name`. Note that setting a cookie name to an empty `QByteArray` will make this cookie invalid.
        #[rust_name = "set_name"]
        fn setName(&mut self, cookie_name: &QByteArray);

        /// Sets the path associated with this cookie to be `path`.
        #[rust_name = "set_path"]
        fn setPath(&mut self, path: &QString);

        /// Sets the `SameSite` option of this cookie to `same_site`.
        ///
        /// Introduced in Qt 6.1.
        #[cfg(cxxqt_qt_version_at_least_6_1)]
        #[rust_name = "set_same_site_policy"]
        fn setSameSitePolicy(&mut self, policy: QNetworkCookieSameSite);

        /// Sets the secure flag of this cookie to `enable`.
        ///
        /// Secure cookies may contain private information and should not be resent over unencrypted connections.
        #[rust_name = "set_secure"]
        fn setSecure(&mut self, enable: bool);

        /// Sets the value of this cookie to be `value`.
        #[rust_name = "set_value"]
        fn setValue(&mut self, value: &QByteArray);

        /// Returns the raw form of this `QNetworkCookie`. The `QByteArray` returned by this function is suitable for an HTTP header, either in a server response (the `Set-Cookie` header) or the client request (the `Cookie` header). You can choose from one of two formats, using form.
        #[rust_name = "to_raw_form"]
        fn toRawForm(&self, form: QNetworkCookieRawForm) -> QByteArray;

        /// Returns this cookie's value, as specified in the cookie string. Note that a cookie is still valid if its value is empty.
        ///
        /// Cookie name-value pairs are considered opaque to the application: that is, their values don't mean anything.
        fn value(&self) -> QByteArray;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qnetworkcookie_parse_cookies"]
        fn qnetworkcookieParseCookies(cookie_string: &[u8]) -> QList_QNetworkCookie;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkcookie_drop"]
        fn drop(cookie: &mut QNetworkCookie);

        #[rust_name = "qnetworkcookie_init_default"]
        fn construct() -> QNetworkCookie;

        #[rust_name = "qnetworkcookie_init_name_val"]
        fn construct(name: &QByteArray, value: &QByteArray) -> QNetworkCookie;
        #[rust_name = "qnetworkcookie_clone"]
        fn construct(other: &QNetworkCookie) -> QNetworkCookie;

        #[rust_name = "qnetworkcookie_eq"]
        fn operatorEq(a: &QNetworkCookie, b: &QNetworkCookie) -> bool;
    }
}

pub use ffi::QNetworkCookieRawForm;

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

impl fmt::Debug for QNetworkCookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_raw_form(QNetworkCookieRawForm::Full).fmt(f)
    }
}

impl fmt::Display for QNetworkCookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_raw_form(QNetworkCookieRawForm::Full).fmt(f)
    }
}

impl QNetworkCookie {
    /// Returns the expiration date for this cookie. Returns `None` if this cookie is a session cookie. If the date is in the past, this cookie has already expired and should not be sent again back to a remote server.
    ///
    /// The expiration date corresponds to the parameters of the "expires" entry in the cookie string.
    pub fn expiration_date(&self) -> Option<QDateTime> {
        self.expiration_date_or_null().nonnull()
    }

    /// Parses the cookie string `cookie_string` as received from a server response in the `Set-Cookie` header. If there's a parsing error, this function returns an empty list.
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

    /// Sets the expiration date of this cookie to `date`. Setting an expiration date of `None` to this cookie will mean it's a session cookie.
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

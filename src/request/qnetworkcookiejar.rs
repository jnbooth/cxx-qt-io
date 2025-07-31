use std::fmt;
use std::ops::Deref;

use cxx::UniquePtr;
use cxx_qt::QObject;
use cxx_qt::casting::Upcast;

use crate::qobject::debug_qobject;

#[cxx_qt::bridge]
mod ffi {

    extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-io/qnetworkcookie.h");
        type QNetworkCookie = crate::QNetworkCookie;
        include!("cxx-qt-io/qlist.h");
        type QList_QNetworkCookie = cxx_qt_lib::QList<QNetworkCookie>;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkcookiejar.h");
    }

    unsafe extern "C++Qt" {
        /// The `QNetworkCookieJar` class implements a simple jar of [`QNetworkCookie`](crate::QNetworkCookie) objects.
        ///
        /// Qt Documentation: [QNetworkCookieJar](https://doc.qt.io/qt-6/qnetworkcookiejar.html#details)
        #[qobject]
        #[base = QObject]
        type QNetworkCookieJar;

        /// Returns the cookies to be added to when a request is sent to `url`. This function is called by the default [`QNetworkAccessManager::create_request`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#createRequest), which adds the cookies returned by this function to the request being sent.
        ///
        /// If more than one cookie with the same name is found, but with differing paths, the one with longer path is returned before the one with shorter path. In other words, this function returns cookies sorted decreasingly by path length.
        #[rust_name = "cookies_for_url"]
        fn cookiesForUrl(self: &QNetworkCookieJar, url: &QUrl) -> QList_QNetworkCookie;

        /// Deletes from cookie jar the cookie found to have the same identifier as `cookie`.
        ///
        /// Returns `true` if a cookie was deleted, `false` otherwise.
        #[rust_name = "delete_cookie"]
        fn deleteCookie(self: Pin<&mut QNetworkCookieJar>, cookie: &QNetworkCookie) -> bool;

        /// Adds `cookie` to this cookie jar.
        ///
        /// Returns `true` if `cookie` was added, `false` otherwise.
        ///
        /// If a cookie with the same identifier already exists in the cookie jar, it will be overridden.
        #[rust_name = "insert_cookie"]
        fn insertCookie(self: Pin<&mut QNetworkCookieJar>, cookie: &QNetworkCookie) -> bool;

        /// Adds the cookies in the list `cookie_list` to this cookie jar. Before being inserted cookies are normalized.
        ///
        /// Returns `true` if one or more cookies are set for `url`, otherwise `false`.
        ///
        /// If a cookie already exists in the cookie jar, it will be overridden by those in `cookie_list`.
        #[rust_name = "set_cookies_from_url"]
        fn setCookiesFromUrl(
            self: Pin<&mut QNetworkCookieJar>,
            cookie_list: &QList_QNetworkCookie,
            url: &QUrl,
        ) -> bool;

        /// If a cookie with the same identifier as `cookie` exists in this cookie jar it will be updated.
        ///
        /// Returns true if `cookie` was updated, false if no cookie in the jar matches the identifier of `cookie`.
        #[rust_name = "update_cookie"]
        fn updateCookie(self: Pin<&mut QNetworkCookieJar>, cookie: &QNetworkCookie) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkcookiejar_default"]
        fn make_unique() -> UniquePtr<QNetworkCookieJar>;
    }
}

pub use ffi::QNetworkCookieJar;

impl fmt::Debug for QNetworkCookieJar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QNetworkCookieJar {
    pub fn new() -> UniquePtr<Self> {
        ffi::qnetworkcookiejar_default()
    }
}

impl Deref for QNetworkCookieJar {
    type Target = QObject;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

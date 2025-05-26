use std::fmt;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkCookieSameSite {
        /// The `SameSite` attribute is not set. Can be interpreted as [`None`](QNetworkCookieSameSite::None) or [`Lax`](QNetworkCookieSameSite::Lax) by the browser.
        Default,
        /// Cookies can be sent in all contexts. This used to be default, but recent browsers made [`Lax`](QNetworkCookieSameSite::Lax) default, and will now require the cookie to be both secure and to set `SameSite=None`.
        None,
        /// Cookies are sent on first party requests and GET requests initiated by third party website. This is the default in modern browsers (since mid 2020).
        Lax,
        /// Cookies will only be sent in a first-party context.
        Strict,
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkcookie.h");
        type QNetworkCookieSameSite;
    }
}

pub use ffi::QNetworkCookieSameSite;

impl fmt::Display for QNetworkCookieSameSite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::Default => "Default",
            Self::None => "None",
            Self::Lax => "Lax",
            Self::Strict => "Strict",
            _ => "unknown",
        })
    }
}

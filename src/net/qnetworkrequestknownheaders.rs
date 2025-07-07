use std::fmt;

#[cxx::bridge]
mod ffi {
    /// List of known header types that [`QNetworkRequest`] parses. Each known header is also represented in raw form with its full HTTP name.
    #[repr(i32)]
    #[derive(PartialEq, Eq)]
    enum QNetworkRequestKnownHeaders {
        /// Corresponds to the HTTP Content-Type header and contains a string containing the media (MIME) type and any auxiliary data (for instance, charset).
        ContentTypeHeader,
        /// Corresponds to the HTTP Content-Length header and contains the length in bytes of the data transmitted.
        ContentLengthHeader,
        /// Corresponds to the HTTP Location header and contains a URL representing the actual location of the data, including the destination URL in case of redirections.
        LocationHeader,
        /// Corresponds to the HTTP Last-Modified header and contains a [`QDateTime`](cxx_qt_lib::QDateTime) representing the last modification date of the contents.
        LastModifiedHeader,
        /// Corresponds to the HTTP Cookie header and contains a [`QList`](cxx_qt_lib::QList)[`<QNetworkCookie>`](crate::QNetworkCookie) representing the cookies to be sent back to the server.
        CookieHeader,
        /// Corresponds to the HTTP Set-Cookie header and contains a [`QList`](cxx_qt_lib::QList)[`<QNetworkCookie>`](crate::QNetworkCookie) representing the cookies sent by the server to be stored locally.
        SetCookieHeader,
        /// Corresponds to the HTTP Content-Disposition header and contains a string containing the disposition type (for instance, attachment) and a parameter (for instance, filename).
        ContentDispositionHeader,
        /// The User-Agent header sent by HTTP clients.
        UserAgentHeader,
        /// The Server header received by HTTP clients.
        ServerHeader,
        /// Corresponds to the HTTP If-Modified-Since header and contains a [`QDateTime`](cxx_qt_lib::QDateTime). It is usually added to a [`QNetworkRequest`]. The server shall send a 304 (Not Modified) response if the resource has not changed since this time.
        IfModifiedSinceHeader,
        /// Corresponds to the HTTP ETag header and contains a [`QString`](cxx_qt_lib::QString) representing the last modification state of the contents.
        ETagHeader,
        /// Corresponds to the HTTP If-Match header and contains a [`QStringList`](cxx_qt_lib::QStringList). It is usually added to a [`QNetworkRequest`]. The server shall send a 412 (Precondition Failed) response if the resource does not match.
        IfMatchHeader,
        /// Corresponds to the HTTP If-None-Match header and contains a [`QStringList`](cxx_qt_lib::QStringList). It is usually added to a [`QNetworkRequest`]. The server shall send a 304 (Not Modified) response if the resource does match.
        IfNoneMatchHeader,
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkrequest.h");
        type QNetworkRequestKnownHeaders;
    }
}

pub use ffi::QNetworkRequestKnownHeaders;

impl fmt::Display for QNetworkRequestKnownHeaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::ContentTypeHeader => "Content-Type",
            Self::ContentLengthHeader => "Content-Length",
            Self::LocationHeader => "Location",
            Self::LastModifiedHeader => "Last-Modified",
            Self::CookieHeader => "Cookie",
            Self::SetCookieHeader => "Set-Cookie",
            Self::ContentDispositionHeader => "Content-Disposition",
            Self::UserAgentHeader => "User-Agent",
            Self::ServerHeader => "Server",
            Self::IfModifiedSinceHeader => "If-Modified-Since",
            Self::ETagHeader => "ETag",
            Self::IfMatchHeader => "If-Match",
            Self::IfNoneMatchHeader => "If-None-Match",
            _ => "unknown",
        })
    }
}

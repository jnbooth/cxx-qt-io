mod attribute;
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

pub use attribute::QNetworkRequestAttribute;

#[cxx::bridge]
mod ffi {
    /// List of known header types that `QNetworkRequest` parses. Each known header is also represented in raw form with its full HTTP name.
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkRequestKnownHeaders {
        /// Corresponds to the HTTP Content-Type header and contains a string containing the media (MIME) type and any auxiliary data (for instance, charset).
        ContentTypeHeader,
        /// Corresponds to the HTTP Content-Length header and contains the length in bytes of the data transmitted.
        ContentLengthHeader,
        /// Corresponds to the HTTP Location header and contains a URL representing the actual location of the data, including the destination URL in case of redirections.
        LocationHeader,
        /// Corresponds to the HTTP Last-Modified header and contains a `QDateTime` representing the last modification date of the contents.
        LastModifiedHeader,
        /// Corresponds to the HTTP Cookie header and contains a `QList<QNetworkCookie>` representing the cookies to be sent back to the server.
        CookieHeader,
        /// Corresponds to the HTTP Set-Cookie header and contains a `QList<QNetworkCookie>` representing the cookies sent by the server to be stored locally.
        SetCookieHeader,
        /// Corresponds to the HTTP Content-Disposition header and contains a string containing the disposition type (for instance, attachment) and a parameter (for instance, filename).
        ContentDispositionHeader,
        /// The User-Agent header sent by HTTP clients.
        UserAgentHeader,
        /// The Server header received by HTTP clients.
        ServerHeader,
        /// Corresponds to the HTTP If-Modified-Since header and contains a `QDateTime`. It is usually added to a QNetworkRequest. The server shall send a 304 (Not Modified) response if the resource has not changed since this time.
        IfModifiedSinceHeader,
        /// Corresponds to the HTTP ETag header and contains a `QString` representing the last modification state of the contents.
        ETagHeader,
        /// Corresponds to the HTTP If-Match header and contains a `QStringList`. It is usually added to a `QNetworkRequest`. The server shall send a 412 (Precondition Failed) response if the resource does not match.
        IfMatchHeader,
        /// Corresponds to the HTTP If-None-Match header and contains a `QStringList`. It is usually added to a QNetworkRequest. The server shall send a 304 (Not Modified) response if the resource does match.
        IfNoneMatchHeader,
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkrequest.h");
        type QNetworkRequestKnownHeaders;
    }

    unsafe extern "C++" {
        type QNetworkRequest = super::QNetworkRequest;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkrequest_drop"]
        fn drop(request: &mut QNetworkRequest);

        #[rust_name = "qnetworkrequest_init_default"]
        fn construct() -> QNetworkRequest;
        #[rust_name = "qnetworkrequest_clone"]
        fn construct(other: &QNetworkRequest) -> QNetworkRequest;

        #[rust_name = "qnetworkrequest_eq"]
        fn operatorEq(a: &QNetworkRequest, b: &QNetworkRequest) -> bool;
    }
}

pub use ffi::QNetworkRequestKnownHeaders;

#[repr(C)]
pub struct QNetworkRequest {
    _space: MaybeUninit<usize>,
}

impl Clone for QNetworkRequest {
    fn clone(&self) -> Self {
        ffi::qnetworkrequest_clone(self)
    }
}

impl Default for QNetworkRequest {
    fn default() -> Self {
        ffi::qnetworkrequest_init_default()
    }
}

impl Drop for QNetworkRequest {
    fn drop(&mut self) {
        ffi::qnetworkrequest_drop(self);
    }
}

impl PartialEq for QNetworkRequest {
    fn eq(&self, other: &Self) -> bool {
        ffi::qnetworkrequest_eq(self, other)
    }
}

impl Eq for QNetworkRequest {}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkRequest {
    type Id = type_id!("QNetworkRequest");
    type Kind = cxx::kind::Trivial;
}

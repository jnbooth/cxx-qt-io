mod attribute;
pub use attribute::QNetworkRequestAttribute;

use cxx::{type_id, ExternType};
#[cfg(cxxqt_qt_version_at_least_6_7)]
use cxx_qt_lib::QAnyStringView;
use cxx_qt_lib::{QByteArray, QUrl, QVariant};
use std::mem::MaybeUninit;
use std::time::Duration;

use crate::util::{IsNonNull, MSecs};

#[cxx::bridge]
mod ffi {
    /// Controls the caching mechanism of `QNetworkAccessManager`.
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkRequestCacheLoadControl {
        /// Always load from network and do not check if the cache has a valid entry (similar to the "Reload" feature in browsers); in addition, force intermediate caches to re-validate.
        AlwaysNetwork,
        /// Default value; load from the network if the cached entry is older than the network entry. This will never return stale data from the cache, but revalidate resources that have become stale.
        PreferNetwork,
        /// Load from cache if available, otherwise load from network. Note that this can return possibly stale (but not expired) items from cache.
        PreferCache,
        /// Only load from cache, indicating error if the item was not cached (i.e., off-line mode).
        AlwaysCache,
    }

    /// List of known header types that [`QNetworkRequest`] parses. Each known header is also represented in raw form with its full HTTP name.
    #[repr(i32)]
    #[derive(Debug)]
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

    /// Indicates if an aspect of the request's loading mechanism has been manually overridden, e.g. by Qt WebKit.
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkRequestLoadControl {
        /// Default value: indicates default behaviour.
        Automatic,
        /// Indicates behaviour has been manually overridden.
        Manual,
    }

    /// This enum lists the possible network request priorities.
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkRequestPriority {
        /// High priority.
        HighPriority = 1,
        /// Normal priority.
        NormalPriority = 3,
        /// Low priority.
        LowPriority = 5,
    }

    /// Indicates whether the Network Access API should automatically follow a HTTP redirect response or not.
    ///
    ///  Note: When Qt handles redirects it will, for legacy and compatibility reasons, issue the redirected request using GET when the server returns a 301 or 302 response, regardless of the original method used, unless it was HEAD.
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkRequestRedirectPolicy {
        /// Not following any redirects.
        ManualRedirectPolicy,
        /// Default value: Only "http"->"http", "http" -> "https" or "https" -> "https" redirects are allowed.
        NoLessSafeRedirectPolicy,
        /// Require the same protocol, host and port. Note, http://example.com and http://example.com:80 will fail with this policy (implicit/explicit ports are considered to be a mismatch).
        SameOriginRedirectPolicy,
        /// Client decides whether to follow each redirect by handling the [`QNetworkReply::redirected`](crate::QNetworkReply::redirected) signal, emitting [`QNetworkReply::redirect_allowed`](crate::QNetworkReply::redirect_allowed) on the [`QNetworkReply`](crate::QNetworkReply) object to allow the redirect or aborting/finishing it to reject the redirect. This can be used, for example, to ask the user whether to accept the redirect, or to decide based on some app-specific configuration.
        UserVerifiedRedirectPolicy,
    }

    extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
        include!("cxx-qt-lib/qlist.h");
        type QList_QByteArray = cxx_qt_lib::QList<QByteArray>;

        #[cfg(cxxqt_qt_version_at_least_6_7)]
        include!("cxx-qt-lib/qanystringview.h");
        #[cfg(cxxqt_qt_version_at_least_6_7)]
        type QAnyStringView<'a> = cxx_qt_lib::QAnyStringView<'a>;

        include!("cxx-qt-io/qhttp2configuration.h");
        type QHttp2Configuration = crate::QHttp2Configuration;

        #[cfg(feature = "ssl")]
        include!("cxx-qt-io/qsslconfiguration.h");
        #[cfg(feature = "ssl")]
        type QSslConfiguration = crate::QSslConfiguration;

        #[cfg(cxxqt_qt_version_at_least_6_5)]
        include!("cxx-qt-io/qhttp1configuration.h");
        #[cfg(cxxqt_qt_version_at_least_6_5)]
        type QHttp1Configuration = crate::QHttp1Configuration;

        #[cfg(cxxqt_qt_version_at_least_6_8)]
        include!("cxx-qt-io/qhttpheaders.h");
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        type QHttpHeaders = crate::QHttpHeaders;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkrequest.h");
        type QNetworkRequestAttribute = super::QNetworkRequestAttribute;
        type QNetworkRequestCacheLoadControl;
        type QNetworkRequestKnownHeaders;
        type QNetworkRequestLoadControl;
        type QNetworkRequestPriority;
        type QNetworkRequestRedirectPolicy;
    }

    unsafe extern "C++" {
        type QNetworkRequest = super::QNetworkRequest;

        /// Returns the threshold for archive bomb checks.
        ///
        /// If the decompressed size of a reply is smaller than this, Qt will simply decompress it, without further checking.
        #[cfg(cxxqt_qt_version_at_least_6_2)]
        #[rust_name = "decompressed_safety_check_threshold"]
        fn decompressedSafetyCheckThreshold(&self) -> i64;

        #[doc(hidden)]
        #[cfg(cxxqt_qt_version_at_least_6_7)]
        #[rust_name = "has_raw_header_view"]
        pub(crate) fn hasRawHeader(&self, header_name: QAnyStringView) -> bool;

        /// Returns `true` if the raw header `header_name` is present in this network request.
        #[cfg(not(cxxqt_qt_version_at_least_6_7))]
        #[rust_name = "has_raw_header"]
        fn hasRawHeader(&self, header_name: &QByteArray) -> bool;

        #[doc(hidden)]
        #[rust_name = "header_or_invalid"]
        fn header(&self, header: QNetworkRequestKnownHeaders) -> QVariant;

        #[cfg(cxxqt_qt_version_at_least_6_8)]
        fn headers(&self) -> QHttpHeaders;

        /// Returns the current parameters that `QNetworkAccessManager` is using for the underlying HTTP/1 connection of this request.
        #[cfg(cxxqt_qt_version_at_least_6_5)]
        #[rust_name = "http1_configuration"]
        fn http1Configuration(&self) -> QHttp1Configuration;

        /// Returns the current parameters that `QNetworkAccessManager` is using for this request and its underlying HTTP/2 connection. This is either a configuration previously set by an application or a default configuration.
        #[rust_name = "http2_configuration"]
        fn http2Configuration(&self) -> QHttp2Configuration;

        /// Returns the maximum number of redirects allowed to be followed for this request.
        #[rust_name = "maximum_redirects_allowed"]
        fn maximumRedirectsAllowed(&self) -> i32;

        /// Returns the host name set for the certificate validation, as set by [`set_peer_verify_name`](QNetworkRequest::set_peer_verify_name). By default this returns a null string.
        #[rust_name = "peer_verify_name"]
        fn peerVerifyName(&self) -> QString;

        /// Return the priority of this request.
        fn priority(&self) -> QNetworkRequestPriority;

        #[doc(hidden)]
        #[cfg(cxxqt_qt_version_at_least_6_7)]
        #[rust_name = "raw_header_view"]
        fn rawHeader(&self, header_name: QAnyStringView) -> QByteArray;

        /// Returns the raw form of header `header_name`. If no such header is present, an empty `QByteArray` is returned, which may be indistinguishable from a header that is present but has no content (use [`has_raw_header`](QNetworkRequest::has_raw_header) to find out if the header exists or not).
        //
        /// Raw headers can be set with [`set_raw_header`](QNetworkRequest::set_raw_header) or with [`set_header`](QNetworkRequest::set_header).
        #[cfg(not(cxxqt_qt_version_at_least_6_7))]
        #[rust_name = "raw_header"]
        fn rawHeader(&self, header_name: &QByteArray) -> bool;

        /// Returns a list of all raw headers that are set in this network request. The list is in the order that the headers were set.
        #[rust_name = "raw_header_list"]
        fn rawHeaderList(&self) -> QList_QByteArray;

        #[doc(hidden)]
        #[rust_name = "set_attribute_qvariant"]
        fn setAttribute(&mut self, code: QNetworkRequestAttribute, value: &QVariant);

        #[cfg(cxxqt_qt_version_at_least_6_2)]
        #[doc(hidden)]
        #[rust_name = "set_decompressed_safety_check_threshold_or_negative"]
        fn setDecompressedSafetyCheckThreshold(&mut self, threshold: i64);

        /// Sets the value of the known header `header` to be `value`, overriding any previously set headers. This operation also sets the equivalent raw HTTP header.
        #[rust_name = "set_header"]
        fn setHeader(&mut self, header: QNetworkRequestKnownHeaders, value: &QVariant);

        /// Sets `new_headers` as headers in this network request, overriding any previously set headers.
        ///
        /// If some headers correspond to the known headers, the values will be parsed and the corresponding parsed form will also be set.
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        #[rust_name = "set_headers"]
        fn setHeaders(&mut self, new_headers: &QHttpHeaders);

        /// Sets request's HTTP/1 parameters from `configuration`.
        #[cfg(cxxqt_qt_version_at_least_6_5)]
        #[rust_name = "set_http1_configuration"]
        fn setHttp1Configuration(&mut self, configuration: &QHttp1Configuration);

        /// Sets request's HTTP/2 parameters from `configuration`.
        ///
        /// **Note:** The configuration must be set prior to making a request.
        ///
        /// **Note:** HTTP/2 multiplexes several streams in a single HTTP/2 connection. This implies that `QNetworkAccessManager` will use the configuration found in the first request from a series of requests sent to the same host.
        #[rust_name = "set_http2_configuration"]
        fn setHttp2Configuration(&mut self, configuration: &QHttp2Configuration);

        /// Sets the maximum number of redirects allowed to be followed for this request to `max_redirects_allowed`.
        #[rust_name = "set_maximum_redirects_allowed"]
        fn setMaximumRedirectsAllowed(&mut self, max_redirects_allowed: i32);

        /// Sets `peer_name` as host name for the certificate validation, instead of the one used for the TCP connection.
        #[rust_name = "set_peer_verify_name"]
        fn setPeerVerifyName(&mut self, peer_name: &QString);

        /// Set the priority of this request to `priority`.
        ///
        /// **Note:** The `priority` is only a hint to the network access manager. It can use it or not. Currently it is used for HTTP to decide which request should be sent first to a server.
        #[rust_name = "set_priority"]
        fn setPriority(&mut self, priority: QNetworkRequestPriority);

        /// Sets the header `header_name` to be of value `header_value`. If `header_name` corresponds to a known header (see [`QNetworkRequestKnownHeaders`]), the raw format will be parsed and the corresponding "cooked" header will be set as well.
        ///
        /// **Note:** Setting the same header twice overrides the previous setting. To accomplish the behaviour of multiple HTTP headers of the same name, you should concatenate the two values, separating them with a comma (",") and set one single raw header.
        ///
        /// **Note:** Since Qt 6.8, the header field names are normalized by converting them to lowercase.
        #[rust_name = "set_raw_header"]
        fn setRawHeader(&mut self, header_name: &QByteArray, header_value: &QByteArray);

        /// Sets this network request's SSL configuration to be `config`. The settings that apply are the private key, the local certificate, the TLS protocol (e.g. TLS 1.3), the CA certificates and the ciphers that the SSL backend is allowed to use.
        #[cfg(feature = "ssl")]
        #[rust_name = "set_ssl_configuration"]
        fn setSslConfiguration(&mut self, configuration: &QSslConfiguration);

        #[doc(hidden)]
        #[cfg(any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_15))]
        #[rust_name = "set_transfer_timeout_msecs"]
        fn setTransferTimeout(&mut self, timeout: i32);

        /// Sets the URL this network request is referring to be `url`.
        #[rust_name = "set_url"]
        fn setUrl(&mut self, url: &QUrl);

        /// Returns this network request's SSL configuration.
        #[cfg(feature = "ssl")]
        #[rust_name = "ssl_configuration"]
        fn sslConfiguration(&self) -> QSslConfiguration;

        #[doc(hidden)]
        #[rust_name = "transfer_timeout_msecs"]
        fn transferTimeout(&self) -> i32;

        /// Returns the URL this network request is referring to.
        fn url(&self) -> QUrl;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qnetworkrequest_attribute"]
        fn qnetworkrequestAttribute(
            request: &QNetworkRequest,
            code: QNetworkRequestAttribute,
        ) -> QVariant;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkrequest_drop"]
        fn drop(request: &mut QNetworkRequest);

        #[rust_name = "qnetworkrequest_init_default"]
        fn construct() -> QNetworkRequest;
        #[rust_name = "qnetworkrequest_init_url"]
        fn construct(url: &QUrl) -> QNetworkRequest;
        #[rust_name = "qnetworkrequest_clone"]
        fn construct(other: &QNetworkRequest) -> QNetworkRequest;

        #[rust_name = "qnetworkrequest_eq"]
        fn operatorEq(a: &QNetworkRequest, b: &QNetworkRequest) -> bool;
    }
}

pub use ffi::{
    QNetworkRequestCacheLoadControl, QNetworkRequestKnownHeaders, QNetworkRequestLoadControl,
    QNetworkRequestPriority, QNetworkRequestRedirectPolicy,
};

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
    /// Constructs a `QNetworkRequest` object with no URL to be requested. Use [`set_url`](crate::QNetworkRequest) to set one.
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

impl QNetworkRequest {
    /// Returns the attribute associated with the code `code`. If the attribute has not been set, it returns `None`.
    pub fn attribute(&self, code: QNetworkRequestAttribute) -> Option<QVariant> {
        ffi::qnetworkrequest_attribute(self, code).nonnull()
    }

    /// Returns `true` if the raw header `header_name` is present in this network request.
    #[cfg(cxxqt_qt_version_at_least_6_7)]
    pub fn has_raw_header<'a, T>(&self, header_name: T) -> bool
    where
        T: Into<QAnyStringView<'a>>,
    {
        self.has_raw_header_view(header_name.into())
    }

    /// Returns the value of the known network header `header` if it is present in this request. If it is not present, returns `None`.
    pub fn header(&self, header: QNetworkRequestKnownHeaders) -> Option<QVariant> {
        self.header_or_invalid(header).nonnull()
    }

    /// Returns the raw form of header `header_name`. If no such header is present, an empty `QByteArray` is returned, which may be indistinguishable from a header that is present but has no content (use [`has_raw_header`](QNetworkRequest::has_raw_header) to find out if the header exists or not).
    //
    /// Raw headers can be set with [`set_raw_header`](QNetworkRequest::set_raw_header) or with [`set_header`](QNetworkRequest::set_header).
    #[cfg(cxxqt_qt_version_at_least_6_7)]
    pub fn raw_header<'a, T>(&self, header_name: T) -> QByteArray
    where
        T: Into<QAnyStringView<'a>>,
    {
        self.raw_header_view(header_name.into())
    }

    /// Sets the attribute associated with code `code` to be value `value`. If the attribute is already set, the previous value is discarded.
    pub fn set_attribute<T>(&mut self, code: QNetworkRequestAttribute, value: T)
    where
        T: Into<QVariant>,
    {
        self.set_attribute_qvariant(code, &value.into());
    }

    #[allow(clippy::doc_markdown)]
    /// Sets the `threshold` for archive bomb checks.
    ///
    /// Some supported compression algorithms can, in a tiny compressed file, encode a spectacularly huge decompressed file. This is only possible if the decompressed content is extremely monotonous, which is seldom the case for real files being transmitted in good faith: files exercising such insanely high compression ratios are typically payloads of buffer-overrun attacks, or denial-of-service (by using up too much memory) attacks. Consequently, files that decompress to huge sizes, particularly from tiny compressed forms, are best rejected as suspected malware.
    ///
    /// If a reply's decompressed size is bigger than this threshold (by default, 10 MiB, i.e. 10 * 1024 * 1024), Qt will check the compression ratio: if that is unreasonably large (40:1 for GZip and Deflate, or 100:1 for Brotli and ZStandard), the reply will be treated as an error. Setting the threshold to `None` disables this check.
    #[cfg(cxxqt_qt_version_at_least_6_2)]
    pub fn set_decompressed_safety_check_threshold(&mut self, threshold: Option<i64>) {
        self.set_decompressed_safety_check_threshold_or_negative(threshold.unwrap_or(-1));
    }

    /// Sets `timeout` as the transfer timeout.
    ///
    /// Transfers are aborted if no bytes are transferred before the timeout expires. `None` means no timer is set. If this function is not called, the timeout is disabled.
    #[cfg(any(cxxqt_qt_version_at_least_6_0, cxxqt_qt_version_at_least_5_15))]
    pub fn set_transfer_timeout(&mut self, duration: Option<Duration>) {
        let msecs = match duration {
            Some(duration) => duration.msecs(),
            None => 0,
        };
        self.set_transfer_timeout_msecs(msecs);
    }

    /// Returns the timeout used for transfers.
    pub fn transfer_timeout(&self) -> Option<Duration> {
        let msecs = u64::try_from(self.transfer_timeout_msecs()).ok()?;
        if msecs == 0 {
            None
        } else {
            Some(Duration::from_millis(msecs))
        }
    }

    /// Unsets the attribute associated with code `code`.
    pub fn unset_attribute(&mut self, code: QNetworkRequestAttribute) {
        self.set_attribute_qvariant(code, &QVariant::default());
    }
}

impl From<&QUrl> for QNetworkRequest {
    /// Constructs a `QNetworkRequest` object with `url` as the URL to be requested.
    fn from(value: &QUrl) -> Self {
        ffi::qnetworkrequest_init_url(value)
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkRequest {
    type Id = type_id!("QNetworkRequest");
    type Kind = cxx::kind::Trivial;
}

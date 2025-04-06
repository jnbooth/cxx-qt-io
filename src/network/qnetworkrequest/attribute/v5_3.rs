#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkRequestAttribute {
        /// Replies only, type: `QMetaType::Int` (no default) Indicates the HTTP status code received from the HTTP server (like 200, 304, 404, 401, etc.). If the connection was not HTTP-based, this attribute will not be present.
        HttpStatusCodeAttribute,
        /// Replies only, type: `QMetaType::QByteArray` (no default) Indicates the HTTP reason phrase as received from the HTTP server (like "Ok", "Found", "Not Found", "Access Denied", etc.) This is the human-readable representation of the status code (see above). If the connection was not HTTP-based, this attribute will not be present. Note: The reason phrase is not used when using HTTP/2.
        HttpReasonPhraseAttribute,
        /// Replies only, type: `QMetaType::QUrl` (no default) If present, it indicates that the server is redirecting the request to a different URL. The Network Access API does follow redirections by default, unless `QNetworkRequest::ManualRedirectPolicy` is used. Additionally, if `QNetworkRequest::UserVerifiedRedirectPolicy` is used, then this attribute will be set if the redirect was not followed. The returned URL might be relative. Use `QUrl::resolved`() to create an absolute URL out of it.
        RedirectionTargetAttribute,
        /// Replies only, type: `QMetaType::Bool` (default: false) Indicates whether the data was obtained through an encrypted (secure) connection.
        ConnectionEncryptedAttribute,
        /// Requests only, type: `QMetaType::Int` (default: `QNetworkRequest::PreferNetwork`) Controls how the cache should be accessed. The possible values are those of `QNetworkRequest::CacheLoadControl`. Note that the default `QNetworkAccessManager` implementation does not support caching. However, this attribute may be used by certain backends to modify their requests (for example, for caching proxies).
        CacheLoadControlAttribute,
        /// Requests only, type: `QMetaType::Int` (default: `QNetworkRequest::PreferNetwork`) Controls how the cache should be accessed. The possible values are those of `QNetworkRequest::CacheLoadControl`. Note that the default `QNetworkAccessManager` implementation does not support caching. However, this attribute may be used by certain backends to modify their requests (for example, for caching proxies).
        CacheSaveControlAttribute,
        /// Replies only, type: `QMetaType::Bool` (default: false) Indicates whether the data was obtained from cache or not.
        SourceIsFromCacheAttribute,
        /// Requests only, type: `QMetaType::Bool` (default: false) Indicates whether the `QNetworkAccessManager` code is allowed to buffer the upload data, e.g. when doing a HTTP POST. When using this flag with sequential upload data, the ContentLengthHeader header must be set.
        DoNotBufferUploadDataAttribute,
        /// Requests only, type: `QMetaType::Bool` (default: false) Indicates whether the `QNetworkAccessManager` code is allowed to use HTTP pipelining with this request.
        HttpPipeliningAllowedAttribute,
        /// Replies only, type: `QMetaType::Bool` Indicates whether the HTTP pipelining was used for receiving this reply.
        HttpPipeliningWasUsedAttribute,
        /// Requests only, type: `QMetaType::QByteArray` Holds the value for the custom HTTP verb to send (destined for usage of other verbs than GET, POST, PUT and DELETE). This verb is set when calling `QNetworkAccessManager::send_custom_request()`.
        CustomVerbAttribute,
        /// Requests only, type: `QMetaType::Int` (default: `QNetworkRequest::Automatic`) Indicates whether to send 'Cookie' headers in the request. This attribute is set to false by Qt WebKit when creating a cross-origin XMLHttpRequest where withCredentials has not been set explicitly to true by the Javascript that created the request. See [here](http://www.w3.org/TR/XMLHttpRequest2/#credentials-flag) for more information.
        CookieLoadControlAttribute,
        /// Requests only, type: `QMetaType::Int` (default: `QNetworkRequest::Automatic`) Indicates whether to save 'Cookie' headers received from the server in reply to the request. This attribute is set to false by Qt WebKit when creating a cross-origin XMLHttpRequest where withCredentials has not been set explicitly to true by the Javascript that created the request. See [here](http://www.w3.org/TR/XMLHttpRequest2/#credentials-flag) for more information.
        AuthenticationReuseAttribute,
        /// Requests only, type: `QMetaType::Int` (default: `QNetworkRequest::Automatic`) Indicates whether to save 'Cookie' headers received from the server in reply to the request. This attribute is set to false by Qt WebKit when creating a cross-origin XMLHttpRequest where withCredentials has not been set explicitly to true by the Javascript that created the request. See [here](http://www.w3.org/TR/XMLHttpRequest2/#credentials-flag) for more information.
        CookieSaveControlAttribute,
        #[doc(hidden)]
        MaximumDownloadBufferSizeAttribute, // internal
        #[doc(hidden)]
        DownloadBufferAttribute, // internal
        #[doc(hidden)]
        SynchronousRequestAttribute, // internal
        /// Type: `QMetaType::Bool` (default: false) Indicates that this is a background transfer, rather than a user initiated transfer. Depending on the platform, background transfers may be subject to different policies.
        BackgroundRequestAttribute,
        /// Requests only, type: `QMetaType::Bool` (default: false) Indicates whether the `QNetworkAccessManager` code is allowed to use SPDY with this request. This applies only to SSL requests, and depends on the server supporting SPDY.
        SpdyAllowedAttribute,
        /// Replies only, type: `QMetaType::Bool` Indicates whether SPDY was used for receiving this reply.
        SpdyWasUsedAttribute,

        /// Special type. Additional information can be passed in QVariants with types ranging from User to UserMax. The default implementation of Network Access will ignore any request attributes in this range and it will not produce any attributes in this range in replies. The range is reserved for extensions of `QNetworkAccessManager`.
        User = 1000,
        /// Special type. See User.
        UserMax = 32767,
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkrequest.h");
        type QNetworkRequestAttribute;
    }
}

pub use ffi::QNetworkRequestAttribute;

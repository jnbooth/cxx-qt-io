#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum QNetworkRequestAttribute {
        /// Replies only, type: `QMetaType::Int` (no default) Indicates the HTTP status code received from the HTTP server (like 200, 304, 404, 401, etc.). If the connection was not HTTP-based, this attribute will not be present.
        HttpStatusCodeAttribute,
        /// Replies only, type: `QMetaType::QByteArray` (no default) Indicates the HTTP reason phrase as received from the HTTP server (like "Ok", "Found", "Not Found", "Access Denied", etc.) This is the human-readable representation of the status code (see above). If the connection was not HTTP-based, this attribute will not be present. Note: The reason phrase is not used when using HTTP/2.
        HttpReasonPhraseAttribute,
        /// Replies only, type: `QMetaType::QUrl` (no default) If present, it indicates that the server is redirecting the request to a different URL. The Network Access API does follow redirections by default, unless `QNetworkRequest::ManualRedirectPolicy` is used. Additionally, if `QNetworkRequest::UserVerifiedRedirectPolicy` is used, then this attribute will be set if the redirect was not followed. The returned URL might be relative. Use `QUrl::resolved`() to create an absolute URL out of it.
        RedirectionTargetAttribute,
        /// Replies only, type: `QMetaType::Bool` (default: false) Indicates whether the data was obtained through an encrypted (secure) connection.
        ConnectionEncryptedAttribute,
        /// Requests only, type: `QMetaType::Int` (default: `QNetworkRequest::PreferNetwork`) Controls how the cache should be accessed. The possible values are those of `QNetworkRequest::CacheLoadControl`. Note that the default [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html) implementation does not support caching. However, this attribute may be used by certain backends to modify their requests (for example, for caching proxies).
        CacheLoadControlAttribute,
        /// Requests only, type: `QMetaType::Int` (default: `QNetworkRequest::PreferNetwork`) Controls how the cache should be accessed. The possible values are those of `QNetworkRequest::CacheLoadControl`. Note that the default [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html) implementation does not support caching. However, this attribute may be used by certain backends to modify their requests (for example, for caching proxies).
        CacheSaveControlAttribute,
        /// Replies only, type: `QMetaType::Bool` (default: false) Indicates whether the data was obtained from cache or not.
        SourceIsFromCacheAttribute,
        /// Requests only, type: `QMetaType::Bool` (default: false) Indicates whether the [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html) code is allowed to buffer the upload data, e.g. when doing a HTTP POST. When using this flag with sequential upload data, the ContentLengthHeader header must be set.
        DoNotBufferUploadDataAttribute,
        /// Requests only, type: `QMetaType::Bool` (default: false) Indicates whether the [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html) code is allowed to use HTTP pipelining with this request.
        HttpPipeliningAllowedAttribute,
        /// Replies only, type: `QMetaType::Bool` Indicates whether the HTTP pipelining was used for receiving this reply.
        HttpPipeliningWasUsedAttribute,
        /// Requests only, type: `QMetaType::QByteArray` Holds the value for the custom HTTP verb to send (destined for usage of other verbs than GET, POST, PUT and DELETE). This verb is set when calling [`QNetworkAccessManager::send_custom_request`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#sendCustomRequest).
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
        /// Requests only, type: `QMetaType::Bool` (default: false) Indicates whether all upload signals should be emitted. By default, the uploadProgress signal is emitted only in 100 millisecond intervals.
        EmitAllUploadProgressSignalsAttribute,
        /// Requests only, type: `QMetaType::Bool` (default: true) Indicates whether the [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html) code is allowed to use HTTP/2 with this request. This applies to SSL requests or 'cleartext' HTTP/2 if Http2CleartextAllowedAttribute is set.
        Http2AllowedAttribute,
        /// Replies only, type: `QMetaType::Bool` (default: false) Indicates whether HTTP/2 was used for receiving this reply.
        Http2WasUsedAttribute,
        /// Replies only, type `QMetaType::Int` Holds the original content-length attribute before being invalidated and removed from the header when the data is compressed and the request was marked to be decompressed automatically.
        OriginalContentLengthAttribute,
        /// Requests only, type: `QMetaType::Int`, should be one of the `QNetworkRequest::RedirectPolicy` values (default: `NoLessSafeRedirectPolicy`).
        RedirectPolicyAttribute,
        /// Requests only, type: `QMetaType::Bool` (default: false) If set, this attribute will force [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html) to use HTTP/2 protocol without initial HTTP/2 protocol negotiation. Use of this attribute implies prior knowledge that a particular server supports HTTP/2. The attribute works with SSL or with 'cleartext' HTTP/2 if Http2CleartextAllowedAttribute is set. If a server turns out to not support HTTP/2, when HTTP/2 direct was specified, [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html) gives up, without attempting to fall back to HTTP/1.1. If both Http2AllowedAttribute and Http2DirectAttribute are set, Http2DirectAttribute takes priority.
        Http2DirectAttribute,
        #[doc(hidden)]
        ResourceTypeAttribute, // internal
        /// Requests only, type: `QMetaType::Bool` (default: false) If set, this attribute will make [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html) delete the `QNetworkReply` after having emitted "finished".
        AutoDeleteReplyOnFinishAttribute,
        /// Requests only, type: `QMetaType::Int` To set when the TCP connections to a server (HTTP1 and HTTP2) should be closed after the last pending request had been processed.
        ConnectionCacheExpiryTimeoutSecondsAttribute,
        /// Requests only, type: `QMetaType::Bool` (default: false) If set, this attribute will tell [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html) to attempt an upgrade to HTTP/2 over cleartext (also known as h2c). Until Qt 7 the default value for this attribute can be overridden to true by setting the QT_NETWORK_H2C_ALLOWED environment variable. This attribute is ignored if the Http2AllowedAttribute is not set.
        Http2CleartextAllowedAttribute,
        /// Requests only, type: `QMetaType::Bool` (default: false) Indicates if the underlying XMLHttpRequest cross-site Access-Control requests should be made using credentials. Has no effect on same-origin requests. This only affects the WebAssembly platform.
        UseCredentialsAttribute,

        /// Special type. Additional information can be passed in QVariants with types ranging from User to UserMax. The default implementation of Network Access will ignore any request attributes in this range and it will not produce any attributes in this range in replies. The range is reserved for extensions of [`QNetworkAccessManager`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html).
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

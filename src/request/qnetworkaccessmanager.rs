use std::fmt;
use std::pin::Pin;
use std::time::Duration;

use cxx::memory::UniquePtrTarget;
use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt_lib::{QByteArray, QString};

use crate::core::qobject::in_same_thread;
use crate::qobject::debug_qobject;
use crate::util::{unpin_for_qt, upcast_mut};
use crate::{
    QAbstractNetworkCache, QHttpMultiPart, QIODevice, QNetworkCookieJar, QNetworkReply,
    QNetworkRequest,
};

#[cxx_qt::bridge]
mod ffi {
    /// Indicates the operation this reply is processing.
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum QNetworkAccessManagerOperation {
        /// retrieve headers operation (created with [`QNetworkAccessManager::head`])
        HeadOperation = 1,
        /// retrieve headers and download contents (created with [`QNetworkAccessManager::get`])
        GetOperation,
        /// upload contents operation (created with [`QNetworkAccessManager::put`])
        PutOperation,
        /// send the contents of an HTML form for processing via HTTP POST (created with [`QNetworkAccessManager::post`])
        PostOperation,
        /// delete contents operation (created with [`QNetworkAccessManager::delete`])
        DeleteOperation,
        /// custom operation (created with [`QNetworkAccessManager::send_custom_request`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#sendCustomRequest))
        CustomOperation,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        include!("cxx-qt-io/qauthenticator.h");
        type QAuthenticator = crate::QAuthenticator;
        include!("cxx-qt-io/qhstspolicy.h");
        type QHstsPolicy = crate::QHstsPolicy;
        include!("cxx-qt-io/qabstractnetworkcache.h");
        type QAbstractNetworkCache = crate::QAbstractNetworkCache;
        include!("cxx-qt-io/qhttpmultipart.h");
        type QHttpMultiPart = crate::QHttpMultiPart;
        include!("cxx-qt-io/qiodevice.h");
        type QIODevice = crate::QIODevice;
        include!("cxx-qt-io/qnetworkcookiejar.h");
        type QNetworkCookieJar = crate::QNetworkCookieJar;
        include!("cxx-qt-io/qnetworkproxy.h");
        type QNetworkProxy = crate::QNetworkProxy;
        include!("cxx-qt-io/qnetworkreply.h");
        type QNetworkReply = crate::QNetworkReply;
        include!("cxx-qt-io/qnetworkrequest.h");
        type QNetworkRequest = crate::QNetworkRequest;
        type QNetworkRequestRedirectPolicy = crate::QNetworkRequestRedirectPolicy;

        include!("cxx-qt-io/qlist.h");
        type QList_QHstsPolicy = cxx_qt_lib::QList<QHstsPolicy>;
    }

    #[cfg(feature = "ssl")]
    extern "C++" {
        include!("cxx-qt-io/qsslconfiguration.h");
        type QSslConfiguration = crate::QSslConfiguration;
        include!("cxx-qt-io/qsslpresharedkeyauthenticator.h");
        type QSslPreSharedKeyAuthenticator = crate::QSslPreSharedKeyAuthenticator;
        type QList_QSslError = cxx_qt_lib::QList<crate::QSslError>;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkaccessmanager.h");
        type QNetworkAccessManagerOperation;
    }

    unsafe extern "C++Qt" {
        /// The `QNetworkAccessManager` class allows the application to send network requests and receive replies.
        ///
        /// Qt Documentation: [QNetworkAccessManager](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#details)
        #[qobject]
        #[base = QObject]
        type QNetworkAccessManager;

        /// Adds HTTP Strict Transport Security policies into HSTS cache. `known_hosts` contains the known hosts that have `QHstsPolicy` information.
        ///
        /// **Note:** An expired policy will remove a known host from the cache, if previously present.
        ///
        /// **Note:** While processing HTTP responses, `QNetworkAccessManager` can also update the HSTS cache, removing or updating exitsting policies or introducing new knownHosts. The current implementation thus is server-driven, client code can provide `QNetworkAccessManager` with previously known or discovered policies, but this information can be overridden by `"Strict-Transport-Security"` response headers.
        #[rust_name = "add_strict_transport_security_hosts"]
        fn addStrictTransportSecurityHosts(
            self: Pin<&mut QNetworkAccessManager>,
            known_hosts: &QList_QHstsPolicy,
        );

        /// Returns the true if this object is currently configured to automatically delete [`QNetworkReplies`](QNetworkReply), false otherwise.
        #[doc(hidden)]
        #[rust_name = "auto_delete_replies"]
        fn autoDeleteReplies(self: &QNetworkAccessManager) -> bool;

        /// Returns the cache that is used to store data obtained from the network.
        fn cache(self: &QNetworkAccessManager) -> *mut QAbstractNetworkCache;

        /// Flushes the internal cache of authentication data and network connections.
        ///
        /// This function is useful for doing auto tests.
        #[rust_name = "clear_access_cache"]
        fn clearAccessCache(self: Pin<&mut QNetworkAccessManager>);

        /// Flushes the internal cache of network connections. In contrast to [`clear_access_cache`](QNetworkAccessManager::clear_access_cache) the authentication data is preserved.
        #[rust_name = "clear_connection_cache"]
        fn clearConnectionCache(self: Pin<&mut QNetworkAccessManager>);

        /// Initiates a connection to the host given by `host_name` at port `port`. This function is useful to complete the TCP handshake to a host before the HTTP request is made, resulting in a lower network latency.
        ///
        /// **Note:** This function has no possibility to report errors.
        #[rust_name = "connect_to_host"]
        fn connectToHost(self: Pin<&mut QNetworkAccessManager>, host_name: &QString, port: u16);

        /// Initiates a connection to the host given by `host_name` at port `port`, using `ssl_configuration`. This function is useful to complete the TCP and SSL handshake to a host before the HTTPS request is made, resulting in a lower network latency.
        ///
        /// **Note:** Preconnecting a HTTP/2 connection can be done by calling [`QSslConfiguration::set_allowed_next_protocols`] on `ssl_configuration` with [`QSslConfiguration::ALPNProtocolHTTP2`] contained in the list of allowed protocols. When using HTTP/2, one single connection per host is enough, i.e. calling this method multiple times per host will not result in faster network transactions.
        ///
        /// **Note:** This function has no possibility to report errors.
        #[cfg(feature = "ssl")]
        #[rust_name = "connect_to_host_encrypted"]
        fn connectToHostEncrypted(
            self: Pin<&mut QNetworkAccessManager>,
            host_name: &QString,
            port: u16,
            ssl_configuration: &QSslConfiguration,
        );

        /// Initiates a connection to the host given by `host_name` at port `port`, using `ssl_configuration` with `peer_name` set to be the `host_name` used for certificate validation. This function is useful to complete the TCP and SSL handshake to a host before the HTTPS request is made, resulting in a lower network latency.
        ///
        /// **Note:** Preconnecting a HTTP/2 connection can be done by calling [`QSslConfiguration::set_allowed_next_protocols`] on `ssl_configuration` with [`QSslConfiguration::ALPNProtocolHTTP2`] contained in the list of allowed protocols. When using HTTP/2, one single connection per host is enough, i.e. calling this method multiple times per host will not result in faster network transactions.
        ///
        /// **Note:** This function has no possibility to report errors.
        #[cfg(feature = "ssl")]
        #[rust_name = "connect_to_host_encrypted_with"]
        fn connectToHostEncrypted(
            self: Pin<&mut QNetworkAccessManager>,
            host_name: &QString,
            port: u16,
            ssl_configuration: &QSslConfiguration,
            peer_name: &QString,
        );

        /// Returns the `QNetworkCookieJar` that is used to store cookies obtained from the network as well as cookies that are about to be sent.
        #[rust_name = "cookie_jar"]
        fn cookieJar(self: &QNetworkAccessManager) -> *mut QNetworkCookieJar;

        /// Sends a request to delete the resource identified by the URL of `request`.
        ///
        /// **Note:** This feature is currently available for HTTP only, performing an `HTTP DELETE` request.
        #[rust_name = "delete"]
        fn deleteResource(
            self: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
        ) -> *mut QNetworkReply;

        /// If `enabled` is true, the internal HSTS cache will use a persistent store to read and write HSTS policies. `store_dir` defines where this store will be located.
        ///
        /// **Note:** If HSTS cache already contains HSTS policies by the time persistent store is enabled, these policies will be preserved in the store. In case both cache and store contain the same known hosts, policies from cache are considered to be more up-to-date (and thus will overwrite the previous values in the store). If this behavior is undesired, enable HSTS store before enabling Strict Transport Security. By default, the persistent store of HSTS policies is disabled.
        #[rust_name = "enable_strict_transport_security_store_in"]
        fn enableStrictTransportSecurityStore(
            self: Pin<&mut QNetworkAccessManager>,
            enabled: bool,
            store_dir: &QString,
        );

        #[doc(hidden)]
        #[rust_name = "get_raw"]
        fn get(
            self: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
        ) -> *mut QNetworkReply;

        #[doc(hidden)]
        #[rust_name = "head_raw"]
        fn head(
            self: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
        ) -> *mut QNetworkReply;

        /// Returns `true` if HTTP Strict Transport Security (HSTS) was enabled. By default HSTS is disabled.
        #[rust_name = "is_strict_transport_security_enabled"]
        fn isStrictTransportSecurityEnabled(self: &QNetworkAccessManager) -> bool;

        /// Returns `true` if HSTS cache uses a permanent store to load and store HSTS policies.
        #[rust_name = "is_strict_transport_security_store_enabled"]
        fn isStrictTransportSecurityStoreEnabled(self: &QNetworkAccessManager) -> bool;

        #[doc(hidden)]
        #[rust_name = "post_data_raw"]
        fn post(
            self: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
            data: &QByteArray,
        ) -> *mut QNetworkReply;

        /// # Safety
        ///
        /// `data` must be opened for reading when this function is called and must remain valid until the `finished` signal is emitted for this reply.
        #[doc(hidden)]
        #[rust_name = "post_device_raw"]
        unsafe fn post(
            self: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
            device: *mut QIODevice,
        ) -> *mut QNetworkReply;

        /// # Safety
        ///
        /// `multi_part` must remain valid until the `finished` signal is emitted for this reply.
        #[doc(hidden)]
        #[rust_name = "post_http_raw"]
        unsafe fn post(
            self: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
            multi_part: *mut QHttpMultiPart,
        ) -> *mut QNetworkReply;

        /// Returns the `QNetworkProxy` that the requests sent using this `QNetworkAccessManager` object will use. The default value for the proxy is [`QNetworkProxyProxyType::DefaultProxy`](crate::QNetworkProxyProxyType::DefaultProxy).
        fn proxy(self: &QNetworkAccessManager) -> QNetworkProxy;

        #[doc(hidden)]
        #[rust_name = "put_data_raw"]
        fn put(
            self: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
            data: &QByteArray,
        ) -> *mut QNetworkReply;

        /// # Safety
        ///
        /// `data` must be opened for reading when this function is called and must remain valid until the `finished` signal is emitted for this reply.
        #[doc(hidden)]
        #[rust_name = "put_device_raw"]
        unsafe fn put(
            self: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
            device: *mut QIODevice,
        ) -> *mut QNetworkReply;

        /// # Safety
        ///
        /// `multi_part` must remain valid until the `finished` signal is emitted for this reply.
        #[doc(hidden)]
        #[rust_name = "put_http_raw"]
        unsafe fn put(
            self: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
            multi_part: *mut QHttpMultiPart,
        ) -> *mut QNetworkReply;

        /// Returns the redirect policy that is used when creating new requests.
        #[rust_name = "redirect_policy"]
        fn redirectPolicy(self: &QNetworkAccessManager) -> QNetworkRequestRedirectPolicy;

        /// **Warning:** passing `false` to this function will cause memory leaks unless
        /// QNetworkReplies are manually deleted.
        #[doc(hidden)]
        #[rust_name = "set_auto_delete_replies"]
        fn setAutoDeleteReplies(self: Pin<&mut QNetworkAccessManager>, should_auto_delete: bool);

        /// Sets the manager's network cache to be the `cache` specified. The cache is used for all requests dispatched by the manager. **The manager takes ownership of `cache`.**
        ///
        /// Use this function to set the network cache object to a class that implements additional features, like saving the cookies to permanent storage.
        ///
        /// # Safety
        ///
        /// `cache` must be either null or a valid pointer to an object in the same thread as this manager. Note that ownership of the cache is transferred to the manager.
        #[rust_name = "set_cache_raw"]
        unsafe fn setCache(
            self: Pin<&mut QNetworkAccessManager>,
            cache: *mut QAbstractNetworkCache,
        );

        /// Sets the manager's cookie jar to be the `cookie_jar` specified. The cookie jar is used by all requests dispatched by the manager. **The manager takes ownership of `cookie_jar`.**
        ///
        /// Use this function to set the cookie jar object to a class that implements additional features, like saving the cookies to permanent storage.
        ///
        /// # Safety
        ///
        /// `cookie_jar` must be either null or a valid pointer to an object in the same thread as this manager. Note that ownership of the cookie jar is transferred to the manager.
        #[rust_name = "set_cookie_jar_raw"]
        unsafe fn setCookieJar(
            self: Pin<&mut QNetworkAccessManager>,
            cookie_jar: *mut QNetworkCookieJar,
        );

        /// Sets the proxy to be used in future requests to be `proxy`. This does not affect requests that have already been sent. The [`proxy_authentication_required`](QNetworkAccessManager::proxy_authentication_required) signal will be emitted if the proxy requests authentication.
        ///
        /// A proxy set with this function will be used for all requests issued by `QNetworkAccessManager`.
        #[rust_name = "set_proxy"]
        fn setProxy(self: Pin<&mut QNetworkAccessManager>, proxy: &QNetworkProxy);

        /// Sets the manager's redirect policy to be the `policy` specified. This policy will affect all subsequent requests created by the manager.
        ///
        /// Use this function to enable or disable HTTP redirects on the manager's level.
        ///
        /// **Note:** When creating a request [`QNetworkRequestAttribute::RedirectPolicyAttribute`](crate::QNetworkRequestAttribute::RedirectPolicyAttribute) has the highest priority, next by priority the manager's policy.
        ///
        /// The default value is [`QNetworkRequestRedirectPolicy::NoLessSafeRedirectPolicy`]. Clients relying on manual redirect handling are encouraged to set this policy explicitly in their code.
        #[rust_name = "set_redirect_policy"]
        fn setRedirectPolicy(
            self: Pin<&mut QNetworkAccessManager>,
            policy: QNetworkRequestRedirectPolicy,
        );

        /// If `enabled` is true, `QNetworkAccessManager` follows the HTTP Strict Transport Security policy (HSTS, RFC6797). When processing a request, `QNetworkAccessManager` automatically replaces the "http" scheme with "https" and uses a secure transport for HSTS hosts. If it's set explicitly, port 80 is replaced by port 443.
        ///
        /// When HSTS is enabled, for each HTTP response containing HSTS header and received over a secure transport, `QNetworkAccessManager` will update its HSTS cache, either remembering a host with a valid policy or removing a host with an expired or disabled HSTS policy.
        #[rust_name = "set_strict_transport_security_enabled"]
        fn setStrictTransportSecurityEnabled(self: Pin<&mut QNetworkAccessManager>, enabled: bool);

        /// Returns the list of HTTP Strict Transport Security policies. This list can differ from what was initially set via [`add_strict_transport_security_hosts`](QNetworkAccessManager::add_strict_transport_security_hosts) if HSTS cache was updated from a "Strict-Transport-Security" response header.
        #[rust_name = "strict_transport_security_hosts"]
        fn strictTransportSecurityHosts(self: &QNetworkAccessManager) -> QList_QHstsPolicy;

        /// Lists all the URL schemes supported by the access manager.
        #[rust_name = "supported_schemes"]
        fn supportedSchemes(self: &QNetworkAccessManager) -> QStringList;

        /// This signal is emitted whenever a final server requests authentication before it delivers the requested contents. The slot connected to this signal should fill the credentials for the contents (which can be determined by inspecting the `reply` object) in the `authenticator` object.
        /////
        /// `QNetworkAccessManager` will cache the credentials internally and will send the same values if the server requires authentication again, without emitting this signal. If it rejects the credentials, this signal will be emitted again.
        ///
        /// **Note:** To have the request not send credentials you must not call [`QAuthenticator::set_user`] or [`QAuthenticator::set_password`] on the `authenticator` object. This will result in the [`finished`](QNetworkAccessManager::finished) signal being emitted with a `QNetworkReply` with error [`QNetworkReplyNetworkError::AuthenticationRequiredError`](crate::QNetworkReplyNetworkError::AuthenticationRequiredError).
        ///
        /// **Note:** It is not possible to use a [`ConnectionType::QueuedConnection`](cxx_qt_lib::ConnectionType::QueuedConnection) to connect to this signal, as the connection will fail if the authenticator has not been filled in with new information when the signal returns.
        #[qsignal]
        #[rust_name = "authentication_required"]
        unsafe fn authenticationRequired(
            self: Pin<&mut QNetworkAccessManager>,
            reply: *mut QNetworkReply,
            authenticator: *mut QAuthenticator,
        );

        /// This signal is emitted when an SSL/TLS session has successfully completed the initial handshake. At this point, no user data has been transmitted. The signal can be used to perform additional checks on the certificate chain, for example to notify users when the certificate for a website has changed. The reply parameter specifies which network reply is responsible. If the reply does not match the expected criteria then it should be aborted by calling [`QNetworkReply::abort`] by a slot connected to this signal. The SSL configuration in use can be inspected using the [`QNetworkReply::ssl_configuration`] method.
        ///
        /// Internally, `QNetworkAccessManager` may open multiple connections to a server, in order to allow it process requests in parallel. These connections may be reused, which means that this signal would not be emitted. This means that you are only guaranteed to receive this signal for the first connection to a site in the lifespan of the `QNetworkAccessManager`.
        #[qsignal]
        unsafe fn encrypted(self: Pin<&mut QNetworkAccessManager>, reply: *mut QNetworkReply);

        /// This signal is emitted whenever a pending network reply is finished. The reply parameter will contain a pointer to the reply that has just finished. This signal is emitted in tandem with the [`QNetworkReply::finished`] signal.
        #[qsignal]
        unsafe fn finished(self: Pin<&mut QNetworkAccessManager>, reply: *mut QNetworkReply);

        /// This signal is emitted if the SSL/TLS handshake negotiates a PSK ciphersuite, and therefore a PSK authentication is then required. The reply object is the `QNetworkReply` that is negotiating such ciphersuites.
        ///
        /// When using PSK, the client must send to the server a valid identity and a valid pre shared key, in order for the SSL handshake to continue. Applications can provide this information in a slot connected to this signal, by filling in the passed `authenticator` object according to their needs.
        ///
        /// **Note:** Ignoring this signal, or failing to provide the required credentials, will cause the handshake to fail, and therefore the connection to be aborted.
        ///
        /// **Note:** The `authenticator` object is owned by the socket and must not be deleted by the application.
        #[cfg(feature = "ssl")]
        #[qsignal]
        #[rust_name = "pre_shared_key_authentication_required"]
        unsafe fn preSharedKeyAuthenticationRequired(
            self: Pin<&mut QNetworkAccessManager>,
            reply: *mut QNetworkReply,
            authenticator: *mut QSslPreSharedKeyAuthenticator,
        );

        /// This signal is emitted whenever a proxy requests authentication and `QNetworkAccessManager` cannot find a valid, cached credential. The slot connected to this signal should fill in the credentials for the proxy `proxy` in the `authenticator` object.
        ///
        /// `QNetworkAccessManager` will cache the credentials internally. The next time the proxy requests authentication, `QNetworkAccessManager` will automatically send the same credential without emitting this signal signal again.
        ///
        /// If the proxy rejects the credentials, `QNetworkAccessManager` will emit the signal again.
        #[qsignal]
        #[rust_name = "proxy_authentication_required"]
        unsafe fn proxyAuthenticationRequired(
            self: Pin<&mut QNetworkAccessManager>,
            proxy: &QNetworkProxy,
            authenticator: *mut QAuthenticator,
        );

        /// This signal is emitted if the SSL/TLS session encountered errors during the set up, including certificate verification errors. The errors parameter contains the list of errors.
        ///
        /// To indicate that the errors are not fatal and that the connection should proceed, [`QNetworkReply::ignore_ssl_errors`] should be called from the slot connected to this signal. If it is not called, the SSL session will be torn down before any data is exchanged (including the URL).
        ///
        /// This signal can be used to display an error message to the user indicating that security may be compromised and display the SSL settings (see [`QNetworkReply::ssl_configuration`] to obtain it). If the user decides to proceed after analyzing the remote certificate, the slot should call [`QNetworkReply::ignore_ssl_errors`].
        #[cfg(feature = "ssl")]
        #[qsignal]
        #[rust_name = "ssl_errors"]
        unsafe fn sslErrors(
            self: Pin<&mut QNetworkAccessManager>,
            reply: *mut QNetworkReply,
            errors: &QList_QSslError,
        );
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qnetworkaccessmanager_options"]
        fn qnetworkaccessmanagerOptions(
            manager: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
        ) -> *mut QNetworkReply;

        #[rust_name = "qnetworkaccessmanager_patch_data_raw"]
        fn qnetworkaccessmanagerPatch(
            manager: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
            data: &QByteArray,
        ) -> *mut QNetworkReply;

        #[rust_name = "qnetworkaccessmanager_patch_raw"]
        unsafe fn qnetworkaccessmanagerPatch(
            manager: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
            data: *mut QIODevice,
        ) -> *mut QNetworkReply;

        #[rust_name = "qnetworkaccessmanager_patch_http_raw"]
        unsafe fn qnetworkaccessmanagerPatch(
            manager: Pin<&mut QNetworkAccessManager>,
            request: &QNetworkRequest,
            multi_part: *mut QHttpMultiPart,
        ) -> *mut QNetworkReply;

        #[rust_name = "qnetworkaccessmanager_set_transfer_timeout_msecs"]
        fn qnetworkaccessmanagerSetTransferTimeoutMsecs(
            manager: Pin<&mut QNetworkAccessManager>,
            timeout: i64,
        );

        #[rust_name = "qnetworkaccessmanager_transfer_timeout_msecs"]
        fn qnetworkaccessmanagerTransferTimeoutMsecs(manager: &QNetworkAccessManager) -> i64;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkaccessmanager_init_default"]
        fn make_unique() -> UniquePtr<QNetworkAccessManager>;
    }
}

pub use ffi::{QNetworkAccessManager, QNetworkAccessManagerOperation};

impl fmt::Debug for QNetworkAccessManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QNetworkAccessManager {
    /// Constructs a `QNetworkAccessManager` object that is the center of the Network Access API.
    pub fn new() -> UniquePtr<Self> {
        let mut this = ffi::qnetworkaccessmanager_init_default();
        this.pin_mut().set_auto_delete_replies(true);
        this
    }

    /// If `enabled` is true, the internal HSTS cache will use a persistent store to read and write HSTS policies. The cache location is defined by [`QStandardPaths::CacheLocation`](crate::QStandardPaths::CacheLocation). If there is no writable `QStandardPaths::CacheLocation`, the store will be located in the program's working directory.
    ///
    /// **Note:** If HSTS cache already contains HSTS policies by the time persistent store is enabled, these policies will be preserved in the store. In case both cache and store contain the same known hosts, policies from cache are considered to be more up-to-date (and thus will overwrite the previous values in the store). If this behavior is undesired, enable HSTS store before enabling Strict Transport Security. By default, the persistent store of HSTS policies is disabled.
    pub fn enable_strict_transport_security_store(self: Pin<&mut Self>, enabled: bool) {
        self.enable_strict_transport_security_store_in(enabled, &QString::default());
    }

    /// Posts a request to obtain the contents of the target request and returns a new `QNetworkReply` object opened for reading which emits the [`QIODevice::ready_read`](crate::QIODevice::ready_read) signal whenever new data arrives.
    ///
    /// The contents as well as associated headers will be downloaded.
    pub fn get<'a>(
        self: Pin<&'a mut Self>,
        request: &QNetworkRequest,
    ) -> Pin<&'a mut QNetworkReply> {
        let reply = self.get_raw(request);
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Posts a request to obtain the network headers for request and returns a new `QNetworkReply` object which will contain such headers.
    ///
    /// The function is named after the HTTP request associated (`HEAD`).
    pub fn head<'a>(
        self: Pin<&'a mut Self>,
        request: &QNetworkRequest,
    ) -> Pin<&'a mut QNetworkReply> {
        let reply = self.head_raw(request);
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Sends a PATCH request to obtain the network headers for request and returns a new `QNetworkReply` object which will contain such headers.
    ///
    /// The function is named after the HTTP request associated (`OPTIONS`).
    pub fn options<'a>(
        self: Pin<&'a mut Self>,
        request: &QNetworkRequest,
    ) -> Pin<&'a mut QNetworkReply> {
        let reply = ffi::qnetworkaccessmanager_options(self, request);
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Sends an HTTP PATCH request to the destination specified by request and returns a new `QNetworkReply` object opened for reading that will contain the reply sent by the server. The contents of the `data` device will be uploaded to the server.
    ///
    /// `data` must be open for reading.
    ///
    /// **Note:** Sending a PATCH request on protocols other than HTTP and HTTPS is undefined and will probably fail.
    pub fn patch<'a, T>(
        self: Pin<&'a mut Self>,
        request: &QNetworkRequest,
        data: Pin<&'a mut T>,
    ) -> Pin<&'a mut QNetworkReply>
    where
        T: Upcast<QIODevice>,
    {
        // SAFETY: The lifetime parameter prevents concurrent modification of `data`.
        let reply = unsafe {
            ffi::qnetworkaccessmanager_patch_raw(self, request, upcast_mut(unpin_for_qt(data)))
        };
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Sends the contents of the `data` byte array to the destination specified by `request`.
    pub fn patch_data(
        self: Pin<&mut Self>,
        request: &QNetworkRequest,
        data: &QByteArray,
    ) -> Pin<&mut QNetworkReply> {
        let reply = ffi::qnetworkaccessmanager_patch_data_raw(self, request, data);
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Sends the contents of the `multi_part` message to the destination specified by request.
    ///
    /// This can be used for sending MIME multipart messages over HTTP.
    pub fn patch_http<'a>(
        self: Pin<&'a mut Self>,
        request: &QNetworkRequest,
        multi_part: Pin<&'a mut QHttpMultiPart>,
    ) -> Pin<&'a mut QNetworkReply> {
        // SAFETY: The lifetime parameter prevents concurrent modification of `multi_part`.
        let reply = unsafe {
            ffi::qnetworkaccessmanager_patch_http_raw(self, request, unpin_for_qt(multi_part))
        };
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }
    /// Sends an HTTP POST request to the destination specified by request and returns a new `QNetworkReply` object opened for reading that will contain the reply sent by the server. The contents of the `data` device will be uploaded to the server.
    ///
    /// `data` must be open for reading.
    ///
    /// **Note:** Sending a POST request on protocols other than HTTP and HTTPS is undefined and will probably fail.
    pub fn post<'a, T>(
        self: Pin<&'a mut Self>,
        request: &QNetworkRequest,
        data: Pin<&'a mut T>,
    ) -> Pin<&'a mut QNetworkReply>
    where
        T: Upcast<QIODevice>,
    {
        // SAFETY: The lifetime parameter prevents concurrent modification of `data`.
        let reply = unsafe { self.post_device_raw(request, upcast_mut(unpin_for_qt(data))) };
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Sends the contents of the `data` byte array to the destination specified by `request`.
    pub fn post_data(
        self: Pin<&mut Self>,
        request: &QNetworkRequest,
        data: &QByteArray,
    ) -> Pin<&mut QNetworkReply> {
        let reply = self.post_data_raw(request, data);
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Sends the contents of the `multi_part` message to the destination specified by request.
    ///
    /// This can be used for sending MIME multipart messages over HTTP.
    pub fn post_http<'a>(
        self: Pin<&'a mut Self>,
        request: &QNetworkRequest,
        multi_part: Pin<&'a mut QHttpMultiPart>,
    ) -> Pin<&'a mut QNetworkReply> {
        // SAFETY: The lifetime parameter prevents concurrent modification of `multi_part`.
        let reply = unsafe { self.post_http_raw(request, unpin_for_qt(multi_part)) };
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Uploads the contents of data to the destination request and returns a new `QNetworkReply` object that will be open for reply.
    ///
    /// `data` must be opened for reading.
    ///
    /// Whether anything will be available for reading from the returned object is protocol dependent. For HTTP, the server may send a small HTML page indicating the upload was successful (or not). Other protocols will probably have content in their replies.
    ///
    /// Note: For HTTP, this request will send a PUT request, which most servers do not allow. Form upload mechanisms, including that of uploading files through HTML forms, use the POST mechanism.
    pub fn put<'a, T>(
        self: Pin<&'a mut Self>,
        request: &QNetworkRequest,
        data: Pin<&'a mut T>,
    ) -> Pin<&'a mut QNetworkReply>
    where
        T: Upcast<QIODevice>,
    {
        // SAFETY: The lifetime parameter prevents concurrent modification of `data`.
        let reply = unsafe { self.put_device_raw(request, upcast_mut(unpin_for_qt(data))) };
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Sends the contents of the `data` byte array to the destination specified by `request`.
    pub fn put_data(
        self: Pin<&mut Self>,
        request: &QNetworkRequest,
        data: &QByteArray,
    ) -> Pin<&mut QNetworkReply> {
        let reply = self.put_data_raw(request, data);
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Sends the contents of the `multi_part` message to the destination specified by request.
    ///
    /// This can be used for sending MIME multipart messages over HTTP.
    pub fn put_http<'a>(
        self: Pin<&'a mut Self>,
        request: &QNetworkRequest,
        multi_part: Pin<&'a mut QHttpMultiPart>,
    ) -> Pin<&'a mut QNetworkReply> {
        // SAFETY: The lifetime parameter prevents concurrent modification of `multi_part`.
        let reply = unsafe { self.put_http_raw(request, unpin_for_qt(multi_part)) };
        // SAFETY: This object is the parent of the reply, which is valid and non-null.
        unsafe { Pin::new_unchecked(&mut *reply) }
    }

    /// Sets the manager's network cache to be the `cache` specified. The cache is used for all requests dispatched by the manager. **The manager takes ownership of `cache`.**
    ///
    /// Use this function to set the network cache object to a class that implements additional features, like saving the cookies to permanent storage.
    pub fn set_cache<T>(self: Pin<&mut Self>, cache: UniquePtr<T>)
    where
        T: Upcast<QAbstractNetworkCache> + UniquePtrTarget,
    {
        let cache = upcast_mut(cache.into_raw());
        // SAFETY: QNetworkAccessManager takes ownership of the cache.
        unsafe { self.set_cache_raw(cache) }
    }

    /// Sets the manager's cookie jar to be the `cookie_jar` specified. The cookie jar is used by all requests dispatched by the manager. **The manager takes ownership of `cookie_jar`.**
    ///
    /// Use this function to set the cookie jar object to a class that implements additional features, like saving the cookies to permanent storage.
    ///
    /// # Panics
    ///
    /// Panics if `cookie_jar` does not live in the same thread as this manager.
    pub fn set_cookie_jar<T>(self: Pin<&mut Self>, cookie_jar: UniquePtr<T>)
    where
        T: Upcast<QNetworkCookieJar> + UniquePtrTarget,
    {
        // Reduce monomorphization
        fn inner(this: Pin<&mut QNetworkAccessManager>, cookie_jar: *mut QNetworkCookieJar) {
            assert!(
                // SAFETY: cookie_jar is valid.
                cookie_jar.is_null() || in_same_thread(&*this, unsafe { &*cookie_jar }),
                "set_cookie_jar: cookie_jar must be in the same thread as self"
            );
            // SAFETY: QNetworkAccessManager takes ownership of the cookie jar.
            unsafe { this.set_cookie_jar_raw(cookie_jar) };
        }
        inner(self, upcast_mut(cookie_jar.into_raw()));
    }

    /// Sets `timeout` as the transfer timeout.
    ///
    /// Transfers are aborted if no bytes are transferred before the timeout expires. `None` means no timer is set. If this function is not called, the timeout is disabled.
    pub fn set_transfer_timeout(self: Pin<&mut Self>, duration: Option<Duration>) {
        let msecs = match duration {
            Some(duration) => duration.as_millis().try_into().unwrap_or(i64::MAX),
            None => 0,
        };
        ffi::qnetworkaccessmanager_set_transfer_timeout_msecs(self, msecs);
    }

    /// Returns the timeout used for transfers.
    pub fn transfer_timeout(&self) -> Option<Duration> {
        let transfer_timeout = ffi::qnetworkaccessmanager_transfer_timeout_msecs(self);
        let msecs = u64::try_from(transfer_timeout).ok()?;
        if msecs == 0 {
            None
        } else {
            Some(Duration::from_millis(msecs))
        }
    }
}

impl fmt::Display for QNetworkAccessManagerOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Self::HeadOperation => "HEAD",
            Self::GetOperation => "GET",
            Self::PutOperation => "PUT",
            Self::PostOperation => "POST",
            Self::DeleteOperation => "DELETE",
            Self::CustomOperation => "Custom",
            _ => "unknown",
        })
    }
}

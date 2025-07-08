use std::fmt;
use std::io::{self, Read};
use std::ops::Deref;
use std::pin::Pin;

use cxx_qt::casting::Upcast;
use cxx_qt::QObject;
use cxx_qt_lib::QVariant;
#[cfg(cxxqt_qt_version_at_least_6_7)]
use cxx_qt_lib::{QAnyStringView, QByteArray};

use crate::qobject::debug_qobject;
use crate::util::IsNonNull;
#[cfg(feature = "ssl")]
use crate::QSslConfiguration;
use crate::{QIODevice, QNetworkRequestAttribute, QNetworkRequestKnownHeaders};

#[cxx_qt::bridge]
mod ffi {
    /// Indicates all possible error conditions found during the processing of the request.
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum QNetworkReplyNetworkError {
        NoError = 0,

        // network layer errors [relating to the destination server] (1-99):
        /// The remote server refused the connection (the server is not accepting requests).
        ConnectionRefusedError = 1,
        /// The remote server closed the connection prematurely, before the entire reply was received and processed.
        RemoteHostClosedError,
        /// The remote host name was not found (invalid hostname).
        HostNotFoundError,
        /// The connection to the remote server timed out.
        TimeoutError,
        /// The operation was canceled via calls to [`QNetworkReply::abort`] or [`QNetworkReply::close`](QIODevice::close) before it was finished.
        OperationCanceledError,
        /// The SSL/TLS handshake failed and the encrypted channel could not be established. The [`QNetworkReply::ssl_errors`] signal should have been emitted.
        SslHandshakeFailedError,
        /// The connection was broken due to disconnection from the network, however the system has initiated roaming to another access point. The request should be resubmitted and will be processed as soon as the connection is re-established.
        TemporaryNetworkFailureError,
        /// The connection was broken due to disconnection from the network or failure to start the network.
        NetworkSessionFailedError,
        /// The background request is not currently allowed due to platform policy.
        BackgroundRequestNotAllowedError,
        /// while following redirects, the maximum limit was reached. The limit is by default set to 50 or as set by [`QNetworkRequest::set_max_redirects_allowed`].
        TooManyRedirectsError,
        /// While following redirects, the network access API detected a redirect from a encrypted protocol (https) to an unencrypted one (http).
        InsecureRedirectError,
        /// An unknown network-related error was detected.
        UnknownNetworkError = 99,

        // proxy errors (101-199):
        /// The connection to the proxy server was refused (the proxy server is not accepting requests).
        ProxyConnectionRefusedError = 101,
        /// The proxy server closed the connection prematurely, before the entire reply was received and processed.
        ProxyConnectionClosedError,
        /// The proxy host name was not found (invalid proxy hostname).
        ProxyNotFoundError,
        /// The connection to the proxy timed out or the proxy did not reply in time to the request sent.
        ProxyTimeoutError,
        /// The proxy requires authentication in order to honour the request but did not accept any credentials offered (if any).
        ProxyAuthenticationRequiredError,
        /// An unknown proxy-related error was detected.
        UnknownProxyError = 199,

        // content errors (201-299):
        /// The access to the remote content was denied (similar to HTTP error 403).
        ContentAccessDenied = 201,
        /// The operation requested on the remote content is not permitted.
        ContentOperationNotPermittedError,
        /// The remote content was not found at the server (similar to HTTP error 404).
        ContentNotFoundError,
        /// The remote server requires authentication to serve the content but the credentials provided were not accepted (if any).
        AuthenticationRequiredError,
        /// The request needed to be sent again, but this failed for example because the upload data could not be read a second time.
        ContentReSendError,
        /// The request could not be completed due to a conflict with the current state of the resource.
        ContentConflictError,
        /// The requested resource is no longer available at the server.
        ContentGoneError,
        /// An unknown error related to the remote content was detected.
        UnknownContentError = 299,

        // protocol errors
        /// The Network Access API cannot honor the request because the protocol is not known.
        ProtocolUnknownError = 301,
        /// The requested operation is invalid for this protocol.
        ProtocolInvalidOperationError,
        /// A breakdown in protocol was detected (parsing error, invalid or unexpected responses, etc.).
        ProtocolFailure = 399,

        // Server side errors (401-499)
        /// The server encountered an unexpected condition which prevented it from fulfilling the request.
        InternalServerError = 401,
        /// The server does not support the functionality required to fulfill the request.
        OperationNotImplementedError,
        /// The server is unable to handle the request at this time.
        ServiceUnavailableError,
        /// An unknown error related to the server response was detected.
        UnknownServerError = 499,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qtypes.h");
        type qint64 = cxx_qt_lib::qint64;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
        include!("cxx-qt-lib/qlist.h");
        type QList_QByteArray = cxx_qt_lib::QList<QByteArray>;

        include!("cxx-qt-io/qnetworkaccessmanager.h");
        type QNetworkAccessManager = crate::QNetworkAccessManager;
        type QNetworkAccessManagerOperation = crate::QNetworkAccessManagerOperation;
        include!("cxx-qt-io/qnetworkrequest.h");
        type QNetworkRequest = crate::QNetworkRequest;
        type QNetworkRequestAttribute = crate::QNetworkRequestAttribute;
        type QNetworkRequestKnownHeaders = crate::QNetworkRequestKnownHeaders;
        include!("cxx-qt-io/qpair.h");
        type QPair_QByteArray_QByteArray =
            crate::QPair<cxx_qt_lib::QByteArray, cxx_qt_lib::QByteArray>;
        include!("cxx-qt-io/qlist.h");
        type QList_QPair_QByteArray_QByteArray = cxx_qt_lib::QList<QPair_QByteArray_QByteArray>;
    }

    #[cfg(cxxqt_qt_version_at_least_6_7)]
    extern "C++" {
        include!("cxx-qt-lib/qanystringview.h");
        type QAnyStringView<'a> = cxx_qt_lib::QAnyStringView<'a>;
    }

    #[cfg(cxxqt_qt_version_at_least_6_8)]
    extern "C++" {
        include!("cxx-qt-io/qhttpheaders.h");
        type QHttpHeaders = crate::QHttpHeaders;
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
        include!("cxx-qt-io/qnetworkreply.h");
        type QNetworkReplyNetworkError;
        type QIODevice = crate::QIODevice;
    }

    unsafe extern "C++Qt" {
        /// The `QNetworkReply` class contains the data and headers for a request sent with [`QNetworkAccessManager`].
        ///
        /// Qt Documentation: [QNetworkReply](https://doc.qt.io/qt-6/qnetworkreply.html#details)
        #[qobject]
        #[base = QIODevice]
        type QNetworkReply;

        /// Aborts the operation immediately and close down any network connections still open. Uploads still in progress are also aborted.
        ///
        /// The [finished](QNetworkReply::finished) signal will also be emitted.
        fn abort(self: Pin<&mut QNetworkReply>);

        #[doc(hidden)]
        #[rust_name = "attribute_or_invalid"]
        fn attribute(self: &QNetworkReply, code: QNetworkRequestAttribute) -> QVariant;

        /// Returns the error that was found during the processing of this request. If no error was found, returns [`QNetworkReplyNetworkError::NoError`].
        fn error(self: &QNetworkReply) -> QNetworkReplyNetworkError;

        #[doc(hidden)]
        #[cfg(cxxqt_qt_version_at_least_6_7)]
        #[rust_name = "has_raw_header_view"]
        fn hasRawHeader(self: &QNetworkReply, header_name: QAnyStringView) -> bool;

        /// Returns `true` if the raw header of name `header_name` was sent by the remote server.
        #[cfg(not(cxxqt_qt_version_at_least_6_7))]
        #[rust_name = "has_raw_header"]
        fn hasRawHeader(self: &QNetworkReply, header_name: &QByteArray) -> bool;

        #[doc(hidden)]
        #[rust_name = "header_or_invalid"]
        fn header(self: &QNetworkReply, header: QNetworkRequestKnownHeaders) -> QVariant;

        #[cfg(cxxqt_qt_version_at_least_6_8)]
        fn headers(self: &QNetworkReply) -> QHttpHeaders;

        /// If this function is called, SSL errors related to network connection will be ignored, including certificate validation errors.
        ///
        /// **Warning:** Be sure to always let the user inspect the errors reported by the [`ssl_errors`](QNetworkReply::ssl_errors) signal, and only call this method upon confirmation from the user that proceeding is ok. If there are unexpected errors, the reply should be aborted. Calling this method without inspecting the actual errors will most likely pose a security risk for your application. Use it with great care!
        ///
        /// This function can be called from the slot connected to the [`ssl_errors`](QNetworkReply::ssl_errors) signal, which indicates which errors were found.
        ///
        /// **Note:** If HTTP Strict Transport Security is enabled for [`QNetworkAccessManager`], this function has no effect.
        #[cfg(feature = "ssl")]
        #[rust_name = "ignore_all_ssl_errors"]
        fn ignoreSslErrors(self: Pin<&mut QNetworkReply>);

        /// If this function is called, the SSL errors given in `errors` will be ignored.
        ///
        /// Multiple calls to this function will replace the list of errors that were passed in previous calls. You can clear the list of errors you want to ignore by calling this function with an empty list.
        ///
        /// **Note:** If HTTP Strict Transport Security is enabled for [`QNetworkAccessManager`], this function has no effect.
        #[cfg(feature = "ssl")]
        #[rust_name = "ignore_ssl_errors"]
        fn ignoreSslErrors(self: Pin<&mut QNetworkReply>, errors: &QList_QSslError);

        /// Returns `true` when the reply has finished or was aborted.
        #[rust_name = "is_finished"]
        fn isFinished(self: &QNetworkReply) -> bool;

        /// Returns `true` when the request is still processing and the reply has not finished or was aborted yet.
        #[rust_name = "is_running"]
        fn isRunning(self: &QNetworkReply) -> bool;

        /// Returns the `QNetworkAccessManager` that was used to create this `QNetworkReply` object.
        unsafe fn manager(self: &QNetworkReply) -> *mut QNetworkAccessManager;

        /// Returns the operation that was posted for this reply.
        fn operation(self: &QNetworkReply) -> QNetworkAccessManagerOperation;

        #[doc(hidden)]
        #[cfg(cxxqt_qt_version_at_least_6_7)]
        #[rust_name = "raw_header_view"]
        fn rawHeader(self: &QNetworkReply, header_name: QAnyStringView) -> QByteArray;

        /// Returns the raw contents of the header `header_name` as sent by the remote server. If there is no such header, returns an empty byte array, which may be indistinguishable from an empty header. Use [`has_raw_header`](QNetworkReply::has_raw_header) to verify if the server sent such header field.
        #[cfg(not(cxxqt_qt_version_at_least_6_7))]
        #[rust_name = "raw_header"]
        fn rawHeader(self: &QNetworkReply, header_name: &QByteArray) -> QByteArray;

        /// Returns a list of headers fields that were sent by the remote server, in the order that they were sent. Duplicate headers are merged together and take place of the latter duplicate.
        #[rust_name = "raw_header_list"]
        fn rawHeaderList(self: &QNetworkReply) -> QList_QByteArray;

        /// Returns a list of raw header pairs.
        #[rust_name = "raw_header_pairs"]
        fn rawHeaderPairs(self: &QNetworkReply) -> &QList_QPair_QByteArray_QByteArray;

        #[doc(hidden)]
        #[rust_name = "read_buffer_size_qint64"]
        fn readBufferSize(self: &QNetworkReply) -> qint64;

        /// Returns the request that was posted for this reply. In special, note that the URL for the request may be different than that of the reply.
        fn request(self: &QNetworkReply) -> QNetworkRequest;

        #[doc(hidden)]
        #[rust_name = "set_read_buffer_size_qint64"]
        fn setReadBufferSize(self: Pin<&mut QNetworkReply>, size: qint64);

        /// Sets the SSL configuration for the network connection associated with this request, if possible, to be that of `config`.
        #[cfg(feature = "ssl")]
        #[rust_name = "set_ssl_configuration"]
        fn setSslConfiguration(self: Pin<&mut QNetworkReply>, config: &QSslConfiguration);

        #[doc(hidden)]
        #[cfg(feature = "ssl")]
        #[rust_name = "ssl_configuration_or_invalid"]
        fn sslConfiguration(self: &QNetworkReply) -> QSslConfiguration;

        /// Returns the URL of the content downloaded or uploaded. Note that the URL may be different from that of the original request. If redirections were enabled in the request, then this function returns the current url that the network API is accessing, i.e the url of the resource the request got redirected to.
        fn url(self: &QNetworkReply) -> QUrl;

        #[doc(hidden)]
        #[qsignal]
        #[rust_name = "download_progress_qint64"]
        fn downloadProgress(
            self: Pin<&mut QNetworkReply>,
            bytes_received: qint64,
            bytes_total: qint64,
        );

        /// This signal is emitted when an SSL/TLS session has successfully completed the initial handshake. At this point, no user data has been transmitted. The signal can be used to perform additional checks on the certificate chain, for example to notify users when the certificate for a website has changed. If the reply does not match the expected criteria then it should be aborted by calling [`abort`](QNetworkReply::abort) by a slot connected to this signal. The SSL configuration in use can be inspected using [`ssl_configuration`](QNetworkReply::ssl_configuration).
        ///
        /// Internally, [`QNetworkAccessManager`](crate::QNetworkAccessManager) may open multiple connections to a server, in order to allow it process requests in parallel. These connections may be reused, which means that this signal would not be emitted. This means that you are only guaranteed to receive this signal for the first connection to a site in the lifespan of the [`QNetworkAccessManager`](crate::QNetworkAccessManager).
        #[cfg(feature = "ssl")]
        #[qsignal]
        fn encrypted(self: Pin<&mut QNetworkReply>);

        /// This signal is emitted when the reply detects an error in processing. The [`finished`](QNetworkReply::finished) signal will probably follow, indicating that the connection is over.
        ///
        /// The code parameter contains the code of the error that was detected. Call [`error_string`](QIODevice::error_string) to obtain a textual representation of the error condition.
        ///
        /// **Note:** Do not delete the object in the slot connected to this signal.
        #[qsignal]
        #[rust_name = "error_occurred"]
        fn errorOccurred(self: Pin<&mut QNetworkReply>, code: QNetworkReplyNetworkError);

        /// This signal is emitted when the reply has finished processing. After this signal is emitted, there will be no more updates to the reply's data or metadata.
        ///
        /// Unless [`close`](QIODevice::close) or [`abort`](QNetworkReply::abort) have been called, the reply will still be opened for reading, so the data can be retrieved by calls to [`read`](QIODevice::read) or [`read_all`](QIODevice::read_all). In particular, if no calls to [`read`](QIODevice::read) were made as a result of [`ready_read`](QIODevice::ready_read), a call to [`read_all`](QIODevice::read_all) will retrieve the full contents in a `QByteArray`.
        ///
        /// This signal is emitted in tandem with [`QNetworkAccessManager::finished`](crate::QNetworkAccessManager::finished) where that signal's reply parameter is this object.
        ///
        /// **Note:** Do not delete the object in the slot connected to this signal.
        ///
        /// You can also use [`is_finished`](QNetworkReply::is_finished) to check if a `QNetworkReply` has finished even before you receive this signal.
        #[qsignal]
        fn finished(self: Pin<&mut QNetworkReply>);

        /// This signal is emitted whenever the metadata in this reply changes. Metadata is any information that is not the content (data) itself, including the network headers. In the majority of cases, the metadata will be known fully by the time the first byte of data is received. However, it is possible to receive updates of headers or other metadata during the processing of the data.
        #[qsignal]
        #[rust_name = "meta_data_changed"]
        fn metaDataChanged(self: Pin<&mut QNetworkReply>);

        /// This signal is emitted if the SSL/TLS handshake negotiates a PSK ciphersuite, and therefore a PSK authentication is then required.
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
            self: Pin<&mut QNetworkReply>,
            authenticator: *mut QSslPreSharedKeyAuthenticator,
        );

        /// When client code handling the [`redirected`](QNetworkReply::redirected) signal has verified the new URL, it emits this signal to allow the redirect to go ahead. This protocol applies to network requests whose redirects policy is set to [`QNetworkRequestRedirectPolicy::UserVerifiedRedirectPolicy`](crate::QNetworkRequestRedirectPolicy::UserVerifiedRedirectPolicy).
        #[rust_name = "redirect_allowed"]
        fn redirectAllowed(self: Pin<&mut QNetworkReply>);

        /// This signal is emitted if the [`QNetworkRequestRedirectPolicy::ManualRedirectPolicy`](crate::QNetworkRequestRedirectPolicy::ManualRedirectPolicy) was not set in the request and the server responded with a 3xx status (specifically 301, 302, 303, 305, 307 or 308 status code) with a valid url in the location header, indicating a HTTP redirect. The `url` parameter contains the new redirect url as returned by the server in the location header.
        fn redirected(self: Pin<&mut QNetworkReply>, url: &QUrl);

        /// This signal is emitted 1 or more times when the request was sent. Useful for custom progress or timeout handling.
        ///
        /// Introduced in Qt 6.3.
        #[cfg(cxxqt_qt_version_at_least_6_3)]
        #[qsignal]
        #[rust_name = "request_sent"]
        fn requestSent(self: Pin<&mut QNetworkReply>);

        /// This signal is emitted 0 or more times, when the socket is connecting, before sending the request. Useful for custom progress or timeout handling.
        ///
        /// Introduced in Qt 6.3.
        #[cfg(cxxqt_qt_version_at_least_6_3)]
        #[qsignal]
        #[rust_name = "socket_started_connecting"]
        fn socketStartedConnecting(self: Pin<&mut QNetworkReply>);

        /// This signal is emitted if the SSL/TLS session encountered errors during the set up, including certificate verification errors. The errors parameter contains the list of errors.
        ///
        /// To indicate that the errors are not fatal and that the connection should proceed, [`ignore_ssl_errors`](QNetworkReply::ignore_ssl_errors) should be called from the slot connected to this signal. If it is not called, the SSL session will be torn down before any data is exchanged (including the URL).
        ///
        /// This signal can be used to display an error message to the user indicating that security may be compromised and display the SSL settings (see [`ssl_configuration`](QNetworkReply::ssl_configuration) to obtain it). If the user decides to proceed after analyzing the remote certificate, the slot should call [`ignore_ssl_errors`](QNetworkReply::ignore_ssl_errors).
        #[cfg(feature = "ssl")]
        #[qsignal]
        #[rust_name = "ssl_errors"]
        fn sslErrors(self: Pin<&mut QNetworkReply>, errors: &QList_QSslError);

        #[doc(hidden)]
        #[qsignal]
        #[rust_name = "upload_progress_qint64"]
        fn uploadProgress(self: Pin<&mut QNetworkReply>, bytes_sent: qint64, bytes_total: qint64);
    }

    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/casting.h");

        #[rust_name = "upcast_qnetworkreply_qobject"]
        unsafe fn upcastPtr(reply: *const QNetworkReply) -> *const QObject;
        #[rust_name = "downcast_qobject_qnetworkreply"]
        unsafe fn downcastPtr(reply: *const QObject) -> *const QNetworkReply;
    }

    impl UniquePtr<QNetworkReply> {}
}

pub use ffi::{QNetworkReply, QNetworkReplyNetworkError};

impl fmt::Debug for QNetworkReply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QNetworkReply {
    /// Returns the attribute associated with the code code. If the attribute has not been set, it returns `None`.
    ///
    /// You can expect the default values listed in [`QNetworkRequestAttribute`] to be applied to the values returned by this function.
    pub fn attribute(&self, code: QNetworkRequestAttribute) -> Option<QVariant> {
        self.attribute_or_invalid(code).nonnull()
    }

    /// Returns `true` if the raw header of name `header_name` was sent by the remote server.
    ///
    /// **Note:** In Qt versions before 6.7, `header_name` must be [`&QByteArray`](cxx_qt_lib::QByteArray).
    #[cfg(cxxqt_qt_version_at_least_6_7)]
    pub fn has_raw_header<'a, T>(&self, header_name: T) -> bool
    where
        T: Into<QAnyStringView<'a>>,
    {
        self.has_raw_header_view(header_name.into())
    }

    /// Returns the value of the known header `header`, if that header was sent by the remote server. If the header was not sent, returns `None`.
    pub fn header(&self, header: QNetworkRequestKnownHeaders) -> Option<QVariant> {
        self.header_or_invalid(header).nonnull()
    }

    /// Returns the raw contents of the header `header_name` as sent by the remote server. If there is no such header, returns an empty byte array, which may be indistinguishable from an empty header. Use [`has_raw_header`](QNetworkReply::has_raw_header) to verify if the server sent such header field.
    ///
    /// **Note:** In Qt versions before 6.7, `header_name` must be [`&QByteArray`](cxx_qt_lib::QByteArray).
    #[cfg(cxxqt_qt_version_at_least_6_7)]
    pub fn raw_header<'a, T>(&self, header_name: T) -> QByteArray
    where
        T: Into<QAnyStringView<'a>>,
    {
        self.raw_header_view(header_name.into())
    }

    /// Returns the size of the read buffer, in bytes.
    pub fn read_buffer_size(&self) -> i64 {
        self.read_buffer_size_qint64().into()
    }

    /// Sets the size of the read buffer to be `size` bytes. The read buffer is the buffer that holds data that is being downloaded off the network, before it is read with [`read`](QIODevice::read). Setting the buffer size to 0 will make the buffer unlimited in size.
    ///
    /// `QNetworkReply` will try to stop reading from the network once this buffer is full (i.e., [`self.bytes_available()`](QIODevice::bytes_available) returns `size` or more), thus causing the download to throttle down as well. If the buffer is not limited in size, `QNetworkReply` will try to download as fast as possible from the network.
    ///
    /// Unlike [`QAbstractSocket::set_read_buffer_size`](crate::QAbstractSocket::set_read_buffer_size), `QNetworkReply` cannot guarantee precision in the read buffer size. That is, [`self.bytes_available()`](QIODevice::bytes_available) can return more than `size`.
    pub fn set_read_buffer_size(self: Pin<&mut Self>, size: i64) {
        self.set_read_buffer_size_qint64(size.into());
    }

    /// Returns the SSL configuration and state associated with this reply, if SSL was used. It will contain the remote server's certificate, its certificate chain leading to the Certificate Authority as well as the encryption ciphers in use.
    ///
    /// The peer's certificate and its certificate chain will be known by the time [`ssl_errors`](QNetworkReply::ssl_errors) is emitted, if it's emitted.
    #[cfg(feature = "ssl")]
    pub fn ssl_configuration(&self) -> Option<QSslConfiguration> {
        self.ssl_configuration_or_invalid().nonnull()
    }

    wrap_qsignal! {
        /// This signal is emitted to indicate the progress of the download part of this network request, if there's any. If there's no download associated with this request, this signal will be emitted once with 0 as the value of both `bytes_received` and `bytes_total`.
        ///
        /// The `bytes_received` parameter indicates the number of bytes received, while `bytes_total` indicates the total number of bytes expected to be downloaded. If the number of bytes to be downloaded is not known, `bytes_total` will be -1.
        ///
        /// The download is finished when `bytes_received` is equal to `bytes_total`. At that time, `bytes_total` will not be -1.
        ///
        /// Note that the values of both `bytes_received` and `bytes_total` may be different from [`self.size()`](QIODevice::size), the total number of bytes obtained through [`read`](QIODevice::read) or [`read_all`](QIODevice::read_all), or the value of the ContentLengthHeader header. The reason for that is that there may be protocol overhead or the data may be compressed during the download.
        download_progress(bytes_received: i64, bytes_total: i64)(download_progress_qint64);
        connect_download_progress(connect_download_progress_qint64);
        on_download_progress(on_download_progress_qint64);
        "downloadProgress"
    }

    wrap_qsignal! {
        /// This signal is emitted to indicate the progress of the upload part of this network request, if there's any. If there's no upload associated with this request, this signal will not be emitted.
        ///
        /// The `bytes_sent` parameter indicates the number of bytes uploaded, while `bytes_total` indicates the total number of bytes to be uploaded. If the number of bytes to be uploaded could not be determined, `bytes_total` will be -1.
        ///
        /// The upload is finished when `bytes_sent` is equal to `bytes_total`. At that time, `bytes_total` will not be -1.
        upload_progress(bytes_sent: i64, bytes_total: i64)(upload_progress_qint64);
        connect_upload_progress(connect_upload_progress_qint64);
        on_upload_progress(on_upload_progress_qint64);
        "uploadProgress"
    }

    /// Casts this object to `QIODevice`.
    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    /// Mutably casts this object to `QIODevice`.
    pub fn as_io_device_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QIODevice> {
        self.as_mut().upcast_pin()
    }
}

impl Deref for QNetworkReply {
    type Target = QIODevice;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

unsafe impl Upcast<QObject> for QNetworkReply {
    unsafe fn upcast_ptr(this: *const Self) -> *const QObject {
        ffi::upcast_qnetworkreply_qobject(this)
    }

    unsafe fn from_base_ptr(base: *const QObject) -> *const Self {
        ffi::downcast_qobject_qnetworkreply(base)
    }
}

impl Read for Pin<&mut QNetworkReply> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_io_device_mut().read(buf)
    }
}

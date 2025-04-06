#[cxx_qt::bridge]
mod ffi {
    /// Indicates all possible error conditions found during the processing of the request.
    #[repr(i32)]
    #[derive(Debug)]
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
        /// The operation was canceled via calls to `abort()` or `close()` before it was finished.
        OperationCanceledError,
        /// The SSL/TLS handshake failed and the encrypted channel could not be established. The `ssl_errors()` signal should have been emitted.
        SslHandshakeFailedError,
        /// The connection was broken due to disconnection from the network, however the system has initiated roaming to another access point. The request should be resubmitted and will be processed as soon as the connection is re-established.
        TemporaryNetworkFailureError,
        /// The connection was broken due to disconnection from the network or failure to start the network.
        NetworkSessionFailedError,
        /// The background request is not currently allowed due to platform policy.
        BackgroundRequestNotAllowedError,
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
        include!("cxx-qt-io/qnetworkreply.h");
        type QNetworkReplyNetworkError;
    }
}

pub use ffi::QNetworkReplyNetworkError;

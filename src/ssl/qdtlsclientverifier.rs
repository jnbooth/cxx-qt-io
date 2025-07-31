use std::fmt;
use std::ops::Deref;
use std::pin::Pin;

use cxx::UniquePtr;
use cxx_qt::QObject;
use cxx_qt::casting::Upcast;
use cxx_qt_lib::QByteArray;

use crate::qobject::debug_qobject;
use crate::util::{IsNonNull, unpin_for_qt};
use crate::{QHostAddress, QUdpSocket};

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddress = crate::QHostAddress;
        include!("cxx-qt-io/qudpsocket.h");
        type QUdpSocket = crate::QUdpSocket;
    }

    extern "C++" {
        include!("cxx-qt-io/qdtlsclientverifier.h");
        type QDtlsGeneratorParameters = crate::QDtlsGeneratorParameters;
        type QDtlsError = crate::QDtlsError;
    }

    unsafe extern "C++Qt" {
        /// This class implements server-side DTLS cookie generation and verification.
        ///
        /// Qt Documentation: [QDtlsClientVerifier](https://doc.qt.io/qt-6/qdtlsclientverifier.html#details)
        #[qobject]
        #[base = QObject]
        type QDtlsClientVerifier;

        /// Returns the current secret and hash algorithm used to generate cookies.
        #[rust_name = "cookie_generator_parameters"]
        fn cookieGeneratorParameters(self: &QDtlsClientVerifier) -> QDtlsGeneratorParameters;

        /// Returns the last error encountered by the connection or [`QDtlsError::NoError`].
        #[rust_name = "dtls_error"]
        fn dtlsError(self: &QDtlsClientVerifier) -> QDtlsError;

        /// Returns a textual description for the last error encountered by the connection or empty string.
        #[rust_name = "dtls_error_string"]
        fn dtlsErrorString(self: &QDtlsClientVerifier) -> QString;

        /// Sets the cryptographic hash algorithm and the secret from `params`. This `QDtlsClientVerifier` will use these to generate cookies. If the new secret has size zero, this function returns `false` and does not change the cookie generator parameters.
        ///
        /// **Note:** This function must be called before the handshake starts.
        #[rust_name = "set_cookie_generator_parameters"]
        fn setCookieGeneratorParameters(
            self: Pin<&mut QDtlsClientVerifier>,
            params: &QDtlsGeneratorParameters,
        ) -> bool;

        #[doc(hidden)]
        #[rust_name = "verified_hello_or_empty"]
        fn verifiedHello(self: &QDtlsClientVerifier) -> QByteArray;

        /// # Safety
        ///
        /// `socket` must be valid.
        #[doc(hidden)]
        #[rust_name = "verify_client_raw"]
        unsafe fn verifyClient(
            self: Pin<&mut QDtlsClientVerifier>,
            socket: *mut QUdpSocket,
            dgram: &QByteArray,
            address: &QHostAddress,
            port: u16,
        ) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qdtlsclientverifier_init_default"]
        fn make_unique() -> UniquePtr<QDtlsClientVerifier>;
    }
}

pub use ffi::QDtlsClientVerifier;

impl fmt::Debug for QDtlsClientVerifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QDtlsClientVerifier {
    /// Constructs a `QDtlsClientVerifier` object.
    pub fn new() -> UniquePtr<Self> {
        ffi::qdtlsclientverifier_init_default()
    }

    /// Convenience function. Returns the last `ClientHello` message that was successfully verified, or `None` if no verification has completed.
    pub fn verified_hello(&self) -> Option<QByteArray> {
        self.verified_hello_or_empty().nonnull()
    }

    /// `dgram` must be a non-empty datagram, `address` cannot be null, broadcast, or multicast. `port` is the remote peer's port. This function returns `true` if dgram contains a `ClientHello` message with a valid cookie. If no matching cookie is found, this function will send a `HelloVerifyRequest` message using socket and return `false`.
    pub fn verify_client(
        self: Pin<&mut Self>,
        socket: Pin<&mut QUdpSocket>,
        dgram: &QByteArray,
        address: &QHostAddress,
        port: u16,
    ) -> bool {
        // SAFETY: `unpin_for_qt(socket)` is passed directly to Qt.
        unsafe { self.verify_client_raw(unpin_for_qt(socket), dgram, address, port) }
    }
}

impl Deref for QDtlsClientVerifier {
    type Target = QObject;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

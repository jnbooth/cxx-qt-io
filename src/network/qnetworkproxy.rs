use std::fmt::{self, Debug, Formatter};
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};
use cxx_qt_lib::{QFlag, QFlags};

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum ProxyCapability {
        TunnelingCapability = 0x0001,
        ListeningCapability = 0x0002,
        UdpTunnelingCapability = 0x0004,
        CachingCapability = 0x0008,
        HostNameLookupCapability = 0x0010,
        SctpTunnelingCapability = 0x00020,
        SctpListeningCapability = 0x00040,
    }

    #[repr(i32)]
    #[derive(Debug)]
    enum ProxyType {
        DefaultProxy,
        Socks5Proxy,
        NoProxy,
        HttpProxy,
        HttpCachingProxy,
        FtpCachingProxy,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qlist.h");
        type QList_QByteArray = cxx_qt_lib::QList<QByteArray>;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
        include!("cxx-qt-io/qnetworkrequest.h");
        type KnownHeaders = crate::KnownHeaders;

        #[cfg(cxxqt_qt_version_at_least_6_8)]
        include!("cxx-qt-io/qhttpheaders.h");
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        type QHttpHeaders = crate::QHttpHeaders;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkproxy.h");
        type ProxyCapability;
        #[allow(unused)]
        type ProxyCapabilities = super::ProxyCapabilities;
        type ProxyType;
    }

    unsafe extern "C++" {
        type QNetworkProxy = super::QNetworkProxy;

        /// Returns the capabilities of this proxy server.
        fn capabilities(self: &QNetworkProxy) -> ProxyCapabilities;

        /// Returns `true` if the raw header `header_name` is in use for this proxy. Returns `false` if the proxy is not of type `HttpProxy` or `HttpCachingProxy`.
        #[rust_name = "has_raw_header"]
        fn hasRawHeader(self: &QNetworkProxy, header_name: &QByteArray) -> bool;

        /// Returns the value of the known network header `header` if it is in use for this proxy. If it is not present, returns `QVariant()` (i.e., an invalid variant).
        fn header(self: &QNetworkProxy, header: KnownHeaders) -> QVariant;

        /// Returns headers that are set in this network request.
        ///
        /// If the proxy is not of type `HttpProxy` or `HttpCachingProxy`, default constructed `QHttpHeaders` is returned.
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        fn headers(self: &QNetworkProxy) -> QHttpHeaders;

        /// Returns the host name of the proxy host.
        #[rust_name = "host_name"]
        fn hostName(self: &QNetworkProxy) -> QString;

        /// Returns `true` if this proxy supports the `CachingCapability` capability.
        ///
        /// It is possible to remove the capability of caching from a proxy by calling `set_capabilities()`.
        #[rust_name = "is_caching_proxy"]
        fn isCachingProxy(self: &QNetworkProxy) -> bool;

        /// Returns `true` if this proxy supports transparent tunneling of TCP connections. This matches the `TunnelingCapability` capability.
        ///
        /// It is possible to remove the capability of caching from a proxy by calling `set_capabilities()`.
        #[rust_name = "is_transparent_proxy"]
        fn isTransparentProxy(self: &QNetworkProxy) -> bool;

        /// Returns the password used for authentication.
        fn password(self: &QNetworkProxy) -> QString;

        /// Returns the port of the proxy host.
        fn port(self: &QNetworkProxy) -> u16;

        /// Returns the raw form of header `header_name`. If no such header is present or the proxy is not of type `HttpProxy` or `HttpCachingProxy`, an empty `QByteArray` is returned, which may be indistinguishable from a header that is present but has no content (use `has_raw_header()` to find out if the header exists or not).
        ///
        /// Raw headers can be set with `set_raw_header()` or with `set_header()`.
        #[rust_name = "raw_header"]
        fn rawHeader(self: &QNetworkProxy, header_name: &QByteArray) -> QByteArray;

        /// Returns a list of all raw headers that are set in this network proxy. The list is in the order that the headers were set.
        ///
        /// If the proxy is not of type `HttpProxy` or `HttpCachingProxy` an empty `QList` is returned.
        #[rust_name = "raw_header_list"]
        fn rawHeaderList(self: &QNetworkProxy) -> QList_QByteArray;

        /// Sets the capabilities of this proxy to capabilities.
        #[rust_name = "set_capabilities"]
        fn setCapabilities(self: &mut QNetworkProxy, capabilities: ProxyCapabilities);

        /// Sets the value of the known header `header` to be `value`, overriding any previously set headers. This operation also sets the equivalent raw HTTP header.
        ///
        /// If the proxy is not of type `HttpProxy` or `HttpCachingProxy` this has no effect.
        #[rust_name = "set_header"]
        fn setHeader(self: &mut QNetworkProxy, header: KnownHeaders, value: &QVariant);

        /// Sets `new_headers` as headers in this network request, overriding any previously set headers.
        ///
        /// If some headers correspond to the known headers, the values will be parsed and the corresponding parsed form will also be set.
        ///
        /// If the proxy is not of type `HttpProxy` or `HttpCachingProxy` this has no effect.
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        #[rust_name = "set_headers"]
        fn setHeaders(self: &mut QNetworkProxy, new_headers: &QHttpHeaders);

        /// Sets the host name of the proxy host to be `host_name`.
        #[rust_name = "set_host_name"]
        fn setHostName(self: &mut QNetworkProxy, host_name: &QString);

        /// Sets the password for proxy authentication to be `password`.
        #[rust_name = "set_password"]
        fn setPassword(self: &mut QNetworkProxy, password: &QString);

        /// Sets the port of the proxy host to be `port`.
        #[rust_name = "set_port"]
        fn setPort(self: &mut QNetworkProxy, port: u16);

        /// Sets the header `header_name` to be of value `header_value`. If `header_name` corresponds to a known header (see `KnownHeaders`), the raw format will be parsed and the corresponding "cooked" header will be set as well.
        ///
        /// If the proxy is not of type `HttpProxy` or `HttpCachingProxy` this has no effect.
        #[rust_name = "set_raw_header"]
        fn setRawHeader(
            self: &mut QNetworkProxy,
            header_name: &QByteArray,
            header_value: &QByteArray,
        );

        /// Sets the proxy type for this instance to be `proxy_type`.
        ///
        /// Note that changing the type of a proxy does not change the set of capabilities this `QNetworkProxy` object holds if any capabilities have been set with `set_capabilities()`.
        #[rust_name = "set_type"]
        fn setType(self: &mut QNetworkProxy, proxy_type: ProxyType);

        /// Sets the user for proxy authentication to be `user`.
        #[rust_name = "set_user"]
        fn setUser(self: &mut QNetworkProxy, user: &QString);

        /// Returns the proxy type for this instance.
        #[cxx_name = "type"]
        fn proxy_type(self: &QNetworkProxy) -> ProxyType;

        /// Returns the user name used for authentication.
        fn user(self: &QNetworkProxy) -> QString;

    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qnetworkproxy_application_proxy"]
        fn qnetworkproxyApplicationProxy() -> QNetworkProxy;

        #[doc(hidden)]
        #[rust_name = "qnetworkproxy_set_application_proxy"]
        fn qnetworkproxySetApplicationProxy(proxy: &QNetworkProxy);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qnetworkproxy_drop"]
        fn drop(proxy: &mut QNetworkProxy);

        #[doc(hidden)]
        #[rust_name = "qnetworkproxy_init_default"]
        fn construct() -> QNetworkProxy;
        #[doc(hidden)]
        #[rust_name = "qnetworkproxy_clone"]
        fn construct(other: &QNetworkProxy) -> QNetworkProxy;
        #[doc(hidden)]
        #[rust_name = "qnetworkproxy_eq"]
        fn operatorEq(a: &QNetworkProxy, b: &QNetworkProxy) -> bool;
        #[doc(hidden)]
        #[rust_name = "qnetworkproxy_to_debug_qstring"]
        fn toDebugQString(value: &QNetworkProxy) -> QString;
    }
}

pub use ffi::{ProxyCapability, ProxyType};

pub type ProxyCapabilities = QFlags<ProxyCapability>;

unsafe impl QFlag for ProxyCapability {
    type TypeId = type_id!("ProxyCapabilities");

    type Repr = i32;

    fn to_repr(self) -> Self::Repr {
        self.repr
    }
}

/// The `QNetworkProxy` class provides a network layer proxy.
///
/// Qt Documentation: [QNetworkProxy](https://doc.qt.io/qt-6/qnetworkproxy.html#details)
#[repr(C)]
pub struct QNetworkProxy {
    _space: MaybeUninit<usize>,
}

impl Clone for QNetworkProxy {
    fn clone(&self) -> Self {
        ffi::qnetworkproxy_clone(self)
    }
}

impl Default for QNetworkProxy {
    fn default() -> Self {
        ffi::qnetworkproxy_init_default()
    }
}

impl Drop for QNetworkProxy {
    fn drop(&mut self) {
        ffi::qnetworkproxy_drop(self);
    }
}

impl PartialEq for QNetworkProxy {
    fn eq(&self, other: &Self) -> bool {
        ffi::qnetworkproxy_eq(self, other)
    }
}

impl Eq for QNetworkProxy {}

impl Debug for QNetworkProxy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ffi::qnetworkproxy_to_debug_qstring(self))
    }
}

impl QNetworkProxy {
    /// Returns the application level network proxying.
    ///
    /// If a `QAbstractSocket` or `QTcpSocket` has the `DefaultProxy` type, then the `QNetworkProxy` returned by this function is used.
    pub fn application_proxy() -> Self {
        ffi::qnetworkproxy_application_proxy()
    }

    /// Sets the application level network proxying to be `network_proxy`.
    ///
    /// If a `QAbstractSocket` or `QTcpSocket` has the `DefaultProxy` type, then the `QNetworkProxy` set with this function is used.
    ///
    /// Setting a default proxy value with this function will disable the use of a system proxy.
    pub fn set_application_proxy(network_proxy: &Self) {
        ffi::qnetworkproxy_set_application_proxy(network_proxy);
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkProxy {
    type Id = type_id!("QNetworkProxy");
    type Kind = cxx::kind::Trivial;
}

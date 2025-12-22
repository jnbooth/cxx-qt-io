use std::fmt;
use std::mem::MaybeUninit;

use cxx::{ExternType, type_id};
use cxx_qt_lib::{QFlags, QVariant};

use crate::QNetworkRequestKnownHeaders;
use crate::util::IsNonNull;

#[cxx::bridge]
mod ffi {
    /// A capability that a given proxy server supports.
    ///
    /// [`QNetworkProxy`] sets different capabilities by default when the object is created (see [`QNetworkProxyProxyType`] for a list of the defaults). However, it is possible to change the capabilities after the object has been created with [`QNetworkProxy::set_capabilities`].
    #[repr(i32)]
    enum QNetworkProxyCapability {
        /// Ability to open transparent, tunneled TCP connections to a remote host. The proxy server relays the transmission verbatim from one side to the other and does no caching.
        TunnelingCapability = 0x0001,
        /// Ability to create a listening socket and wait for an incoming TCP connection from a remote host.
        ListeningCapability = 0x0002,
        /// Ability to relay UDP datagrams via the proxy server to and from a remote host.
        UdpTunnelingCapability = 0x0004,
        /// Ability to cache the contents of the transfer. This capability is specific to each protocol and proxy type. For example, HTTP proxies can cache the contents of web data transferred with "GET" commands.
        CachingCapability = 0x0008,
        /// Ability to connect to perform the lookup on a remote host name and connect to it, as opposed to requiring the application to perform the name lookup and request connection to IP addresses only.
        HostNameLookupCapability = 0x0010,
        /// Ability to open transparent, tunneled SCTP connections to a remote host.
        SctpTunnelingCapability = 0x00020,
        /// Ability to create a listening socket and wait for an incoming SCTP connection from a remote host.
        SctpListeningCapability = 0x00040,
    }

    /// This enum describes the types of network proxying provided in Qt.
    ///
    /// There are two types of proxies that Qt understands: transparent proxies and caching proxies. The first group consists of proxies that can handle any arbitrary data transfer, while the second can only handle specific requests. The caching proxies only make sense for the specific classes where they can be used.
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkProxyProxyType {
        /// Proxy is determined based on the application proxy set using [`QNetworkProxy::set_application_proxy`].
        DefaultProxy,
        /// Socks5 proxying is used.
        Socks5Proxy,
        /// No proxying is used.
        NoProxy,
        /// HTTP transparent proxying is used.
        HttpProxy,
        /// Proxying for HTTP requests only.
        HttpCachingProxy,
        /// Proxying for FTP requests only.
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
        type QNetworkRequestKnownHeaders = crate::QNetworkRequestKnownHeaders;
    }

    #[cfg(cxxqt_qt_version_at_least_6_8)]
    extern "C++" {
        include!("cxx-qt-io/qhttpheaders.h");
        type QHttpHeaders = crate::QHttpHeaders;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkproxy.h");
        type QNetworkProxyCapability;
        #[allow(unused)]
        type QNetworkProxyCapabilities = super::QNetworkProxyCapabilities;
        type QNetworkProxyProxyType;
    }

    unsafe extern "C++" {
        type QNetworkProxy = super::QNetworkProxy;

        /// Returns the capabilities of this proxy server.
        fn capabilities(&self) -> QNetworkProxyCapabilities;

        /// Returns `true` if the raw header `header_name` is in use for this proxy. Returns `false` if the proxy is not of type [`QNetworkProxyProxyType::HttpProxy`] or [`QNetworkProxyProxyType::HttpCachingProxy`].
        #[rust_name = "has_raw_header"]
        fn hasRawHeader(&self, header_name: &QByteArray) -> bool;

        #[doc(hidden)]
        #[rust_name = "header_or_invalid"]
        fn header(&self, header: QNetworkRequestKnownHeaders) -> QVariant;

        /// Returns headers that are set in this network request.
        ///
        /// If the proxy is not of type [`QNetworkProxyProxyType::HttpProxy`] or [`QNetworkProxyProxyType::HttpCachingProxy`], default constructed `QHttpHeaders` is returned.
        ///
        /// Introduced in Qt 6.8.
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        fn headers(&self) -> QHttpHeaders;

        /// Returns the host name of the proxy host.
        #[rust_name = "host_name"]
        fn hostName(&self) -> QString;

        /// Returns `true` if this proxy supports the [`QNetworkProxyCapability::CachingCapability`] capability.
        ///
        /// It is possible to remove the capability of caching from a proxy by calling [`set_capabilities`](QNetworkProxy::set_capabilities).
        #[rust_name = "is_caching_proxy"]
        fn isCachingProxy(&self) -> bool;

        /// Returns `true` if this proxy supports transparent tunneling of TCP connections. This matches the [`QNetworkProxyCapability::TunnelingCapability`] capability.
        ///
        /// It is possible to remove the capability of caching from a proxy by calling [`set_capabilities`](QNetworkProxy::set_capabilities).
        #[rust_name = "is_transparent_proxy"]
        fn isTransparentProxy(&self) -> bool;

        /// Returns the password used for authentication.
        fn password(&self) -> QString;

        /// Returns the port of the proxy host.
        fn port(&self) -> u16;

        /// Returns the raw form of header `header_name`. If no such header is present or the proxy is not of type [`QNetworkProxyProxyType::HttpProxy`] or [`QNetworkProxyProxyType::HttpCachingProxy`], an empty `QByteArray` is returned, which may be indistinguishable from a header that is present but has no content (use [`has_raw_header`](QNetworkProxy::has_raw_header) to find out if the header exists or not).
        ///
        /// Raw headers can be set with [`set_raw_header`](QNetworkProxy::set_raw_header) or with [`set_header`](QNetworkProxy::set_header).
        #[rust_name = "raw_header"]
        fn rawHeader(&self, header_name: &QByteArray) -> QByteArray;

        /// Returns a list of all raw headers that are set in this network proxy. The list is in the order that the headers were set.
        ///
        /// If the proxy is not of type [`QNetworkProxyProxyType::HttpProxy`] or [`QNetworkProxyProxyType::HttpCachingProxy`] an empty `QList` is returned.
        #[rust_name = "raw_header_list"]
        fn rawHeaderList(&self) -> QList_QByteArray;

        /// Sets the capabilities of this proxy to capabilities.
        #[rust_name = "set_capabilities"]
        fn setCapabilities(&mut self, capabilities: QNetworkProxyCapabilities);

        #[doc(hidden)]
        #[rust_name = "set_header_variant"]
        fn setHeader(&mut self, header: QNetworkRequestKnownHeaders, value: &QVariant);

        /// Sets `new_headers` as headers in this network request, overriding any previously set headers.
        ///
        /// If some headers correspond to the known headers, the values will be parsed and the corresponding parsed form will also be set.
        ///
        /// If the proxy is not of type [`QNetworkProxyProxyType::HttpProxy`] or [`QNetworkProxyProxyType::HttpCachingProxy`] this has no effect.
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        #[rust_name = "set_headers"]
        fn setHeaders(&mut self, new_headers: &QHttpHeaders);

        /// Sets the host name of the proxy host to be `host_name`.
        #[rust_name = "set_host_name"]
        fn setHostName(&mut self, host_name: &QString);

        /// Sets the password for proxy authentication to be `password`.
        #[rust_name = "set_password"]
        fn setPassword(&mut self, password: &QString);

        /// Sets the port of the proxy host to be `port`.
        #[rust_name = "set_port"]
        fn setPort(&mut self, port: u16);

        /// Sets the header `header_name` to be of value `header_value`. If `header_name` corresponds to a known header (see [`QNetworkRequestKnownHeaders`](crate::QNetworkRequestKnownHeaders)), the raw format will be parsed and the corresponding "cooked" header will be set as well.
        ///
        /// If the proxy is not of type [`QNetworkProxyProxyType::HttpProxy`] or [`QNetworkProxyProxyType::HttpCachingProxy`] this has no effect.
        #[rust_name = "set_raw_header"]
        fn setRawHeader(&mut self, header_name: &QByteArray, header_value: &QByteArray);

        /// Sets the proxy type for this instance to be `proxy_type`.
        ///
        /// Note that changing the type of a proxy does not change the set of capabilities this `QNetworkProxy` object holds if any capabilities have been set with [`set_capabilities`](QNetworkProxy::set_capabilities).
        #[rust_name = "set_proxy_type"]
        fn setType(&mut self, proxy_type: QNetworkProxyProxyType);

        /// Sets the user for proxy authentication to be `user`.
        #[rust_name = "set_user"]
        fn setUser(&mut self, user: &QString);

        /// Returns the proxy type for this instance.
        #[cxx_name = "type"]
        fn proxy_type(&self) -> QNetworkProxyProxyType;

        /// Returns the user name used for authentication.
        fn user(&self) -> QString;

    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qnetworkproxy_application_proxy"]
        fn qnetworkproxyApplicationProxy() -> QNetworkProxy;

        #[rust_name = "qnetworkproxy_set_application_proxy"]
        fn qnetworkproxySetApplicationProxy(proxy: &QNetworkProxy);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkproxy_drop"]
        fn drop(proxy: &mut QNetworkProxy);

        #[rust_name = "qnetworkproxy_init_default"]
        fn construct() -> QNetworkProxy;
        #[rust_name = "qnetworkproxy_init_from_proxytype"]
        fn construct(proxy_type: QNetworkProxyProxyType) -> QNetworkProxy;
        #[rust_name = "qnetworkproxy_clone"]
        fn construct(other: &QNetworkProxy) -> QNetworkProxy;

        #[rust_name = "qnetworkproxy_eq"]
        fn operatorEq(a: &QNetworkProxy, b: &QNetworkProxy) -> bool;

        #[rust_name = "qnetworkproxy_to_debug_qstring"]
        fn toDebugQString(value: &QNetworkProxy) -> QString;
    }
}

pub use ffi::{QNetworkProxyCapability, QNetworkProxyProxyType};

/// [`QFlags`] of [`QNetworkProxyCapability`].
pub type QNetworkProxyCapabilities = QFlags<QNetworkProxyCapability>;

unsafe_impl_qflag!(QNetworkProxyCapability, "QNetworkProxyCapabilities");

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
    /// Constructs a `QNetworkProxy` with [`QNetworkProxyProxyType::DefaultProxy`] type.
    ///
    /// The proxy type is determined by [`QNetworkProxy::application_proxy()`], which defaults to [`QNetworkProxyProxyType::NoProxy`] or a system-wide proxy if one is configured.
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

impl fmt::Debug for QNetworkProxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qnetworkproxy_to_debug_qstring(self).fmt(f)
    }
}

impl QNetworkProxy {
    /// Returns the application level network proxying.
    ///
    /// If a [`QAbstractSocket`](crate::QAbstractSocket) or [`QTcpSocket`](crate::QTcpSocket) has the [`QNetworkProxyProxyType::DefaultProxy`] type, then the `QNetworkProxy` returned by this function is used.
    pub fn application_proxy() -> Self {
        ffi::qnetworkproxy_application_proxy()
    }

    /// Returns the value of the known network header `header` if it is in use for this proxy. If it is not present, returns `None`.
    pub fn header(&self, header: QNetworkRequestKnownHeaders) -> Option<QVariant> {
        self.header_or_invalid(header).nonnull()
    }

    /// Sets the application level network proxying to be `network_proxy`.
    ///
    /// If a [`QAbstractSocket`](crate::QAbstractSocket) or [`QTcpSocket`](crate::QTcpSocket) has the [`QNetworkProxyProxyType::DefaultProxy`] type, then the `QNetworkProxy` set with this function is used.
    ///
    /// Setting a default proxy value with this function will disable the use of a system proxy.
    pub fn set_application_proxy(network_proxy: &Self) {
        ffi::qnetworkproxy_set_application_proxy(network_proxy);
    }

    /// Sets the value of the known header `header` to be `value`, overriding any previously set headers. This operation also sets the equivalent raw HTTP header.
    ///
    /// If the proxy is not of type [`QNetworkProxyProxyType::HttpProxy`] or [`QNetworkProxyProxyType::HttpCachingProxy`] this has no effect.
    pub fn set_header<T>(&mut self, header: QNetworkRequestKnownHeaders, value: T)
    where
        T: Into<QVariant>,
    {
        self.set_header_variant(header, &value.into());
    }
}

impl From<QNetworkProxyProxyType> for QNetworkProxy {
    fn from(value: QNetworkProxyProxyType) -> Self {
        ffi::qnetworkproxy_init_from_proxytype(value)
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkProxy {
    type Id = type_id!("QNetworkProxy");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use cxx_qt_lib::QString;

    use super::*;

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QNetworkProxyProps {
            capabilities: QNetworkProxyCapabilities,
            host_name: QString,
            password: QString,
            port: u16,
            proxy_type: QNetworkProxyProxyType,
            user: QString,
        }

        let props = QNetworkProxyProps {
            capabilities: QNetworkProxyCapability::CachingCapability.into(),
            host_name: QString::from("host"),
            password: QString::from("password"),
            port: 32,
            proxy_type: QNetworkProxyProxyType::FtpCachingProxy,
            user: QString::from("user"),
        };

        let mut proxy = QNetworkProxy::default();

        proxy.set_capabilities(props.capabilities);
        proxy.set_host_name(&props.host_name);
        proxy.set_password(&props.password);
        proxy.set_port(props.port);
        proxy.set_proxy_type(props.proxy_type);
        proxy.set_user(&props.user);

        let actual_props = QNetworkProxyProps {
            capabilities: proxy.capabilities(),
            host_name: proxy.host_name(),
            password: proxy.password(),
            port: proxy.port(),
            proxy_type: proxy.proxy_type(),
            user: proxy.user(),
        };

        assert_eq!(actual_props, props);
    }
}

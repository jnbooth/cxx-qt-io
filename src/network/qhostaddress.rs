use std::fmt::{self, Debug, Display, Formatter};
use std::mem::MaybeUninit;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ptr;

use crate::util::IsNonNull;
use crate::{QAbstractSocketNetworkLayerProtocol, QPair, QPairPair_QHostAddress_i32};
use cxx::{type_id, ExternType};
use cxx_qt_lib::{QFlags, QString};

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(Debug)]
    enum QHostAddressConversionModeFlag {
        /// Convert IPv4-mapped IPv6 addresses ([RFC 4291 sect. 2.5.5.2](https://datatracker.ietf.org/doc/html/rfc4291#section-2.5.5.2)) when comparing. Therefore a [`QHostAddress`] with address `::ffff:192.168.1.1`` will compare equal to one with `192.168.1.1`.
        ConvertV4MappedToIPv4 = 1,
        /// Convert IPv4-compatible IPv6 addresses ([RFC 4291 sect. 2.5.5.1](https://datatracker.ietf.org/doc/html/rfc4291#section-2.5.5.1)) when comparing. Therefore a [`QHostAddress`] with address `::192.168.1.1` will compare equal to one with `192.168.1.1`.
        ConvertV4CompatToIPv4 = 2,
        ConvertUnspecifiedAddress = 4,
        /// Convert the IPv6 loopback addresses to its IPv4 equivalent when comparing. Therefore e.g. a [`QHostAddress`] with address `::1` will compare equal to one with `127.0.0.1`.
        ConvertLocalHost = 8,
        /// Sets all three preceding flags.
        TolerantConversion = 0xff,

        /// Don't convert IPv6 addresses to IPv4 when comparing two `QHostAddress` objects of different protocols, so they will always be considered different.
        StrictConversion = 0,
    }

    #[repr(i32)]
    #[derive(Debug)]
    enum QHostAddressSpecialAddress {
        /// The null address object. Equivalent to [`QHostAddress::default()`]. See also [`QHostAddress::is_null`].
        Null,
        /// The IPv4 localhost address. Equivalent to `127.0.0.1`.
        Broadcast,
        /// The IPv6 localhost address. Equivalent to `::1`.
        LocalHost,
        /// The IPv4 broadcast address. Equivalent to `255.255.255.255`.
        LocalHostIPv6,
        /// The dual stack any-address. A socket bound with this address will listen on both IPv4 and IPv6 interfaces.
        Any,
        /// The IPv6 any-address. Equivalent to `::`. A socket bound with this address will listen only on IPv6 interfaces.
        AnyIPv6,
        /// The IPv4 any-address. Equivalent to `0.0.0.0`. A socket bound with this address will listen only on IPv4 interfaces.
        AnyIPv4,
    }

    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-io/qabstractsocket.h");
        type QAbstractSocketNetworkLayerProtocol = crate::QAbstractSocketNetworkLayerProtocol;
        include!("cxx-qt-io/qpair_qhostaddress_i32.h");
        type QPair_QHostAddress_i32 = crate::QPair<crate::QPairPair_QHostAddress_i32>;
    }

    extern "C++" {
        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddressConversionModeFlag;
        #[allow(unused)]
        type QHostAddressConversionMode = crate::QHostAddressConversionMode;
        type QHostAddressSpecialAddress;

        #[cxx_name = "Q_IPV6ADDR"]
        type QIpv6Addr = super::QIpv6Addr;
    }

    unsafe extern "C++" {
        type QHostAddress = super::QHostAddress;

        /// Sets the host address to null and sets the protocol to [`QAbstractSocketNetworkLayerProtocol::UnknownNetworkLayerProtocol`](crate::QAbstractSocketNetworkLayerProtocol::UnknownNetworkLayerProtocol).
        fn clear(&mut self);

        /// Returns `true` if the address is the IPv4 broadcast address, `false` otherwise. The IPv4 broadcast address is `255.255.255.255`.
        ///
        /// Note that this function does not return `true` for an IPv4 network's local broadcast address. For that, please use [`QNetworkInterface`](crate::QNetworkInterface) to obtain the broadcast addresses of the local machine.
        #[rust_name = "is_broadcast"]
        fn isBroadcast(&self) -> bool;

        /// Returns `true` if the address is an IPv4 or IPv6 global address, `false` otherwise. A global address is an address that is not reserved for special purposes (like loopback or multicast) or future purposes.
        ///
        /// Note that IPv6 unique local unicast addresses are considered global addresses (see [`is_unique_local_unicast`](QHostAddress::is_unique_local_unicast)), as are IPv4 addresses reserved for local networks by [RFC 1918](https://datatracker.ietf.org/doc/html/rfc1918).
        ///
        /// Also note that IPv6 site-local addresses are deprecated and should be considered as global in new applications. This function returns `true` for site-local addresses too.
        #[rust_name = "is_global"]
        fn isGlobal(&self) -> bool;

        /// Returns `true` if this IP is in the subnet described by the network prefix `subnet` and netmask `netmask`.
        ///
        /// An IP is considered to belong to a subnet if it is contained between the lowest and the highest address in that subnet. In the case of IP version 4, the lowest address is the network address, while the highest address is the broadcast address.
        ///
        /// The `subnet` argument does not have to be the actual network address (the lowest address in the subnet). It can be any valid IP belonging to that subnet. In particular, if it is equal to the IP address held by this object, this function will always return `true` (provided the netmask is a valid value).
        #[rust_name = "is_in_subnet"]
        fn isInSubnet(&self, subnet: &QHostAddress, netmask: i32) -> bool;

        /// Returns `true` if the address is an IPv4 or IPv6 link-local address, `false` otherwise.
        ///
        /// An IPv4 link-local address is an address in the network `169.254.0.0/16`. An IPv6 link-local address is one in the network `fe80::/10`. See the [IANA IPv6 Address Space](https://www.iana.org/assignments/ipv6-address-space/ipv6-address-space.xhtml) registry for more information.
        #[rust_name = "is_link_local"]
        fn isLinkLocal(&self) -> bool;

        /// Returns `true` if the address is the IPv6 loopback address, or any of the IPv4 loopback addresses.
        #[rust_name = "is_loopback"]
        fn isLoopback(&self) -> bool;

        /// Returns `true` if the address is an IPv4 or IPv6 multicast address, `false` otherwise.
        #[rust_name = "is_multicast"]
        fn isMulticast(&self) -> bool;

        /// Returns `true` if this host address is not valid for any host or interface.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        /// Returns `true` if the address is an IPv6 unique local unicast address or IPv4 address reserved for local networks by [RFC 1918](https://datatracker.ietf.org/doc/html/rfc1918), `false` otherwise.
        #[cfg(cxxqt_qt_version_at_least_6_6)]
        #[rust_name = "is_private_use"]
        fn isPrivateUse(&self) -> bool;

        /// Returns `true` if the address is an IPv6 site-local address, `false` otherwise.
        ///
        /// An IPv6 site-local address is one in the network fec0::/10. See the [IANA IPv6 Address Space](https://www.iana.org/assignments/ipv6-address-space/ipv6-address-space.xhtml) registry for more information.
        ///
        /// IPv6 site-local addresses are deprecated and should not be depended upon in new applications. New applications should not depend on this function and should consider site-local addresses the same as global (which is why `is_global()` also returns `true`). Site-local addresses were replaced by Unique Local Addresses (ULA).
        #[rust_name = "is_site_local"]
        fn isSiteLocal(&self) -> bool;

        /// Returns `true` if the address is an IPv6 unique local unicast address, `false` otherwise.
        ///
        /// An IPv6 unique local unicast address is one in the network fc00::/7. See the [IANA IPv6 Address Space](https://www.iana.org/assignments/ipv6-address-space/ipv6-address-space.xhtml) registry for more information.
        ///
        /// Note that Unique local unicast addresses count as global addresses too. [RFC 4193](https://datatracker.ietf.org/doc/html/rfc4193) says that, in practice, "applications may treat these addresses like global scoped addresses." Only routers need care about the distinction.
        #[rust_name = "is_unique_local_unicast"]
        fn isUniqueLocalUnicast(&self) -> bool;

        /// Returns the network layer protocol of the host address.
        fn protocol(&self) -> QAbstractSocketNetworkLayerProtocol;

        /// Returns the scope ID of an IPv6 address. For IPv4 addresses, or if the address does not contain a scope ID, an empty `QString` is returned.
        ///
        /// The IPv6 scope ID specifies the scope of reachability for non-global IPv6 addresses, limiting the area in which the address can be used. All IPv6 addresses are associated with such a reachability scope. The scope ID is used to disambiguate addresses that are not guaranteed to be globally unique.
        ///
        /// IPv6 specifies the following four levels of reachability:
        ///
        /// * Node-local: Addresses that are only used for communicating with services on the same interface (e.g., the loopback interface `::1`).
        /// * Link-local: Addresses that are local to the network interface (link). There is always one link-local address for each IPv6 interface on your host. Link-local addresses `fe80...` are generated from the MAC address of the local network adaptor, and are not guaranteed to be unique.
        /// * Global: For globally routable addresses, such as public servers on the Internet.
        ///
        /// When using a link-local or site-local address for IPv6 connections, you must specify the scope ID. The scope ID for a link-local address is usually the same as the interface name (e.g., `"eth0"`, `"en1"`) or number (e.g., `"1"`, `"2"`).
        #[rust_name = "scope_id"]
        fn scopeId(&self) -> QString;

        /// Sets the IPv6 scope ID of the address to `id`. If the address protocol is not IPv6, this function does nothing. The scope ID may be set as an interface name (such as `"eth0"` or `"en1"`) or as an integer representing the interface index. If `id` is an interface name, QtNetwork will convert to an interface index using [`QNetworkInterface::interface_index_from_name`](crate::QNetworkInterface::interface_index_from_name) before calling the operating system networking functions.
        #[rust_name = "set_scope_id"]
        fn setScopeId(&mut self, id: &QString);

        #[doc(hidden)]
        #[rust_name = "to_ipv4_address"]
        unsafe fn toIPv4Address(&self, ok: *mut bool) -> u32;

        #[rust_name = "to_qstring"]
        fn toString(&self) -> QString;

    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qhostaddress_to_ipv6_address"]
        fn qhostaddressToIPv6Address(address: &QHostAddress) -> QIpv6Addr;

        #[rust_name = "qhostaddress_parse_subnet"]
        fn qhostaddressParseSubnet(subnet: &QString) -> QPair_QHostAddress_i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhostaddress_drop"]
        fn drop(address: &mut QHostAddress);

        #[rust_name = "qhostaddress_init_default"]
        fn construct() -> QHostAddress;
        #[rust_name = "qhostaddress_from_qipv6addr"]
        fn construct(address: &QIpv6Addr) -> QHostAddress;
        #[rust_name = "qhostaddress_from_qstring"]
        fn construct(index: &QString) -> QHostAddress;
        #[rust_name = "qhostaddress_from_specialaddress"]
        fn construct(address: &QHostAddressSpecialAddress) -> QHostAddress;
        #[rust_name = "qhostaddress_from_uint32"]
        fn construct(address: u32) -> QHostAddress;
        #[rust_name = "qhostaddress_clone"]
        fn construct(other: &QHostAddress) -> QHostAddress;

        #[rust_name = "qhostaddress_eq"]
        fn operatorEq(a: &QHostAddress, b: &QHostAddress) -> bool;

        #[rust_name = "qhostaddress_to_debug_qstring"]
        fn toDebugQString(value: &QHostAddress) -> QString;
    }
}

pub use ffi::{QHostAddressConversionModeFlag, QHostAddressSpecialAddress};

/// [`QFlags`] of [`QHostAddressConversionModeFlag`].
pub type QHostAddressConversionMode = QFlags<QHostAddressConversionModeFlag>;

unsafe_impl_qflag!(QHostAddressConversionModeFlag, "QHostAddressConversionMode");

/// The `QHostAddress` class provides an IP address.
///
/// Qt Documentation: [QHostAddress](https://doc.qt.io/qt-6/qhostaddress.html#details)
#[repr(C)]
pub struct QHostAddress {
    _space: MaybeUninit<usize>,
}

impl Clone for QHostAddress {
    fn clone(&self) -> Self {
        ffi::qhostaddress_clone(self)
    }
}

impl Default for QHostAddress {
    /// Returns a null address.
    fn default() -> Self {
        ffi::qhostaddress_init_default()
    }
}

impl Drop for QHostAddress {
    fn drop(&mut self) {
        ffi::qhostaddress_drop(self);
    }
}

impl PartialEq for QHostAddress {
    fn eq(&self, other: &Self) -> bool {
        ffi::qhostaddress_eq(self, other)
    }
}

impl Eq for QHostAddress {}

impl Display for QHostAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.to_qstring())
    }
}

impl Debug for QHostAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qhostaddress_to_debug_qstring(self))
    }
}

impl IsNonNull for QHostAddress {
    fn is_nonnull(value: &Self) -> bool {
        !value.is_null()
    }
}

impl QHostAddress {
    pub fn parse_subnet(subnet: &QString) -> QPair<QPairPair_QHostAddress_i32> {
        ffi::qhostaddress_parse_subnet(subnet)
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QHostAddress {
    type Id = type_id!("QHostAddress");
    type Kind = cxx::kind::Trivial;
}

impl From<&QString> for QHostAddress {
    fn from(value: &QString) -> Self {
        ffi::qhostaddress_from_qstring(value)
    }
}

impl From<QHostAddressSpecialAddress> for QHostAddress {
    fn from(value: QHostAddressSpecialAddress) -> Self {
        ffi::qhostaddress_from_specialaddress(&value)
    }
}

impl From<Ipv4Addr> for QHostAddress {
    fn from(value: Ipv4Addr) -> Self {
        ffi::qhostaddress_from_uint32(value.to_bits())
    }
}

impl TryFrom<QHostAddress> for Ipv4Addr {
    type Error = &'static str;

    fn try_from(value: QHostAddress) -> Result<Self, Self::Error> {
        let mut ok = false;
        // SAFETY: ptr::from_mut(&mut ok) is valid.
        let address = unsafe { value.to_ipv4_address(ptr::from_mut(&mut ok)) };
        if !ok {
            return Err("address is neither an IPv4 address nor an IPv4-mapped IPv6 address");
        }
        Ok(Self::from_bits(address))
    }
}

impl From<Ipv6Addr> for QHostAddress {
    fn from(value: Ipv6Addr) -> Self {
        ffi::qhostaddress_from_qipv6addr(&value.into())
    }
}

impl From<QHostAddress> for Ipv6Addr {
    fn from(value: QHostAddress) -> Self {
        ffi::qhostaddress_to_ipv6_address(&value).into()
    }
}

impl From<IpAddr> for QHostAddress {
    fn from(value: IpAddr) -> Self {
        match value {
            IpAddr::V4(addr) => addr.into(),
            IpAddr::V6(addr) => addr.into(),
        }
    }
}

impl From<QHostAddress> for IpAddr {
    fn from(value: QHostAddress) -> Self {
        if value.protocol() != QAbstractSocketNetworkLayerProtocol::IPv4Protocol {
            return IpAddr::V6(value.into());
        }
        // SAFETY: Null pointer is ignored.
        let address = unsafe { value.to_ipv4_address(ptr::null_mut()) };
        IpAddr::V4(Ipv4Addr::from_bits(address))
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct QIpv6Addr {
    c: [u8; 16],
}

unsafe impl ExternType for QIpv6Addr {
    type Id = type_id!("Q_IPV6ADDR");
    type Kind = cxx::kind::Trivial;
}

impl From<Ipv6Addr> for QIpv6Addr {
    fn from(value: Ipv6Addr) -> Self {
        Self { c: value.octets() }
    }
}

impl From<QIpv6Addr> for Ipv6Addr {
    fn from(value: QIpv6Addr) -> Self {
        Self::from(value.c)
    }
}

use cxx::{type_id, ExternType};
use cxx_qt_lib::{QFlags, QList};
use std::fmt;
use std::mem::MaybeUninit;

use cxx_qt_lib::QString;

use crate::util::IsNonNull;
use crate::QHostAddress;

#[cxx::bridge]
mod ffi {
    /// Specifies the flags associated with this network interface.
    ///
    /// Note that one network interface cannot be both broadcast-based and point-to-point.
    #[repr(i32)]
    enum QNetworkInterfaceInterfaceFlag {
        /// The network interface is "up" - enabled by administrative action
        IsUp = 0x1,
        /// The network interface is operational: configured "up" and (typically) physically connected to a network
        IsRunning = 0x2,
        /// The network interface works in broadcast mode
        CanBroadcast = 0x4,
        /// The network interface is a loopback interface: that is, it's a virtual interface whose destination is the host computer itself
        IsLoopBack = 0x8,
        /// The network interface is a point-to-point interface: that is, there is one, single other address that can be directly reached by it
        IsPointToPoint = 0x10,
        /// The network interface supports multicasting
        CanMulticast = 0x20,
    }

    /// Specifies the type of hardware (PHY layer, OSI level 1) this interface is, if it could be determined. Interface types that are not among those listed below will generally be listed as Unknown, though future versions of Qt may add new enumeration values.
    #[repr(i32)]
    #[derive(Debug)]
    enum QNetworkInterfaceInterfaceType {
        /// The interface type could not be determined or is not one of the other listed types.
        Unknown,
        /// The virtual loopback interface, which is assigned the loopback IP addresses (`127.0.0.1`, `::1`).
        Loopback,
        /// A type of interface determined to be virtual, but not any of the other possible types. For example, tunnel interfaces are (currently) detected as virtual ones.
        Virtual,
        /// IEEE 802.3 Ethernet interfaces, though on many systems other types of IEEE 802 interfaces may also be detected as Ethernet (especially Wi-Fi).
        Ethernet,
        /// Serial Line Internet Protocol interfaces.
        Slip,
        /// ISO 11898 Controller Area Network bus interfaces, usually found on automotive systems.
        CanBus,
        /// Point-to-Point Protocol interfaces, establishing a direct connection between two nodes over a lower transport layer (often serial over radio or physical line).
        Ppp,
        /// ANSI X3T12 Fiber Distributed Data Interface, a local area network over optical fibers.
        Fddi,
        /// IEEE 802.11 Wi-Fi interfaces. Note that on some systems, `QNetworkInterface` may be unable to distinguish regular Ethernet from Wi-Fi and will not return this enum value.
        Wifi,
        /// Interfaces using the Linux Phonet socket family, for communication with cellular modems. See the [Linux kernel documentation](https://www.kernel.org/doc/Documentation/networking/phonet.txt) for more information.
        Phonet,
        /// IEEE 802.15.4 Personal Area Network interfaces, other than 6LoWPAN (see below).
        Ieee802154,
        /// 6LoWPAN (IPv6 over Low-power Wireless Personal Area Networks) interfaces, which operate on IEEE 802.15.4 PHY, but have specific header compression schemes for IPv6 and UDP. This type of interface is often used for mesh networking.
        SixLoWPAN,
        /// IEEE 802.16 Wireless Metropolitan Area Network, also known under the commercial name "WiMAX".
        Ieee80216,
        /// IEEE 1394 interfaces (a.k.a. "FireWire").
        Ieee1394,
    }

    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-io/qlist.h");
        type QList_QHostAddress = cxx_qt_lib::QList<crate::QHostAddress>;
        type QList_QNetworkAddressEntry = cxx_qt_lib::QList<crate::QNetworkAddressEntry>;
        type QList_QNetworkInterface = cxx_qt_lib::QList<QNetworkInterface>;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkinterface.h");
        type QNetworkInterfaceInterfaceFlag;
        type QNetworkInterfaceInterfaceType;
        #[allow(unused)]
        type QNetworkInterfaceInterfaceFlags = super::QNetworkInterfaceInterfaceFlags;
    }

    unsafe extern "C++" {
        type QNetworkInterface = super::QNetworkInterface;

        /// Returns the list of IP addresses that this interface possesses along with their associated netmasks and broadcast addresses.
        ///
        /// If the netmask or broadcast address or other information is not necessary, you can call the [`all_addresses`](QNetworkInterface::all_addresses) function to obtain just the IP addresses of the active interfaces.
        #[rust_name = "address_entries"]
        fn addressEntries(&self) -> QList_QNetworkAddressEntry;

        /// Returns the flags associated with this network interface.
        fn flags(&self) -> QNetworkInterfaceInterfaceFlags;

        /// Returns the low-level hardware address for this interface. On Ethernet interfaces, this will be a MAC address in string representation, separated by colons.
        ///
        /// Other interface types may have other types of hardware addresses. Implementations should not depend on this function returning a valid MAC address.
        #[rust_name = "hardware_address"]
        fn hardwareAddress(&self) -> QString;

        /// Returns the human-readable name of this network interface on Windows, such as "Local Area Connection", if the name could be determined. If it couldn't, this function returns the same as [`self.name()`](QNetworkInterface::name). The human-readable name is a name that the user can modify in the Windows Control Panel, so it may change during the execution of the program.
        ///
        /// On Unix, this function currently always returns the same as [`self.name()`](QNetworkInterface::name), since Unix systems don't store a configuration for human-readable names.
        #[rust_name = "human_readable_name"]
        fn humanReadableName(&self) -> QString;

        #[doc(hidden)]
        #[rust_name = "index_or_zero"]
        fn index(&self) -> i32;

        /// Returns true if this `QNetworkInterface` object contains valid information about a network interface.
        #[rust_name = "is_valid"]
        fn isValid(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "maximum_transmission_unit_or_zero"]
        fn maximumTransmissionUnit(&self) -> i32;

        /// Returns the name of this network interface. On Unix systems, this is a string containing the type of the interface and optionally a sequence number, such as `"eth0"`, `"lo"` or `"pcn0"`. On Windows, it's an internal ID that cannot be changed by the user.
        fn name(&self) -> QString;

        /// Returns the type of this interface, if it could be determined. If it could not be determined, this function returns [`QNetworkInterfaceInterfaceType::Unknown`].
        #[cxx_name = "type"]
        fn interface_type(&self) -> QNetworkInterfaceInterfaceType;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qnetworkinterface_all_addresses"]
        fn qnetworkinterfaceAllAddresses() -> QList_QHostAddress;

        #[rust_name = "qnetworkinterface_all_interfaces"]
        fn qnetworkinterfaceAllInterfaces() -> QList_QNetworkInterface;

        #[rust_name = "qnetworkinterface_interface_from_index"]
        fn qnetworkinterfaceInterfaceFromIndex(index: i32) -> QNetworkInterface;

        #[rust_name = "qnetworkinterface_interface_from_name"]
        fn qnetworkinterfaceInterfaceFromName(name: &QString) -> QNetworkInterface;

        #[rust_name = "qnetworkinterface_interface_index_from_name"]
        fn qnetworkinterfaceInterfaceIndexFromName(name: &QString) -> i32;

        #[rust_name = "qnetworkinterface_interface_name_from_index"]
        fn qnetworkinterfaceInterfaceNameFromIndex(index: i32) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkinterface_drop"]
        fn drop(iface: &mut QNetworkInterface);

        #[rust_name = "qnetworkinterface_init_default"]
        fn construct() -> QNetworkInterface;
        #[rust_name = "qnetworkinterface_clone"]
        fn construct(other: &QNetworkInterface) -> QNetworkInterface;
        #[rust_name = "qnetworkinterface_to_debug_qstring"]
        fn toDebugQString(value: &QNetworkInterface) -> QString;
    }
}

pub use ffi::{QNetworkInterfaceInterfaceFlag, QNetworkInterfaceInterfaceType};

impl QNetworkInterfaceInterfaceType {
    /// An alias for `Wifi`.
    #[allow(non_upper_case_globals)]
    pub const Ieee80211: Self = Self {
        repr: Self::Wifi.repr,
    };
}

/// [`QFlags`] of [`QNetworkInterfaceInterfaceFlag`].
pub type QNetworkInterfaceInterfaceFlags = QFlags<QNetworkInterfaceInterfaceFlag>;

unsafe_impl_qflag!(
    QNetworkInterfaceInterfaceFlag,
    "QNetworkInterfaceInterfaceFlags"
);

/// The `QNetworkInterface` class provides a listing of the host's IP addresses and network interfaces.
///
/// Qt Documentation: [QNetworkInterface](https://doc.qt.io/qt-6/qnetworkinterface.html#details)
#[repr(C)]
pub struct QNetworkInterface {
    _space: MaybeUninit<usize>,
}

impl Clone for QNetworkInterface {
    fn clone(&self) -> Self {
        ffi::qnetworkinterface_clone(self)
    }
}

impl Default for QNetworkInterface {
    /// Constructs an empty network interface object.
    fn default() -> Self {
        ffi::qnetworkinterface_init_default()
    }
}

impl Drop for QNetworkInterface {
    fn drop(&mut self) {
        ffi::qnetworkinterface_drop(self);
    }
}

impl fmt::Debug for QNetworkInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qnetworkinterface_to_debug_qstring(self).fmt(f)
    }
}

impl IsNonNull for QNetworkInterface {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl QNetworkInterface {
    /// This convenience function returns all IP addresses found on the host machine. It is equivalent to calling [`address_entries`](QNetworkInterface::address_entries) on all the objects returned by [`self.all_interfaces()`](QNetworkInterface::all_interfaces) that are in the [`QNetworkInterfaceInterfaceFlag::IsUp`] state to obtain lists of [`QNetworkAddressEntry`](crate::QNetworkAddressEntry) objects then calling [`QNetworkAddressEntry::ip`](crate::QNetworkAddressEntry::ip) on each of these.
    pub fn all_addresses() -> QList<QHostAddress> {
        ffi::qnetworkinterface_all_addresses()
    }

    /// Returns a listing of all the network interfaces found on the host machine. In case of failure it returns a list with zero elements.
    pub fn all_interfaces() -> QList<QNetworkInterface> {
        ffi::qnetworkinterface_all_interfaces()
    }

    /// Returns a `QNetworkInterface` object for the interface whose internal ID is index. Network interfaces have a unique identifier called the "interface index" to distinguish it from other interfaces on the system. Often, this value is assigned progressively and interfaces being removed and then added again get a different value every time.
    ///
    /// This index is also found in the IPv6 address' scope ID field.
    pub fn interface_from_index(index: i32) -> Option<Self> {
        ffi::qnetworkinterface_interface_from_index(index).nonnull()
    }

    /// Returns a `QNetworkInterface` object for the interface named `name`. If no such interface exists, this function returns `None`.
    ///
    /// The string `name` may be either an actual interface name (such as `"eth0"` or `"en1"`) or an interface index in string form (`"1"`, `"2"`, etc.).
    pub fn interface_from_name(name: &QString) -> Option<Self> {
        ffi::qnetworkinterface_interface_from_name(name).nonnull()
    }

    /// Returns the index of the interface whose name is `name` or `None` if there is no interface with that name.
    pub fn interface_index_from_name(name: &QString) -> Option<i32> {
        let index = ffi::qnetworkinterface_interface_index_from_name(name);
        if index == 0 {
            None
        } else {
            Some(index)
        }
    }

    /// Returns the name of the interface whose index is `index` or `None` if there is no interface with that index.
    pub fn interface_name_from_index(index: i32) -> Option<QString> {
        ffi::qnetworkinterface_interface_name_from_index(index).nonnull()
    }

    /// Returns the interface system index, if known. This is an integer assigned by the operating system to identify this interface and it generally doesn't change. It matches the scope ID field in IPv6 addresses.
    ///
    /// If the index isn't known, this function returns `None`.
    pub fn index(&self) -> Option<i32> {
        let index = self.index_or_zero();
        if index == 0 {
            None
        } else {
            Some(index)
        }
    }

    /// Returns the maximum transmission unit on this interface, if known, or `None` otherwise.
    ///
    /// The maximum transmission unit is the largest packet that may be sent on this interface without incurring link-level fragmentation. Applications may use this value to calculate the size of the payload that will fit an unfragmented UDP datagram. Remember to subtract the sizes of headers used in your communication over the interface, e.g. TCP (20 bytes) or UDP (12), IPv4 (20) or IPv6 (40, absent some form of header compression), when computing how big a payload you can transmit. Also note that the MTU along the full path (the Path MTU) to the destination may be smaller than the interface's MTU.
    pub fn maximum_transmission_unit(&self) -> Option<i32> {
        let max = self.maximum_transmission_unit_or_zero();
        if max == 0 {
            None
        } else {
            Some(max)
        }
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkInterface {
    type Id = type_id!("QNetworkInterface");
    type Kind = cxx::kind::Trivial;
}

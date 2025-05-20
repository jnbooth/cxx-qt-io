use cxx::{type_id, ExternType};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    /// This enum indicates whether a given host address is eligible to be published in the Domain Name System (DNS) or other similar name resolution mechanisms. In general, an address is suitable for publication if it is an address this machine will be reached at for an indeterminate amount of time, though it need not be permanent. For example, addresses obtained via DHCP are often eligible, but cryptographically-generated temporary IPv6 addresses are not.
    #[repr(i8)]
    #[derive(Debug)]
    enum QNetworkAddressEntryDnsEligibilityStatus {
        /// Qt and the operating system could not determine whether this address should be published or not. The application may need to apply further heuristics if it cannot find any eligible addresses.
        DnsEligibilityUnknown = -1,
        /// This address should not be published in DNS and should not be transmitted to other parties, except maybe as the source address of an outgoing packet.
        DnsIneligible = 0,
        /// This address is eligible for publication in DNS.
        DnsEligible = 1,
    }

    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-io/qdeadlinetimer.h");
        type QDeadlineTimer = crate::QDeadlineTimer;
        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddress = crate::QHostAddress;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkaddressentry.h");
        type QNetworkAddressEntryDnsEligibilityStatus;
    }

    unsafe extern "C++" {
        type QNetworkAddressEntry = super::QNetworkAddressEntry;

        /// Returns the broadcast address associated with the IPv4 address and netmask. It can usually be derived from those two by setting to 1 the bits of the IP address where the netmask contains a 0. (In other words, by bitwise-OR'ing the IP address with the inverse of the netmask)
        ///
        /// This member is always empty for IPv6 addresses, since the concept of broadcast has been abandoned in that system in favor of multicast. In particular, the group of hosts corresponding to all the nodes in the local network can be reached by the "all-nodes" special multicast group (address `FF02::1`).
        fn broadcast(&self) -> QHostAddress;

        /// Resets both the preferred and valid lifetimes for this address. After this call, [`is_lifetime_known`](QNetworkAddressEntry::is_lifetime_known) will return `false`.
        #[rust_name = "clear_address_lifetime"]
        fn clearAddressLifetime(&mut self);

        /// Returns whether this address is eligible for publication in the Domain Name System (DNS) or similar name resolution mechanisms.
        ///
        /// In general, an address is suitable for publication if it is an address this machine will be reached at for an indeterminate amount of time, though it need not be permanent. For example, addresses obtained via DHCP are often eligible, but cryptographically-generated temporary IPv6 addresses are not.
        ///
        /// On some systems, [`QNetworkInterface`](crate::QNetworkInterface) will need to heuristically determine which addresses are eligible.
        #[rust_name = "dns_eligibility"]
        fn dnsEligibility(&self) -> QNetworkAddressEntryDnsEligibilityStatus;

        /// This function returns one IPv4 or IPv6 address found, that was found in a network interface.
        fn ip(&self) -> QHostAddress;

        /// Returns `true` if the address lifetime is known, `false` if not. If the lifetime is not known, both [`preferred_lifetime`](QNetworkAddressEntry::preferred_lifetime) and [`validity_lifetime`](QNetworkAddressEntry::validity_lifetime) will return [`QDeadlineTimer::forever()`](crate::QDeadlineTimer::forever).
        #[rust_name = "is_lifetime_known"]
        fn isLifetimeKnown(&self) -> bool;

        /// Returns `true` if this address is permanent on this interface, `false` if it's temporary. A permanent address is one which has no expiration time and is often static (manually configured).
        ///
        /// If this information could not be determined, this function returns `true`.
        ///
        /// **Note:** Depending on the operating system and the networking configuration tool, it is possible for a temporary address to be interpreted as permanent, if the tool did not inform the details correctly to the operating system.
        #[rust_name = "is_permanent"]
        fn isPermanent(&self) -> bool;

        /// Returns `true` if this address is temporary on this interface, `false` if it's permanent.
        #[rust_name = "is_temporary"]
        fn isTemporary(&self) -> bool;

        /// Returns the netmask associated with the IP address. The netmask is expressed in the form of an IP address, such as `255.255.0.0`.
        ///
        /// For IPv6 addresses, the prefix length is converted to an address where the number of bits set to 1 is equal to the prefix length. For a prefix length of 64 bits (the most common value), the netmask will be expressed as a `QHostAddress` holding the address `FFFF:FFFF:FFFF:FFFF::`.
        fn netmask(&self) -> QHostAddress;

        /// Returns the deadline when this address becomes deprecated (no longer preferred), if known. If the address lifetime is not known (see [`is_lifetime_known`](QNetworkAddressEntry::is_lifetime_known)), this function always returns [`QDeadlineTimer::forever()`](crate::QDeadlineTimer::forever).
        ///
        /// While an address is preferred, it may be used by the operating system as the source address for new, outgoing packets. After it becomes deprecated, it will remain valid for incoming packets for a while longer until finally removed (see [`validity_lifetime`](QNetworkAddressEntry::validity_lifetime)).
        #[rust_name = "preferred_lifetime"]
        fn preferredLifetime(&self) -> QDeadlineTimer;

        #[doc(hidden)]
        #[rust_name = "prefix_length_or_negative"]
        fn prefixLength(&self) -> i32;

        /// Sets the broadcast IP address of this `QNetworkAddressEntry` object to `new_broadcast`.
        #[rust_name = "set_broadcast"]
        fn setBroadcast(&mut self, new_broadcast: &QHostAddress);

        /// Sets the DNS eligibility flag for this address to `status`.
        #[rust_name = "set_dns_eligibility"]
        fn setDnsEligibility(&mut self, status: QNetworkAddressEntryDnsEligibilityStatus);

        /// Sets the IP address the `QNetworkAddressEntry` object contains to `new_ip`.
        #[rust_name = "set_ip"]
        fn setIp(&mut self, new_ip: &QHostAddress);

        /// Sets the netmask that this `QNetworkAddressEntry` object contains to `new_netmask`. Setting the netmask also sets the prefix length to match the new netmask.
        #[rust_name = "set_netmask"]
        fn setNetmask(&mut self, new_netmask: &QHostAddress);

        #[doc(hidden)]
        #[rust_name = "set_prefix_length_or_negative"]
        fn setPrefixLength(&mut self, length: i32);

        /// Returns the deadline when this address becomes invalid and will be removed from the networking stack, if known. If the address lifetime is not known (see [`is_lifetime_known`](QNetworkAddressEntry::is_lifetime_known)), this function always returns [`QDeadlineTimer::forever()`].
        ///
        /// While an address is valid, it will be accepted by the operating system as a valid destination address for this machine. Whether it is used as a source address for new, outgoing packets is controlled by, among other rules, the preferred lifetime (see [`preferred_lifetime`](QNetworkAddressEntry::preferred_lifetime)).
        #[rust_name = "validity_lifetime"]
        fn validityLifetime(&self) -> QDeadlineTimer;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkaddressentry_drop"]
        fn drop(address: &mut QNetworkAddressEntry);

        #[rust_name = "qnetworkaddressentry_init_default"]
        fn construct() -> QNetworkAddressEntry;
        #[rust_name = "qnetworkaddressentry_clone"]
        fn construct(other: &QNetworkAddressEntry) -> QNetworkAddressEntry;

        #[rust_name = "qnetworkaddressentry_eq"]
        fn operatorEq(a: &QNetworkAddressEntry, b: &QNetworkAddressEntry) -> bool;

        #[rust_name = "qnetworkaddressentry_to_debug_qstring"]
        fn toDebugQString(value: &QNetworkAddressEntry) -> QString;
    }
}

pub use ffi::QNetworkAddressEntryDnsEligibilityStatus;

/// The `QNetworkAddressEntry` class stores one IP address supported by a network interface, along with its associated netmask and broadcast address.
///
/// Qt Documentation: [QNetworkAddressEntry](https://doc.qt.io/qt-6/qnetworkaddressentry.html#details)
#[repr(C)]
pub struct QNetworkAddressEntry {
    _space: MaybeUninit<usize>,
}

impl Clone for QNetworkAddressEntry {
    fn clone(&self) -> Self {
        ffi::qnetworkaddressentry_clone(self)
    }
}

impl Default for QNetworkAddressEntry {
    fn default() -> Self {
        ffi::qnetworkaddressentry_init_default()
    }
}

impl Drop for QNetworkAddressEntry {
    fn drop(&mut self) {
        ffi::qnetworkaddressentry_drop(self);
    }
}

impl PartialEq for QNetworkAddressEntry {
    fn eq(&self, other: &Self) -> bool {
        ffi::qnetworkaddressentry_eq(self, other)
    }
}

impl Eq for QNetworkAddressEntry {}

impl fmt::Debug for QNetworkAddressEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qnetworkaddressentry_to_debug_qstring(self).fmt(f)
    }
}

impl QNetworkAddressEntry {
    /// Returns the prefix length of this IP address. The prefix length matches the number of bits set to 1 in the netmask (see [`netmask`](QNetworkAddressEntry::netmask)). For IPv4 addresses, the value is between 0 and 32. For IPv6 addresses, it's contained between 0 and 128 and is the preferred form of representing addresses.
    ///
    /// This function returns `None` if the prefix length could not be determined (i.e., [`self.netmask()`](QNetworkAddressEntry::netmask) returns a null [`QHostAddress`](crate::QHostAddress)).
    pub fn prefix_length(&self) -> Option<i32> {
        let prefix_length = self.prefix_length_or_negative();
        if prefix_length == -1 {
            None
        } else {
            Some(prefix_length)
        }
    }

    /// Sets the prefix length of this IP address to `length`. The value of `length` must be valid for this type of IP address: between 0 and 32 for IPv4 addresses, between 0 and 128 for IPv6 addresses. Setting to any invalid value is equivalent to setting to `None`, which means "no prefix length".
    ///
    /// Setting the prefix length also sets the netmask (see [`netmask`](QNetworkAddressEntry::netmask)).
    pub fn set_prefix_length(&mut self, length: Option<i32>) {
        self.set_prefix_length_or_negative(length.unwrap_or(-1));
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkAddressEntry {
    type Id = type_id!("QNetworkAddressEntry");
    type Kind = cxx::kind::Trivial;
}

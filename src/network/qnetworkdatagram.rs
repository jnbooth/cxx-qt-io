use cxx::{type_id, ExternType};
use cxx_qt_lib::QByteArray;
use std::mem::MaybeUninit;

use crate::util::IsNonNull;
use crate::QHostAddress;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;

        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddress = crate::QHostAddress;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkdatagram.h");
    }

    unsafe extern "C++" {
        type QNetworkDatagram = super::QNetworkDatagram;

        /// Clears the payload data and metadata in this `QNetworkDatagram` object, resetting them to their default values.
        fn clear(&mut self);

        /// Returns the data payload of this datagram. For a datagram received from the network, it contains the payload of the datagram. For an outgoing datagram, it is the datagram to be sent.
        ///
        /// Note that datagrams can be transmitted with no data, so the returned `QByteArray` may be empty.
        fn data(&self) -> QByteArray;

        #[doc(hidden)]
        #[rust_name = "destination_address_or_null"]
        fn destinationAddress(&self) -> QHostAddress;

        #[doc(hidden)]
        #[rust_name = "destination_port_or_negative"]
        fn destinationPort(&self) -> i32;

        #[doc(hidden)]
        #[rust_name = "hop_limit_or_negative"]
        fn hopLimit(&self) -> i32;

        #[doc(hidden)]
        #[rust_name = "interface_index_or_zero"]
        fn interfaceIndex(&self) -> u32;

        /// Returns true if this `QNetworkDatagram` object is null. This function is the opposite of `is_valid()`.
        #[rust_name = "is_null"]
        fn isNull(&self) -> bool;

        /// Returns `true` if this QNetworkDatagram object is valid. A valid `QNetworkDatagram` object contains at least one sender or receiver address. Valid datagrams can contain empty payloads.
        #[rust_name = "is_valid"]
        fn isValid(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "sender_address_or_null"]
        fn senderAddress(&self) -> QHostAddress;

        #[doc(hidden)]
        #[rust_name = "sender_port_or_negative"]
        fn senderPort(&self) -> i32;

        /// Sets the data payload of this datagram to data. It is usually not necessary to call this function on received datagrams. For outgoing datagrams, this function sets the data to be sent on the network.
        ///
        /// Since datagrams can empty, an empty `QByteArray` is a valid value for data.
        #[rust_name = "set_data"]
        fn setData(&mut self, data: &QByteArray);

        /// Sets the destination address associated with this datagram to be the address address and port number port. The destination address and port numbers are usually set by `QUdpSocket` upon reception, so there's no need to call this function on a received datagram.
        ///
        /// For outgoing datagrams, this function can be used to set the address the datagram should be sent to. It can be the unicast address used to communicate with the peer or a broadcast or multicast address to send to a group of devices.
        #[rust_name = "set_destination"]
        fn setDestination(&mut self, address: &QHostAddress, port: u16);

        #[doc(hidden)]
        #[rust_name = "set_hop_limit_or_negative"]
        fn setHopLimit(&mut self, count: i32);

        #[doc(hidden)]
        #[rust_name = "set_interface_index_or_zero"]
        fn setInterfaceIndex(&mut self, index: u32);

        /// Sets the sender address associated with this datagram to be the address `address` and port number `port`. The sender address and port numbers are usually set by `QUdpSocket` upon reception, so there's no need to call this function on a received datagram.
        ///
        /// For outgoing datagrams, this function can be used to set the address the datagram should carry. The address `address` must usually be one of the local addresses assigned to this machine, which can be obtained using `QNetworkInterface`. If left unset, the operating system will choose the most appropriate address to use given the destination in question.
        ///
        /// The port number port must be the port number associated with the socket, if there is one. The value of 0 can be used to indicate that the operating system should choose the port number.
        #[rust_name = "set_sender"]
        fn setSender(&mut self, address: &QHostAddress, port: u16);
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qnetworkdatagram_make_reply"]
        fn qnetworkdatagramMakeReply(
            datagram: &QNetworkDatagram,
            payload: &QByteArray,
        ) -> QNetworkDatagram;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkdatagram_drop"]
        fn drop(datagram: &mut QNetworkDatagram);

        #[rust_name = "qnetworkdatagram_init_default"]
        fn construct() -> QNetworkDatagram;
        #[rust_name = "qnetworkdatagram_init_data"]
        fn construct(data: &QByteArray) -> QNetworkDatagram;
        #[rust_name = "qnetworkdatagram_init_data_addr"]
        fn construct(
            data: &QByteArray,
            destination_address: &QHostAddress,
            port: u16,
        ) -> QNetworkDatagram;
        #[rust_name = "qnetworkdatagram_clone"]
        fn construct(other: &QNetworkDatagram) -> QNetworkDatagram;
    }
}

/// The `QNetworkDatagram` class provides the data and metadata of a UDP datagram.
///
/// Qt Documentation: [QNetworkDatagram](https://doc.qt.io/qt-6/qnetworkdatagram.html#details)
#[repr(C)]
pub struct QNetworkDatagram {
    _space: MaybeUninit<usize>,
}

impl Clone for QNetworkDatagram {
    fn clone(&self) -> Self {
        ffi::qnetworkdatagram_clone(self)
    }
}

impl Default for QNetworkDatagram {
    /// Creates a `QNetworkDatagram` object with no payload data and undefined destination address.
    ///
    /// The payload can be modified by using `set_data()` and the destination address can be set with `set_destination()`.
    ///
    /// If the destination address is left undefined, `QUdpSocket::write_datagram()` will attempt to send the datagram to the address last associated with, by using `QUdpSocket::connect_to_host()`.
    fn default() -> Self {
        ffi::qnetworkdatagram_init_default()
    }
}

impl Drop for QNetworkDatagram {
    fn drop(&mut self) {
        ffi::qnetworkdatagram_drop(self);
    }
}

impl IsNonNull for QNetworkDatagram {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl QNetworkDatagram {
    /// Creates a `QNetworkDatagram` object and sets `data` as the payload data, along with `destination_address` and `port` as the destination address of the datagram.
    pub fn new(data: &QByteArray, destination_address: &QHostAddress, port: u16) -> Self {
        ffi::qnetworkdatagram_init_data_addr(data, destination_address, port)
    }

    /// Returns the destination address associated with this datagram. For a datagram received from the network, it is the address the peer node sent the datagram to, which can either be a local address of this machine or a multicast or broadcast address. For an outgoing datagrams, it is the address the datagram should be sent to.
    ///
    /// If no destination address was set on this datagram, this function returns `None`.
    pub fn destination_address(&self) -> Option<QHostAddress> {
        self.destination_address_or_null().nonnull()
    }

    /// Returns the port number of the destination associated with this datagram. For a datagram received from the network, it is the local port number that the peer node sent the datagram to. For an outgoing datagram, it is the peer port the datagram should be sent to.
    ///
    /// If no destination address was associated with this datagram, this function returns `None`.
    pub fn destination_port(&self) -> Option<u16> {
        self.destination_port_or_negative().try_into().ok()
    }

    /// Returns the hop count limit associated with this datagram. The hop count limit is the number of nodes that are allowed to forward the IP packet before it expires and an error is sent back to the sender of the datagram. In IPv4, this value is usually known as "time to live" (TTL).
    ///
    /// If this datagram was received from the network, this is the remaining hop count of the datagram after reception and was decremented by 1 by each node that forwarded the packet. A value of `None` indicates that the hop limit could not be obtained.
    ///
    /// If this is an outgoing datagram, this is the value to be set in the IP header upon sending. A value of `None` indicates the operating system should choose the value.
    pub fn hop_limit(&self) -> Option<i32> {
        let limit = self.hop_limit_or_negative();
        if limit == -1 {
            None
        } else {
            Some(limit)
        }
    }

    /// Returns the interface index this datagram is associated with. The interface index is a positive number that uniquely identifies the network interface in the operating system. This number matches the value returned by `QNetworkInterface::index()` for the interface.
    ///
    /// If this datagram was received from the network, this is the index of the interface that the packet was received from. If this is an outgoing datagram, this is the index of the interface that the datagram should be sent on.
    ///
    /// A value of `None` indicates that the interface index is unknown.
    pub fn interface_index(&self) -> Option<u32> {
        let index = self.interface_index_or_zero();
        if index == 0 {
            None
        } else {
            Some(index)
        }
    }

    /// Creates a new `QNetworkDatagram` representing a reply to this incoming datagram and sets the payload data to payload. This function is a very convenient way of responding to a datagram back to the original sender.
    ///
    /// This function is especially convenient since it will automatically copy parameters from this datagram to the new datagram as appropriate:
    ///
    /// * this datagram's sender address and port are copied to the new datagram's destination address and port;
    /// * this datagram's interface index, if any, is copied to the new datagram's interface index;
    /// * this datagram's destination address and port are copied to the new datagram's sender address and port only if the address is IPv6 global (non-multicast) address;
    /// * the hop count limit on the new datagram is reset to the default (-1);
    ///
    /// This datagram's destination address is not copied if it is an IPv4 address because it is not possible to tell an IPv4 broadcast address apart from a regular IPv4 address without an exhaustive search of all addresses assigned to this machine. Attempting to send a datagram with the sender address equal to the broadcast address is likely to fail. However, this should not affect the communication as network interfaces with multiple IPv4 addresses are uncommon, so the address the operating system will select will likely be one the peer will understand.
    #[must_use = "returns a new datagram without modifying the original"]
    pub fn make_reply(&self, payload: &QByteArray) -> Self {
        ffi::qnetworkdatagram_make_reply(self, payload)
    }

    /// Returns the sender address associated with this datagram. For a datagram received from the network, it is the address of the peer node that sent the datagram. For an outgoing datagrams, it is the local address to be used when sending.
    ///
    /// If no sender address was set on this datagram, this function returns `None`.
    pub fn sender_address(&self) -> Option<QHostAddress> {
        self.sender_address_or_null().nonnull()
    }

    /// Returns the port number of the destination associated with this datagram. For a datagram received from the network, it is the port number that the peer node sent the datagram from. For an outgoing datagram, it is the local port the datagram should be sent from.
    ///
    /// If no sender address was associated with this datagram, this function returns `None`.
    pub fn sender_port(&self) -> Option<u16> {
        self.sender_port_or_negative().try_into().ok()
    }

    /// Sets the hop count limit associated with this datagram to count. The hop count limit is the number of nodes that are allowed to forward the IP packet before it expires and an error is sent back to the sender of the datagram. In IPv4, this value is usually known as "time to live" (TTL).
    ///
    /// It is usually not necessary to call this function on datagrams received from the network.
    ///
    /// If this is an outgoing packet, this is the value to be set in the IP header upon sending. The valid range for the value is 1 to 255. This function also accepts a value of `None` to indicate that the operating system should choose the value.
    pub fn set_hop_limit(&mut self, count: Option<i32>) {
        self.set_hop_limit_or_negative(count.unwrap_or(-1));
    }

    /// Sets the interface index this datagram is associated with to index. The interface index is a positive number that uniquely identifies the network interface in the operating system. This number matches the value returned by `QNetworkInterface::index()` for the interface.
    ///
    /// It is usually not necessary to call this function on datagrams received from the network.
    ///
    /// If this is an outgoing packet, this is the index of the interface the datagram should be sent on. A value of `None` indicates that the operating system should choose the interface based on other factors.
    ///
    /// Note that the interface index can also be set with `QHostAddress::set_scope_id()` for IPv6 destination addresses and then with `set_destination()`. If the scope ID set in the destination address and index are different and neither is zero, it is undefined which interface the operating system will send the datagram on.
    pub fn set_interface_index(&mut self, index: Option<u32>) {
        self.set_interface_index_or_zero(index.unwrap_or(0));
    }
}

impl From<&QByteArray> for QNetworkDatagram {
    /// Creates a `QNetworkDatagram` object with undefined destination address.
    ///
    /// The destination address can be set with `set_destination()`.
    ///
    /// If the destination address is left undefined, `QUdpSocket::write_datagram()` will attempt to send the datagram to the address last associated with, by using `QUdpSocket::connect_to_host()`.
    fn from(data: &QByteArray) -> Self {
        ffi::qnetworkdatagram_init_data(data)
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkDatagram {
    type Id = type_id!("QNetworkDatagram");
    type Kind = cxx::kind::Trivial;
}

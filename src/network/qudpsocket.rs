use crate::qio::{QIOExt, QIO};
use crate::util::IsNonNull;
use crate::{QAbstractSocket, QHostAddress, QIODevice, QNetworkDatagram, QNetworkInterface};
use cxx_qt::Upcast;
use std::ffi::c_char;
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::pin::Pin;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qhostaddress.h");
        type QHostAddress = crate::QHostAddress;
        include!("cxx-qt-io/qnetworkdatagram.h");
        type QNetworkDatagram = crate::QNetworkDatagram;
        include!("cxx-qt-io/qnetworkinterface.h");
        type QNetworkInterface = crate::QNetworkInterface;
    }

    unsafe extern "C++Qt" {
        include!(<QtNetwork/QUdpSocket>);
        type QIODevice = crate::QIODevice;
        type QAbstractSocket = crate::QAbstractSocket;

        /// The `QUdpSocket` class provides a UDP socket.
        ///
        /// Qt Documentation: [QUdpSocket](https://doc.qt.io/qt-6/qudpsocket.html#details)
        #[qobject]
        #[base = QAbstractSocket]
        type QUdpSocket;

        /// Returns `true` if at least one datagram is waiting to be read; otherwise returns `false`.
        #[rust_name = "has_pending_datagrams"]
        fn hasPendingDatagrams(self: &QUdpSocket) -> bool;

        #[rust_name = "join_multicast_group_on_default"]
        pub(self) fn joinMulticastGroup(
            self: Pin<&mut QUdpSocket>,
            group_address: &QHostAddress,
        ) -> bool;
        #[rust_name = "join_multicast_group_on_interface"]
        pub(self) fn joinMulticastGroup(
            self: Pin<&mut QUdpSocket>,
            group_address: &QHostAddress,
            iface: &QNetworkInterface,
        ) -> bool;

        #[rust_name = "leave_multicast_group_on_default"]
        pub(self) fn leaveMulticastGroup(
            self: Pin<&mut QUdpSocket>,
            group_address: &QHostAddress,
        ) -> bool;
        #[rust_name = "leave_multicast_group_on_interface"]
        pub(self) fn leaveMulticastGroup(
            self: Pin<&mut QUdpSocket>,
            group_address: &QHostAddress,
            iface: &QNetworkInterface,
        ) -> bool;

        #[rust_name = "multicast_interface_or_invalid"]
        pub(self) fn multicastInterface(self: &QUdpSocket) -> QNetworkInterface;

        #[rust_name = "pending_datagram_size_or_negative"]
        pub(self) fn pendingDatagramSize(self: &QUdpSocket) -> i64;

        ///Receives a datagram no larger than `max_size` bytes and stores it in `data`. The sender's host address and port is stored in `address` and `port` (unless the pointers are null).
        ///
        /// Returns the size of the datagram on success; otherwise returns -1.
        ///
        /// If `max_size` is too small, the rest of the datagram will be lost. To avoid loss of data, call [`pending_datagram_size`](QUdpSocket::pending_datagram_size) to determine the size of the pending datagram before attempting to read it. If `max_size` is 0, the datagram will be discarded.
        ///
        /// # Safety
        ///
        /// `data` must be valid for reads for `max_size` many bytes.
        /// `address` and `port` must be valid or null.
        #[rust_name = "read_datagram_unsafe"]
        unsafe fn readDatagram(
            self: Pin<&mut QUdpSocket>,
            data: *mut c_char,
            max_size: i64,
            address: *mut QHostAddress,
            port: *mut u16,
        ) -> i64;

        #[rust_name = "receive_datagram_or_invalid"]
        pub(self) fn receiveDatagram(self: Pin<&mut QUdpSocket>, max_size: i64)
            -> QNetworkDatagram;

        /// Sets the outgoing interface for multicast datagrams to the interface `iface`. This corresponds to the `IP_MULTICAST_IF` socket option for IPv4 sockets and the `IPV6_MULTICAST_IF` socket option for IPv6 sockets. The socket must be in [`QAbstractSocketSocketState::BoundState`](crate::QAbstractSocketSocketState::BoundState), otherwise this function does nothing.
        #[rust_name = "set_multicast_interface"]
        pub fn setMulticastInterface(self: Pin<&mut QUdpSocket>, iface: &QNetworkInterface);

        /// Sends the datagram at data of size size to the host address address at port port. Returns the number of bytes sent on success; otherwise returns -1.
        ///
        /// Datagrams are always written as one block. The maximum size of a datagram is highly platform-dependent, but can be as low as 8192 bytes. If the datagram is too large, this function will return -1 and [`error`](QAbstractSocket::error) will return [`QAbstractSocketSocketError::DatagramTooLargeError`](crate::QAbstractSocketSocketError::DatagramTooLarge).
        ///
        /// Sending datagrams larger than 512 bytes is in general disadvised, as even if they are sent successfully, they are likely to be fragmented by the IP layer before arriving at their final destination.
        ///
        /// **Warning:** Calling this function on a connected UDP socket may result in an error and no packet being sent. If you are using a connected socket, use [`write`](QIODevice::write) to send datagrams.
        ///
        /// # Safety
        ///
        /// `data` must be valid for reads for `size` many bytes.
        #[rust_name = "write_datagram_unsafe"]
        unsafe fn writeDatagram(
            self: Pin<&mut QUdpSocket>,
            data: *const c_char,
            size: i64,
            address: &QHostAddress,
            port: u16,
        ) -> i64;

        /// Sends the datagram `datagram` to the host address and port numbers contained in `datagram`, using the network interface and hop count limits also set there. If the destination address and port numbers are unset, this function will send to the address that was passed to [`connect_to_host`](QAbstractSocket::connect_to_host).
        ///
        /// If the destination address is IPv6 with a non-empty scope id but differs from the interface index in datagram, it is undefined which interface the operating system will choose to send on.
        ///
        /// The function returns the number of bytes sent if it succeeded or -1 if it encountered an error.
        #[rust_name = "send_datagram"]
        fn writeDatagram(self: Pin<&mut QUdpSocket>, datagram: &QNetworkDatagram) -> i64;

    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        include!("cxx-qt-io/common.h");

        #[rust_name = "upcast_qudpsocket_qiodevice"]
        unsafe fn upcast(socket: *const QUdpSocket) -> *const QIODevice;
        #[rust_name = "downcast_qiodevice_qudpsocket"]
        unsafe fn downcast(socket: *const QIODevice) -> *const QUdpSocket;
    }
}

pub use ffi::QUdpSocket;

#[allow(clippy::cast_possible_wrap)]
impl QUdpSocket {
    /// Joins the multicast group specified by `group_address` on a specified network `interface`, or the default interface chosen by the operating system if `interface` is `None`. The socket must be in [`QAbstractSocketSocketState::BoundState`](crate::QAbstractSocketSocketState::BoundState), otherwise an error occurs.
    ///
    /// Note that if you are attempting to join an IPv4 group, your socket must not be bound using IPv6 (or in dual mode, using [`QHostAddressSpecialAddress::Any`](crate::QHostAddressSpecialAddress::Any)). You must use [`QHostAddressSpecialAddress::Any`](crate::QHostAddressSpecialAddress::Any) instead.
    ///
    /// This function returns `true` if successful; otherwise it returns `false` and sets the socket error accordingly.
    ///
    /// **Note:** Joining IPv6 multicast groups without an interface selection (using `None`) is not supported in all operating systems.
    pub fn join_multicast_group(
        self: Pin<&mut Self>,
        group_address: &QHostAddress,
        interface: Option<&QNetworkInterface>,
    ) -> bool {
        match interface {
            Some(interface) => self.join_multicast_group_on_interface(group_address, interface),
            None => self.join_multicast_group_on_default(group_address),
        }
    }

    /// Leaves the multicast group specified by `group_address` on a specified network `interface`, or the default interface chosen by the operating system if `interface` is `None`, The socket must be in [`QAbstractSocketSocketState::BoundState`](crate::QAbstractSocketSocketState::BoundState), otherwise an error occurs.
    ///
    /// This function returns `true` if successful; otherwise it returns `false` and sets the socket error accordingly.
    ///
    /// **Note:** This function should be called with the same arguments as were passed to [`join_multicast_group`](QUdpSocket::join_multicast_group).
    pub fn leave_multicast_group(
        self: Pin<&mut Self>,
        group_address: &QHostAddress,
        interface: Option<&QNetworkInterface>,
    ) -> bool {
        match interface {
            Some(interface) => self.leave_multicast_group_on_interface(group_address, interface),
            None => self.leave_multicast_group_on_default(group_address),
        }
    }

    /// Returns the interface for the outgoing interface for multicast datagrams. This corresponds to the `IP_MULTICAST_IF` socket option for IPv4 sockets and the `IPV6_MULTICAST_IF` socket option for IPv6 sockets. If no interface has been previously set, this function returns an invalid `QNetworkInterface`. The socket must be in [`QAbstractSocketSocketState::BoundState`](crate::QAbstractSocketSocketState::BoundState), otherwise `None` is returned.
    pub fn multicast_interface(&self) -> Option<QNetworkInterface> {
        self.multicast_interface_or_invalid().nonnull()
    }

    /// Returns the size of the first pending UDP datagram. If there is no datagram available, this function returns `None`.
    pub fn pending_datagram_size(&self) -> Option<i64> {
        let size = self.pending_datagram_size_or_negative();
        if size == -1 {
            None
        } else {
            Some(size)
        }
    }

    ///Receives a datagram stores it in `data`. The sender's host address and port is stored in `address` and `port` (unless the pointers are null).
    ///
    /// On success, returns `Some((n, address, port))`, where `n` is the size of the datagram received, `address` is the sender's host address, and `port` is the sender's port. On failure, returns `None`.
    ///
    /// If `data.len()` is too small, the rest of the datagram will be lost. To avoid loss of data, call `pending_datagram_size()` to determine the size of the pending datagram before attempting to read it. If `data.len()` is 0, the datagram will be discarded.
    pub fn read_datagram(
        self: Pin<&mut Self>,
        data: &mut [c_char],
    ) -> Option<(i64, QHostAddress, u16)> {
        let mut address: MaybeUninit<QHostAddress> = MaybeUninit::uninit();
        let mut port = 0;
        unsafe {
            // SAFETY: `data.as_mut_ptr()` is valid up to `data.len()`, and `address.as_mut_ptr()`
            // and `&mut port` are valid.
            let n = self.read_datagram_unsafe(
                data.as_mut_ptr(),
                data.len() as i64,
                address.as_mut_ptr(),
                &mut port,
            );
            if n == -1 {
                return None;
            }
            Some((n, address.assume_init(), port))
        }
    }

    /// Receives a datagram no larger than `max_size` bytes and returns it in the `QNetworkDatagram` object, along with the sender's host address and port. If possible, this function will also try to determine the datagram's destination address, port, and the number of hop counts at reception time.
    ///
    /// On failure, returns `None`.
    ///
    /// If `max_size` is too small, the rest of the datagram will be lost. If `max_size` is `Some(0)`, the datagram will be discarded. If `max_size` is `None` (the default), this function will attempt to read the entire datagram.
    pub fn receive_datagram(
        self: Pin<&mut Self>,
        max_size: Option<i64>,
    ) -> Option<QNetworkDatagram> {
        self.receive_datagram_or_invalid(max_size.unwrap_or(-1))
            .nonnull()
    }

    /// Sends the datagram at data of size size to the host address address at port port. Returns the number of bytes sent on success; otherwise returns -1.
    ///
    /// Datagrams are always written as one block. The maximum size of a datagram is highly platform-dependent, but can be as low as 8192 bytes. If the datagram is too large, this function will return `None` and [`error`](QAbstractSocket::error) will return `DatagramTooLargeError`.
    ///
    /// Sending datagrams larger than 512 bytes is in general disadvised, as even if they are sent successfully, they are likely to be fragmented by the IP layer before arriving at their final destination.
    ///
    /// **Warning:** Calling this function on a connected UDP socket may result in an error and no packet being sent. If you are using a connected socket, use [`write`](QIODevice::write) to send datagrams.
    pub fn write_datagram(
        self: Pin<&mut Self>,
        data: &[c_char],
        address: &QHostAddress,
        port: u16,
    ) -> i64 {
        // SAFETY: `data.as_ptr()` is valid up to `data.len()`.
        unsafe { self.write_datagram_unsafe(data.as_ptr(), data.len() as i64, address, port) }
    }

    /// Casts this object to `QIODevice`.
    pub fn as_io_device(&self) -> &QIODevice {
        self.upcast()
    }

    /// Mutably casts this object to `QIODevice`.
    pub fn as_io_device_mut<'a>(self: &'a mut Pin<&mut Self>) -> Pin<&'a mut QIODevice> {
        self.as_mut().upcast_pin()
    }

    /// Casts this object to `QAbstractSocket`.
    pub fn as_abstract_socket(&self) -> &QAbstractSocket {
        self.upcast()
    }

    /// Mutably casts this object to `QAbstractSocket`.
    pub fn as_abstract_socket_mut<'a>(
        self: &'a mut Pin<&mut Self>,
    ) -> Pin<&'a mut QAbstractSocket> {
        self.as_mut().upcast_pin()
    }
}

impl Deref for QUdpSocket {
    type Target = QAbstractSocket;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

impl Upcast<QIODevice> for QUdpSocket {
    unsafe fn upcast_ptr(this: *const Self) -> *const QIODevice {
        ffi::upcast_qudpsocket_qiodevice(this)
    }

    unsafe fn from_base_ptr(base: *const QIODevice) -> *const Self {
        ffi::downcast_qiodevice_qudpsocket(base)
    }
}

impl AsRef<QIODevice> for QUdpSocket {
    fn as_ref(&self) -> &QIODevice {
        self.upcast()
    }
}

impl AsRef<QAbstractSocket> for QUdpSocket {
    fn as_ref(&self) -> &QAbstractSocket {
        self.upcast()
    }
}

impl QIO for QUdpSocket {
    fn flush(mut self: Pin<&mut Self>) -> bool {
        self.as_abstract_socket_mut().flush()
    }

    fn get_error_kind(&self) -> io::ErrorKind {
        self.as_abstract_socket().get_error_kind()
    }
}

impl Read for Pin<&mut QUdpSocket> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QIOExt::read(self.as_mut(), buf)
    }
}

impl Write for Pin<&mut QUdpSocket> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        QIOExt::write(self.as_mut(), buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        QIOExt::flush(self.as_mut())
    }
}

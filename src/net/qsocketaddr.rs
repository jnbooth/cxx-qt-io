use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use cxx_qt_lib::QString;

use crate::{QAbstractSocketNetworkLayerProtocol, QHostAddress, QHostAddressSpecialAddress};

/// Anything that implements `Into<QSocketAddr>` can be used with [`QAbstractSocket::connect_to_host`](crate::QAbstractSocket::connect_to_host).
///
/// This struct is analogous to Rust's [`SocketAddr`].
#[derive(Debug, PartialEq, Eq)]
pub enum QSocketAddr {
    Address(QHostAddress, u16),
    Name(QString, u16, QAbstractSocketNetworkLayerProtocol),
}

impl From<(QHostAddress, u16)> for QSocketAddr {
    fn from((addr, port): (QHostAddress, u16)) -> Self {
        Self::Address(addr, port)
    }
}

macro_rules! impl_from_address {
    ($t:ty) => {
        impl From<($t, u16)> for QSocketAddr {
            fn from((addr, port): ($t, u16)) -> Self {
                Self::Address(addr.into(), port)
            }
        }
    };
}

impl_from_address!(QHostAddressSpecialAddress);
impl_from_address!(Ipv6Addr);
impl_from_address!(IpAddr);
impl_from_address!(Ipv4Addr);

impl From<SocketAddrV4> for QSocketAddr {
    fn from(value: SocketAddrV4) -> Self {
        Self::Address(QHostAddress::from(*value.ip()), value.port())
    }
}

impl From<SocketAddrV6> for QSocketAddr {
    fn from(value: SocketAddrV6) -> Self {
        Self::Address(QHostAddress::from(*value.ip()), value.port())
    }
}

impl From<SocketAddr> for QSocketAddr {
    fn from(value: SocketAddr) -> Self {
        match value {
            SocketAddr::V4(addr) => addr.into(),
            SocketAddr::V6(addr) => addr.into(),
        }
    }
}

impl From<(QString, u16, QAbstractSocketNetworkLayerProtocol)> for QSocketAddr {
    fn from((name, port, protocol): (QString, u16, QAbstractSocketNetworkLayerProtocol)) -> Self {
        Self::Name(name, port, protocol)
    }
}

impl From<(&QString, u16, QAbstractSocketNetworkLayerProtocol)> for QSocketAddr {
    fn from((name, port, protocol): (&QString, u16, QAbstractSocketNetworkLayerProtocol)) -> Self {
        Self::Name(name.clone(), port, protocol)
    }
}

impl From<(&str, u16, QAbstractSocketNetworkLayerProtocol)> for QSocketAddr {
    fn from((name, port, protocol): (&str, u16, QAbstractSocketNetworkLayerProtocol)) -> Self {
        Self::Name(QString::from(name), port, protocol)
    }
}

impl From<(&String, u16, QAbstractSocketNetworkLayerProtocol)> for QSocketAddr {
    fn from((name, port, protocol): (&String, u16, QAbstractSocketNetworkLayerProtocol)) -> Self {
        Self::Name(QString::from(name), port, protocol)
    }
}

impl From<(String, u16, QAbstractSocketNetworkLayerProtocol)> for QSocketAddr {
    fn from((name, port, protocol): (String, u16, QAbstractSocketNetworkLayerProtocol)) -> Self {
        Self::Name(QString::from(&name), port, protocol)
    }
}

impl<S> From<(S, u16)> for QSocketAddr
where
    QSocketAddr: From<(S, u16, QAbstractSocketNetworkLayerProtocol)>,
{
    /// Connect using [`QAbstractSocketNetworkLayerProtocol::AnyIPProtocol`].
    fn from((name, port): (S, u16)) -> Self {
        Self::from((
            name,
            port,
            QAbstractSocketNetworkLayerProtocol::AnyIPProtocol,
        ))
    }
}

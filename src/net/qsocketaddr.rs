use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use cxx_qt_lib::QString;

use crate::{QAbstractSocketNetworkLayerProtocol, QHostAddress, QHostAddressSpecialAddress};

/// Anything that implements `Into<QSocketAddr>` can be used with [`QAbstractSocket::connect_to_host`](crate::QAbstractSocket::connect_to_host).
///
/// This struct is analogous to Rust's [`SocketAddr`].
#[derive(Debug, PartialEq, Eq)]
pub struct QSocketAddr {
    pub name: QString,
    pub port: u16,
    pub protocol: QAbstractSocketNetworkLayerProtocol,
}

impl From<(QHostAddress, u16)> for QSocketAddr {
    fn from((addr, port): (QHostAddress, u16)) -> Self {
        Self {
            name: addr.to_qstring(),
            port,
            protocol: QAbstractSocketNetworkLayerProtocol::AnyIPProtocol,
        }
    }
}

macro_rules! impl_from_address {
    ($t:ty, $i:ident) => {
        impl From<($t, u16)> for QSocketAddr {
            fn from((addr, port): ($t, u16)) -> Self {
                Self {
                    name: QHostAddress::from(addr).to_qstring(),
                    port,
                    protocol: QAbstractSocketNetworkLayerProtocol::$i,
                }
            }
        }
    };
}

impl_from_address!(QHostAddressSpecialAddress, AnyIPProtocol);
impl_from_address!(Ipv6Addr, IPv6Protocol);
impl_from_address!(Ipv4Addr, IPv4Protocol);

impl From<(IpAddr, u16)> for QSocketAddr {
    fn from((addr, port): (IpAddr, u16)) -> Self {
        match addr {
            IpAddr::V4(addr) => QSocketAddr::from((addr, port)),
            IpAddr::V6(addr) => QSocketAddr::from((addr, port)),
        }
    }
}

impl From<SocketAddrV4> for QSocketAddr {
    fn from(value: SocketAddrV4) -> Self {
        Self::from((*value.ip(), value.port()))
    }
}

impl From<SocketAddrV6> for QSocketAddr {
    fn from(value: SocketAddrV6) -> Self {
        Self::from((*value.ip(), value.port()))
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

macro_rules! impl_from_hostname {
    ($t:ty) => {
        impl From<($t, u16, QAbstractSocketNetworkLayerProtocol)> for QSocketAddr {
            fn from(
                (name, port, protocol): ($t, u16, QAbstractSocketNetworkLayerProtocol),
            ) -> Self {
                Self {
                    name: QString::from(name),
                    port,
                    protocol,
                }
            }
        }
    };
}

impl From<(QString, u16, QAbstractSocketNetworkLayerProtocol)> for QSocketAddr {
    fn from((name, port, protocol): (QString, u16, QAbstractSocketNetworkLayerProtocol)) -> Self {
        Self {
            name,
            port,
            protocol,
        }
    }
}

impl From<(&QString, u16, QAbstractSocketNetworkLayerProtocol)> for QSocketAddr {
    fn from((name, port, protocol): (&QString, u16, QAbstractSocketNetworkLayerProtocol)) -> Self {
        Self {
            name: name.clone(),
            port,
            protocol,
        }
    }
}

impl_from_hostname!(&str);
impl_from_hostname!(&String);
impl_from_hostname!(String);

impl<T> From<(T, u16)> for QSocketAddr
where
    QSocketAddr: From<(T, u16, QAbstractSocketNetworkLayerProtocol)>,
{
    /// Connect using [`QAbstractSocketNetworkLayerProtocol::AnyIPProtocol`].
    fn from((name, port): (T, u16)) -> Self {
        Self::from((
            name,
            port,
            QAbstractSocketNetworkLayerProtocol::AnyIPProtocol,
        ))
    }
}

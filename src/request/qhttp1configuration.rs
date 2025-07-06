use std::fmt;
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qtypes.h");
        type qsizetype = cxx_qt_lib::qsizetype;
    }

    extern "C++" {
        include!("cxx-qt-io/qhttp1configuration.h");
    }

    unsafe extern "C++" {
        type QHttp1Configuration = super::QHttp1Configuration;

        #[doc(hidden)]
        #[rust_name = "number_of_connections_per_host_qsizetype"]
        fn numberOfConnectionsPerHost(&self) -> qsizetype;

        #[doc(hidden)]
        #[rust_name = "set_number_of_connections_per_host_qsizetype"]
        fn setNumberOfConnectionsPerHost(&mut self, number: qsizetype);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhttp1configuration_drop"]
        fn drop(config: &mut QHttp1Configuration);

        #[rust_name = "qhttp1configuration_init_default"]
        fn construct() -> QHttp1Configuration;
        #[rust_name = "qhttp1configuration_clone"]
        fn construct(other: &QHttp1Configuration) -> QHttp1Configuration;

        #[rust_name = "qhttp1configuration_eq"]
        fn operatorEq(a: &QHttp1Configuration, b: &QHttp1Configuration) -> bool;
    }
}

/// The `QHttp1Configuration` class controls HTTP/1 parameters and settings.
///
/// Introduced in Qt 6.5.
///
/// Qt Documentation: [QHttp1Configuration](https://doc.qt.io/qt-6/qhttp1configuration.html#details)
#[repr(C)]
pub struct QHttp1Configuration {
    _space: MaybeUninit<usize>,
}

impl Clone for QHttp1Configuration {
    fn clone(&self) -> Self {
        ffi::qhttp1configuration_clone(self)
    }
}

impl Default for QHttp1Configuration {
    /// Default constructs a `QHttp1Configuration` object.
    fn default() -> Self {
        ffi::qhttp1configuration_init_default()
    }
}

impl Drop for QHttp1Configuration {
    fn drop(&mut self) {
        ffi::qhttp1configuration_drop(self);
    }
}

impl PartialEq for QHttp1Configuration {
    fn eq(&self, other: &Self) -> bool {
        ffi::qhttp1configuration_eq(self, other)
    }
}

impl Eq for QHttp1Configuration {}

impl fmt::Debug for QHttp1Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QHttp1Configuration")
            .field(
                "number_of_connections_per_host",
                &self.number_of_connections_per_host(),
            )
            .finish()
    }
}

impl QHttp1Configuration {
    /// Returns the number of connections used per http(s) host:port combination. The default is 6.
    pub fn number_of_connections_per_host(&self) -> isize {
        self.number_of_connections_per_host_qsizetype().into()
    }

    /// Sets the number of connections (minimum: 1; maximum: 255) used per http(s) host:port combination to `number`.
    ///
    /// If `number` is ≤ 0, does nothing. If `number` is > 255, 255 is used.
    pub fn set_number_of_connections_per_host(&mut self, number: isize) {
        self.set_number_of_connections_per_host_qsizetype(number.into());
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QHttp1Configuration {
    type Id = type_id!("QHttp1Configuration");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QHttp1ConfigurationProps {
            number_of_connections_per_host: isize,
        }

        let props = QHttp1ConfigurationProps {
            number_of_connections_per_host: 3,
        };

        let mut config = QHttp1Configuration::default();

        config.set_number_of_connections_per_host(props.number_of_connections_per_host);

        let actual_props = QHttp1ConfigurationProps {
            number_of_connections_per_host: config.number_of_connections_per_host(),
        };

        assert_eq!(actual_props, props);
    }
}

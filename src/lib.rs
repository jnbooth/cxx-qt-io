#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::undocumented_unsafe_blocks))]
#![cfg_attr(test, allow(clippy::unwrap_used))]
#![allow(non_upper_case_globals)]
//!
//!
//! # Feature flags
//!
//! cxx-qt-io uses a set of [feature flags] to reduce the amount of compiled code. It
//! is possible to just enable certain features over others. By default, Tokio
//! does not enable any features but allows one to enable a subset for their use
//! case. Below is a list of the available feature flags. You may also notice
//! above each function, struct and trait there is listed one or more feature flags
//! that are required for that item to be used. If you are new to Tokio it is
//! recommended that you use the `full` feature flag which will enable all public APIs.
//! Beware though that this will pull in many extra dependencies that you may not
//! need.
//!
//! - `full`: Enables all features listed below except `link_qt_object_files`.
//! - `qt_core`: Enables all features for the `QtCore` module (i.e. `fs`).
//! - `qt_network`: Enables all features for the `QtNetwork` module (i.e. `net`, `request`, and `ssl`).
//! - `fs`: Bindings for [`QDir`], [`QFile`], [`QSaveFile`], and [`QTemporaryFile`].
//! - `net`: Bindings for [`QLocalSocket`], [`QTcpServer`], [`QTcpSocket`], and [`QUdpSocket`].
//! - `request`: Bindings for [`QNetworkAccessManager`], [`QNetworkRequest`], and [`QNetworkReply`].
//! - `ssl`: Bindings for [`QSslServer`] and [`QSslSocket`].
//! - `link_qt_object_files`: Sets the `link_qt_object_files` feature flag for `cxx-qt-build`.
//!   This is required for static linking.

#[cfg(any(not(cxxqt_qt_version_major = "6"), not(cxxqt_qt_version_at_least_6_1)))]
compile_error!("cxx-qt-io only supports Qt 6.1 and above");

#[cfg(all(doc, not(cxxqt_qt_version_at_least_6_4)))]
pub struct QSslServer;

#[macro_use]
mod macros;

mod core;
pub use core::*;

#[cfg(feature = "fs")]
mod fs;
#[cfg(feature = "fs")]
pub use fs::*;

#[cfg(feature = "net")]
mod net;
#[cfg(feature = "net")]
pub use net::*;

#[cfg(feature = "request")]
mod request;
#[cfg(feature = "request")]
pub use request::*;

#[cfg(feature = "ssl")]
mod ssl;
#[cfg(feature = "ssl")]
pub use ssl::*;

mod util;

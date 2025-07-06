use std::fmt;
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qhttp2configuration.h");
    }

    unsafe extern "C++" {
        type QHttp2Configuration = super::QHttp2Configuration;

        /// Returns `true` if the Huffman coding in HPACK is enabled.
        #[rust_name = "huffman_compression_enabled"]
        fn huffmanCompressionEnabled(&self) -> bool;

        /// Returns the maximum number of concurrent streams.
        ///
        /// Introduced in Qt 6.9.
        #[cfg(cxxqt_qt_version_at_least_6_9)]
        #[rust_name = "max_concurrent_streams"]
        fn maxConcurrentStreams(&self) -> u32;

        /// Returns the maximum payload size that HTTP/2 frames can have. The default (initial) value is 16384 octets.
        #[rust_name = "max_frame_size"]
        fn maxFrameSize(&self) -> u32;

        /// Returns `true` if server push was enabled.
        ///
        /// **Note:** By default, [`QNetworkAccessManager`](crate::QNetworkAccessManager) disables server push via the 'SETTINGS' frame.
        #[rust_name = "server_push_enabled"]
        fn serverPushEnabled(&self) -> bool;

        /// Returns the window size for connection-level flow control. The default value [`QNetworkAccessManager`](crate::QNetworkAccessManager) will be using is 2147483647 octets.
        #[rust_name = "session_receive_window_size"]
        fn sessionReceiveWindowSize(&self) -> u32;

        /// If `enable` is true, HPACK compression will additionally compress string using the Huffman coding. Enabled by default.
        ///
        /// **Note:** This parameter only affects 'HEADERS' frames that [`QNetworkAccessManager`](crate::QNetworkAccessManager) is sending.
        #[rust_name = "set_huffman_compression_enabled"]
        fn setHuffmanCompressionEnabled(&mut self, enable: bool);

        /// Sets `value` as the maximum number of concurrent streams that will be advertised to the peer when sending SETTINGS frame.
        ///
        /// Introduced in Qt 6.9.
        #[cfg(cxxqt_qt_version_at_least_6_9)]
        #[rust_name = "set_max_concurrent_streams"]
        fn setMaxConcurrentStreams(&mut self, value: u32);

        /// Sets the maximum frame size that [`QNetworkAccessManager`](crate::QNetworkAccessManager) will advertise to the server when sending its initial SETTINGS frame.
        ///
        /// **Note:** While this size is required to be within a range between 16384 and 16777215 inclusive, the actual payload size in frames that carry payload maybe be less than 16384.
        ///
        /// Returns `true` on success, `false` otherwise.
        #[rust_name = "set_max_frame_size"]
        fn setMaxFrameSize(&mut self, size: u32) -> bool;

        /// If `enable` is true, a remote server can potentially use server push to send responses in advance.
        #[rust_name = "set_server_push_enabled"]
        fn setServerPushEnabled(&mut self, enable: bool);

        /// Sets the window size for connection-level flow control. `size` cannot be 0 and must not exceed 2147483647 octets.
        ///
        /// Returns `true` on success, `false` otherwise.
        #[rust_name = "set_session_receive_window_size"]
        fn setSessionReceiveWindowSize(&mut self, size: u32) -> bool;

        /// Sets the window size for stream-level flow control. `size` cannot be 0 and must not exceed 2147483647 octets.
        ///
        /// Returns `true` on success, `false` otherwise.
        #[rust_name = "set_stream_receive_window_size"]
        fn setStreamReceiveWindowSize(&mut self, size: u32) -> bool;

        /// Returns the window size for stream-level flow control. The default value [`QNetworkAccessManager`](crate::QNetworkAccessManager) will be using is 214748364 octets (see [RFC 7540](https://httpwg.org/specs/rfc7540.html#SettingValues)).
        #[rust_name = "stream_receive_window_size"]
        fn streamReceiveWindowSize(&self) -> u32;

    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhttp2configuration_drop"]
        fn drop(config: &mut QHttp2Configuration);

        #[rust_name = "qhttp2configuration_init_default"]
        fn construct() -> QHttp2Configuration;
        #[rust_name = "qhttp2configuration_clone"]
        fn construct(other: &QHttp2Configuration) -> QHttp2Configuration;

        #[rust_name = "qhttp2configuration_eq"]
        fn operatorEq(a: &QHttp2Configuration, b: &QHttp2Configuration) -> bool;
    }
}

/// The `QHttp2Configuration` class controls HTTP/2 parameters and settings.
///
/// Qt Documentation: [QHttp2Configuration](https://doc.qt.io/qt-6/qhttp2configuration.html#details)
#[repr(C)]
pub struct QHttp2Configuration {
    _space: MaybeUninit<usize>,
}

impl Clone for QHttp2Configuration {
    fn clone(&self) -> Self {
        ffi::qhttp2configuration_clone(self)
    }
}

impl Default for QHttp2Configuration {
    /// Default constructs a `QHttp2Configuration` object.
    fn default() -> Self {
        ffi::qhttp2configuration_init_default()
    }
}

impl Drop for QHttp2Configuration {
    fn drop(&mut self) {
        ffi::qhttp2configuration_drop(self);
    }
}

impl PartialEq for QHttp2Configuration {
    fn eq(&self, other: &Self) -> bool {
        ffi::qhttp2configuration_eq(self, other)
    }
}

impl Eq for QHttp2Configuration {}

impl fmt::Debug for QHttp2Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = f.debug_struct("QHttp2Configuration");

        debug.field(
            "huffman_compression_enabled",
            &self.huffman_compression_enabled(),
        );

        #[cfg(cxxqt_qt_version_at_least_6_9)]
        debug.field("max_concurrent_streams", &self.max_concurrent_streams());

        debug
            .field("max_frame_size", &self.max_frame_size())
            .field("server_push_enabled", &self.server_push_enabled())
            .field(
                "session_receive_window_size",
                &self.session_receive_window_size(),
            )
            .field(
                "stream_receive_window_size",
                &self.stream_receive_window_size(),
            )
            .finish()
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QHttp2Configuration {
    type Id = type_id!("QHttp2Configuration");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QHttp2ConfigurationProps {
            huffman_compression_enabled: bool,
            #[cfg(cxxqt_qt_version_at_least_6_9)]
            max_concurrent_streams: u32,
            server_push_enabled: bool,
            session_receive_window_size: u32,
            stream_receive_window_size: u32,
        }

        let props = QHttp2ConfigurationProps {
            huffman_compression_enabled: true,
            #[cfg(cxxqt_qt_version_at_least_6_9)]
            max_concurrent_streams: 5,
            server_push_enabled: true,
            session_receive_window_size: 100,
            stream_receive_window_size: 200,
        };

        let mut config = QHttp2Configuration::default();

        config.set_huffman_compression_enabled(props.huffman_compression_enabled);
        config.set_server_push_enabled(props.server_push_enabled);
        #[cfg(cxxqt_qt_version_at_least_6_9)]
        config.set_max_concurrent_streams(props.max_concurrent_streams);
        config.set_server_push_enabled(props.server_push_enabled);
        config.set_session_receive_window_size(props.session_receive_window_size);
        config.set_stream_receive_window_size(props.stream_receive_window_size);

        let actual_props = QHttp2ConfigurationProps {
            huffman_compression_enabled: config.huffman_compression_enabled(),
            #[cfg(cxxqt_qt_version_at_least_6_9)]
            max_concurrent_streams: config.max_concurrent_streams(),
            server_push_enabled: config.server_push_enabled(),
            session_receive_window_size: config.session_receive_window_size(),
            stream_receive_window_size: config.stream_receive_window_size(),
        };

        assert_eq!(actual_props, props);
    }
}

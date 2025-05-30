use std::fmt;
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};
use cxx_qt_lib::QVariant;

use crate::QNetworkRequestKnownHeaders;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-io/qiodevice.h");
        type QIODevice = crate::QIODevice;
        include!("cxx-qt-io/qnetworkrequest.h");
        type QNetworkRequestKnownHeaders = crate::QNetworkRequestKnownHeaders;
    }

    extern "C++" {
        include!("cxx-qt-io/qhttppart.h");
    }

    unsafe extern "C++" {
        type QHttpPart = super::QHttpPart;

        /// Sets the body of this MIME part to `body`. The body set with this method will be used unless the device is set via [`set_body_device`](QHttpPart::set_body_device). For a large amount of data (e.g. an image), use [`set_body_device`](QHttpPart::set_body_device), which will not copy the data internally.
        #[rust_name = "set_body"]
        fn setBody(&mut self, body: &QByteArray);

        /// Sets the device to read the content from to `device`. For large amounts of data this method should be preferred over [`set_body`](QHttpPart::set_body), because the content is not copied when using this method, but read directly from the device. device must be open and readable. `QHttpPart` does not take ownership of `device`, i.e. the device must be closed and destroyed if necessary. if device is sequential (e.g. sockets, but not files), [`QNetworkAccessManager::post`](https://doc.qt.io/qt-6/qnetworkaccessmanager.html#post) should be called after `device` has emitted [`finished`](QIODevice::finished). For unsetting the device and using data set via [`set_body`](QHttpPart::set_body), call this method with a null pointer.
        ///
        /// # Safety
        /// *device* must remain a valid pointer for the lifespan of the request and should not be
        /// mutated until afterward.
        #[rust_name = "set_body_device"]
        unsafe fn setBodyDevice(&mut self, device: *mut QIODevice);

        #[doc(hidden)]
        #[rust_name = "set_header_qvariant"]
        fn setHeader(&mut self, header: QNetworkRequestKnownHeaders, value: &QVariant);

        /// Sets the header `header_name` to be of value `header_value`. If `header_name` corresponds to a known header (see [`QNetworkRequestKnownHeaders`]), the raw format will be parsed and the corresponding "cooked" header will be set as well.
        ///
        /// **Note:** Setting the same header twice overrides the previous setting. To accomplish the behaviour of multiple HTTP headers of the same name, you should concatenate the two values, separating them with a comma (",") and set one single raw header.
        #[rust_name = "set_raw_header"]
        fn setRawHeader(&mut self, header_name: &QByteArray, header_value: &QByteArray);

    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhttppart_drop"]
        fn drop(part: &mut QHttpPart);

        #[rust_name = "qhttppart_init_default"]
        fn construct() -> QHttpPart;
        #[rust_name = "qhttppart_clone"]
        fn construct(other: &QHttpPart) -> QHttpPart;

        #[rust_name = "qhttppart_eq"]
        fn operatorEq(a: &QHttpPart, b: &QHttpPart) -> bool;

        #[rust_name = "qhttppart_to_debug_qstring"]
        fn toDebugQString(value: &QHttpPart) -> QString;
    }
}

/// The `QHttpPart` class holds a body part to be used inside a HTTP multipart MIME message.
///
/// Qt Documentation: [QHttpPart](https://doc.qt.io/qt-6/qhttppart.html#details)
#[repr(C)]
pub struct QHttpPart {
    _space: MaybeUninit<usize>,
}

impl Clone for QHttpPart {
    fn clone(&self) -> Self {
        ffi::qhttppart_clone(self)
    }
}

impl Default for QHttpPart {
    fn default() -> Self {
        ffi::qhttppart_init_default()
    }
}

impl Drop for QHttpPart {
    fn drop(&mut self) {
        ffi::qhttppart_drop(self);
    }
}

impl PartialEq for QHttpPart {
    fn eq(&self, other: &Self) -> bool {
        ffi::qhttppart_eq(self, other)
    }
}

impl Eq for QHttpPart {}

impl fmt::Debug for QHttpPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qhttppart_to_debug_qstring(self).fmt(f)
    }
}

impl QHttpPart {
    // Sets the value of the known header `header` to be `value`, overriding any previously set headers.
    pub fn set_header<T>(&mut self, header: QNetworkRequestKnownHeaders, value: T)
    where
        T: Into<QVariant>,
    {
        self.set_header_qvariant(header, &value.into());
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QHttpPart {
    type Id = type_id!("QHttpPart");
    type Kind = cxx::kind::Trivial;
}

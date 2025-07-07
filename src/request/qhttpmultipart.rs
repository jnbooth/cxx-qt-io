use std::fmt;
use std::ops::Deref;

use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt::QObject;

#[cxx_qt::bridge]
mod ffi {
    /// List of known content types for a multipart subtype as described in RFC 2046 and others.
    #[repr(i32)]
    #[derive(Debug, PartialEq, Eq)]
    enum QHttpMultiPartContentType {
        /// Corresponds to the "multipart/mixed" subtype, meaning the body parts are independent of each other, as described in RFC 2046.
        MixedType,
        /// Corresponds to the "multipart/related" subtype, meaning the body parts are related to each other, as described in RFC 2387.
        RelatedType,
        /// Corresponds to the "multipart/form-data" subtype, meaning the body parts contain form elements, as described in RFC 2388.
        FormDataType,
        /// Corresponds to the "multipart/alternative" subtype, meaning the body parts are alternative representations of the same information, as described in RFC 2046.
        AlternativeType,
    }

    extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;

        include!("cxx-qt-io/qhttppart.h");
        type QHttpPart = crate::QHttpPart;
    }

    extern "C++" {
        include!("cxx-qt-io/qhttpmultipart.h");
        type QHttpMultiPartContentType;
    }

    unsafe extern "C++Qt" {
        /// The `QHttpMultiPart` class resembles a MIME multipart message to be sent over HTTP.
        ///
        /// Qt Documentation: [QHttpMultiPart](https://doc.qt.io/qt-6/qhttpmultipart.html#details)
        #[qobject]
        #[base = QObject]
        type QHttpMultiPart;

        /// Appends `http_part` to this multipart.
        fn append(self: Pin<&mut QHttpMultiPart>, part: &QHttpPart);

        /// Returns the boundary.
        fn boundary(self: &QHttpMultiPart) -> QByteArray;

        /// Sets the content type to `content_type`. The content type will be used in the HTTP header section when sending the multipart message via [`QNetworkAccessManager::post`](crate::QNetworkAccessManager::post). In case you want to use a multipart subtype not contained in [`QHttpMultiPartContentType`], you can add the `"Content-Type"` header field to the [`QNetworkRequest`](crate::QNetworkRequest) by hand, and then use this request together with the multipart message for posting.
        #[rust_name = "set_content_type"]
        fn setContentType(self: Pin<&mut QHttpMultiPart>, content_type: QHttpMultiPartContentType);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhttpmultipart_init_content_type"]
        fn make_unique(content_type: QHttpMultiPartContentType) -> UniquePtr<QHttpMultiPart>;
    }
}

pub use ffi::{QHttpMultiPart, QHttpMultiPartContentType};

use crate::qobject::debug_qobject;

impl fmt::Debug for QHttpMultiPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QHttpMultiPart {
    /// Constructs a `QHttpMultiPart` with content type `content_type`.
    pub fn new(content_type: QHttpMultiPartContentType) -> UniquePtr<Self> {
        ffi::qhttpmultipart_init_content_type(content_type)
    }
}

impl Deref for QHttpMultiPart {
    type Target = QObject;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

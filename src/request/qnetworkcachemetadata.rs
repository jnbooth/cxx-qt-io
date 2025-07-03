use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};
use cxx_qt_lib::QHash;

use crate::util::IsNonNull;
use crate::RawHeaderList;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = cxx_qt_lib::QDateTime;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-io/qhash.h");
        type QHash_QNetworkRequestAttribute_QVariant =
            cxx_qt_lib::QHash<crate::QHashPair_QNetworkRequestAttribute_QVariant>;

        include!("cxx-qt-io/qlist.h");
        type QList_QPair_QByteArray_QByteArray =
            cxx_qt_lib::QList<crate::QPair<cxx_qt_lib::QByteArray, cxx_qt_lib::QByteArray>>;
    }

    #[cfg(cxxqt_qt_version_at_least_6_8)]
    extern "C++" {
        include!("cxx-qt-io/qhttpheaders.h");
        type QHttpHeaders = crate::QHttpHeaders;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkcachemetadata.h");
    }

    unsafe extern "C++" {
        type QNetworkCacheMetaData = super::QNetworkCacheMetaData;

        /// Returns all the attributes stored with this cache item.
        fn attributes(&self) -> QHash_QNetworkRequestAttribute_QVariant;

        /// Returns the date and time when the meta data expires.
        #[rust_name = "expiration_date"]
        fn expirationDate(&self) -> QDateTime;

        /// Returns headers in form of `QHttpHeaders` that are set in this meta data.
        ///
        /// Introduced in Qt 6.8.
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        fn headers(&self) -> QHttpHeaders;

        /// Returns `true` if this network cache meta data has attributes that have been set, otherwise `false`.
        #[rust_name = "is_valid"]
        fn isValid(&self) -> bool;

        /// Returns the date and time when the meta data was last modified.
        #[rust_name = "last_modified"]
        fn lastModified(&self) -> QDateTime;

        #[doc(hidden)]
        #[rust_name = "raw_headers_qlist"]
        fn rawHeaders(&self) -> QList_QPair_QByteArray_QByteArray;

        /// Returns if this cache should be allowed to be stored on disk.
        ///
        /// Some cache implementations can keep these cache items in memory for performance reasons, but for security reasons they should not be written to disk.
        ///
        /// Specifically with http, documents with Cache-control set to no-store or any https document that doesn't have "Cache-control: public" set will set the `save_to_disk` to false.
        #[rust_name = "save_to_disk"]
        fn saveToDisk(&self) -> bool;

        /// Sets all attributes of this cache item to be the map `attributes`.
        #[rust_name = "set_attributes"]
        fn setAttributes(&mut self, attributes: &QHash_QNetworkRequestAttribute_QVariant);

        /// Sets the date and time when the meta data expires to `date_time`.
        #[rust_name = "set_expiration_date"]
        fn setExpirationDate(&mut self, date_time: &QDateTime);

        /// Sets the headers of this network cache meta data to `headers`.
        ///
        /// Introduced in Qt 6.8.
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        #[rust_name = "set_headers"]
        fn setHeaders(&mut self, headers: &QHttpHeaders);

        /// Sets the date and time when the meta data was last modified to `date_time`.
        #[rust_name = "set_last_modified"]
        fn setLastModified(&mut self, date_time: &QDateTime);

        #[doc(hidden)]
        #[rust_name = "set_raw_headers_qlist"]
        fn setRawHeaders(&mut self, list: &QList_QPair_QByteArray_QByteArray);

        /// Sets whether this network cache meta data and associated content should be allowed to be stored on disk to `allow`.
        #[rust_name = "set_save_to_disk"]
        fn setSaveToDisk(&mut self, allow: bool);

        /// Sets the URL this network cache meta data to be `url`.
        ///
        /// The password and fragment are removed from the url.
        #[rust_name = "set_url"]
        fn setUrl(&mut self, url: &QUrl);

        /// Returns the URL this network cache meta data is referring to.
        fn url(&self) -> QUrl;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkcachemetadata_drop"]
        fn drop(metadata: &mut QNetworkCacheMetaData);

        #[rust_name = "qnetworkcachemetadata_init_default"]
        fn construct() -> QNetworkCacheMetaData;
        #[rust_name = "qnetworkcachemetadata_clone"]
        fn construct(other: &QNetworkCacheMetaData) -> QNetworkCacheMetaData;

        #[rust_name = "qnetworkcachemetadata_eq"]
        fn operatorEq(a: &QNetworkCacheMetaData, b: &QNetworkCacheMetaData) -> bool;
    }
}

pub type QNetworkCacheMetaDataAttributesMap =
    QHash<crate::QHashPair_QNetworkRequestAttribute_QVariant>;

pub type QNetworkCacheMetaDataRawHeader =
    crate::QPair<cxx_qt_lib::QByteArray, cxx_qt_lib::QByteArray>;

pub type QNetworkCacheMetaDataRawHeaderList = cxx_qt_lib::QList<QNetworkCacheMetaDataRawHeader>;

/// The `QNetworkCacheMetaData` class provides class information.
///
/// Qt Documentation: [QNetworkCacheMetaData](https://doc.qt.io/qt-6/qnetworkcachemetadata.html#details)
#[repr(C)]
pub struct QNetworkCacheMetaData {
    _space: MaybeUninit<usize>,
}

impl Clone for QNetworkCacheMetaData {
    fn clone(&self) -> Self {
        ffi::qnetworkcachemetadata_clone(self)
    }
}

impl Default for QNetworkCacheMetaData {
    fn default() -> Self {
        ffi::qnetworkcachemetadata_init_default()
    }
}

impl Drop for QNetworkCacheMetaData {
    fn drop(&mut self) {
        ffi::qnetworkcachemetadata_drop(self);
    }
}

impl PartialEq for QNetworkCacheMetaData {
    fn eq(&self, other: &Self) -> bool {
        ffi::qnetworkcachemetadata_eq(self, other)
    }
}

impl Eq for QNetworkCacheMetaData {}

impl IsNonNull for QNetworkCacheMetaData {
    fn is_nonnull(value: &Self) -> bool {
        value.is_valid()
    }
}

impl QNetworkCacheMetaData {
    /// Returns a list of all raw headers that are set in this meta data. The list is in the same order that the headers were set.
    pub fn raw_headers(&self) -> RawHeaderList {
        RawHeaderList {
            inner: self.raw_headers_qlist(),
        }
    }

    /// Sets the raw headers to `list`.
    pub fn set_raw_headers(&mut self, list: &RawHeaderList) {
        self.set_raw_headers_qlist(&list.inner);
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QNetworkCacheMetaData {
    type Id = type_id!("QNetworkCacheMetaData");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use cxx_qt_lib::{QByteArray, QDateTime, QString, QUrl};

    use super::*;
    use crate::RawHeaderList;

    #[test]
    fn props() {
        #[derive(Debug, PartialEq, Eq)]
        struct QNetworkCacheMetaDataProps {
            expiration_date: QDateTime,
            last_modified: QDateTime,
            raw_headers: RawHeaderList,
            save_to_disk: bool,
            url: QUrl,
        }

        let props = QNetworkCacheMetaDataProps {
            expiration_date: QDateTime::current_date_time_utc().add_days(2.into()),
            last_modified: QDateTime::current_date_time_utc().add_days((-2).into()),
            raw_headers: [
                (QByteArray::from("a"), QByteArray::from("b")),
                (QByteArray::from("c"), QByteArray::from("d")),
            ]
            .iter()
            .collect(),
            save_to_disk: true,
            url: QUrl::from_local_file(&QString::from("./test")),
        };

        let mut metadata = QNetworkCacheMetaData::default();

        metadata.set_expiration_date(&props.expiration_date);
        metadata.set_last_modified(&props.last_modified);
        metadata.set_raw_headers(&props.raw_headers);
        metadata.set_save_to_disk(props.save_to_disk);
        metadata.set_url(&props.url);

        let actual_props = QNetworkCacheMetaDataProps {
            expiration_date: metadata.expiration_date(),
            last_modified: metadata.last_modified(),
            raw_headers: metadata.raw_headers(),
            save_to_disk: metadata.save_to_disk(),
            url: metadata.url(),
        };

        assert_eq!(actual_props, props);
    }
}

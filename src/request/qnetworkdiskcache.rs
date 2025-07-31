use std::fmt;
use std::ops::Deref;
use std::pin::Pin;

use cxx::UniquePtr;
use cxx_qt::casting::Upcast;
use cxx_qt::QObject;
use cxx_qt_lib::QString;

use crate::qobject::debug_qobject;
use crate::util::IsNonNull;
use crate::{QAbstractNetworkCache, QNetworkCacheMetaData};

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qtypes.h");
        type qint64 = cxx_qt_lib::qint64;

        include!("cxx-qt-io/qnetworkcachemetadata.h");
        type QNetworkCacheMetaData = crate::QNetworkCacheMetaData;
    }

    extern "C++" {
        include!("cxx-qt-io/qnetworkdiskcache.h");
        type QAbstractNetworkCache = crate::QAbstractNetworkCache;
    }

    unsafe extern "C++Qt" {
        /// The `QNetworkDiskCache` class provides a very basic disk cache.
        ///
        /// Qt Documentation: [QNetworkDiskCache](https://doc.qt.io/qt-6/qnetworkdiskcache.html#details)
        #[qobject]
        #[base = QAbstractNetworkCache]
        type QNetworkDiskCache;

        /// Returns the location where cached files will be stored.
        #[rust_name = "cache_directory"]
        fn cacheDirectory(self: &QNetworkDiskCache) -> QString;

        #[doc(hidden)]
        #[rust_name = "file_meta_data_or_invalid"]
        fn fileMetaData(self: &QNetworkDiskCache, file_name: &QString) -> QNetworkCacheMetaData;

        #[doc(hidden)]
        #[rust_name = "maximum_cache_size_qint64"]
        fn maximumCacheSize(self: &QNetworkDiskCache) -> qint64;

        /// Sets the directory where cached files will be stored to `cache_dir`.
        ///
        /// `QNetworkDiskCache` will create this directory if it does not exists.
        ///
        /// Prepared cache items will be stored in the new cache directory when they are inserted.
        #[rust_name = "set_cache_directory"]
        fn setCacheDirectory(self: Pin<&mut QNetworkDiskCache>, cache_dir: &QString);

        #[doc(hidden)]
        #[rust_name = "set_maximum_cache_size_qint64"]
        fn setMaximumCacheSize(self: Pin<&mut QNetworkDiskCache>, size: qint64);
    }

    #[namespace = "rust::cxxqt1"]
    unsafe extern "C++" {
        include!("cxx-qt/casting.h");

        #[rust_name = "upcast_qnetworkdiskcache_qobject"]
        unsafe fn upcastPtr(socket: *const QNetworkDiskCache) -> *const QObject;
        #[rust_name = "downcast_qobject_qnetworkdiskcache"]
        unsafe fn downcastPtr(socket: *const QObject) -> *const QNetworkDiskCache;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qnetworkdiskcache_default"]
        fn make_unique() -> UniquePtr<QNetworkDiskCache>;
    }
}

pub use ffi::QNetworkDiskCache;

impl fmt::Debug for QNetworkDiskCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QNetworkDiskCache {
    /// Creates a new disk cache.
    pub fn new() -> UniquePtr<Self> {
        ffi::qnetworkdiskcache_default()
    }

    /// Returns the metadata for the cache file `file_name`.
    ///
    /// Returns `None` if `file_name` is not a cache file.
    pub fn file_meta_data(&self, file_name: &QString) -> Option<QNetworkCacheMetaData> {
        self.file_meta_data_or_invalid(file_name).nonnull()
    }

    /// Returns the current maximum size for the disk cache.
    pub fn maximum_cache_size(&self) -> i64 {
        self.maximum_cache_size_qint64().into()
    }
    /// Sets the maximum size of the disk cache to be size.
    ///
    /// If the new size is smaller then the current cache size then the cache will clean itself so that its size is under the maximum cache size.
    pub fn set_maximum_cache_size(self: Pin<&mut Self>, size: i64) {
        self.set_maximum_cache_size_qint64(size.into());
    }

    /// Casts this object to `QAbstractNetworkCache`.
    pub fn as_abstract_network_cache(&self) -> &QAbstractNetworkCache {
        self.upcast()
    }

    /// Mutably casts this object to `QAbstractNetworkCache`.
    pub fn as_abstract_network_cache_mut(self: Pin<&mut Self>) -> Pin<&mut QAbstractNetworkCache> {
        self.upcast_pin()
    }
}

impl Deref for QNetworkDiskCache {
    type Target = QAbstractNetworkCache;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

// SAFETY: qobject_cast
unsafe impl Upcast<QObject> for QNetworkDiskCache {
    unsafe fn upcast_ptr(this: *const Self) -> *const QObject {
        ffi::upcast_qnetworkdiskcache_qobject(this)
    }

    unsafe fn from_base_ptr(base: *const QObject) -> *const Self {
        ffi::downcast_qobject_qnetworkdiskcache(base)
    }
}

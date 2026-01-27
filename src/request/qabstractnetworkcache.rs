use std::fmt;
use std::io::{self, Read, Write};
use std::ops::Deref;
use std::pin::Pin;

use cxx::UniquePtr;
use cxx_qt::QObject;
use cxx_qt::casting::Upcast;
use cxx_qt_lib::QUrl;

use crate::qobject::debug_qobject;
use crate::util::IsNonNull;
use crate::{QIODevice, QNetworkCacheMetaData};

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qtypes.h");
        type qint64 = cxx_qt_lib::qint64;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;

        include!("cxx-qt-io/qiodevice.h");
        type QIODevice = crate::QIODevice;
        include!("cxx-qt-io/qnetworkcachemetadata.h");
        type QNetworkCacheMetaData = crate::QNetworkCacheMetaData;
    }

    extern "C++" {
        include!("cxx-qt-io/qabstractnetworkcache.h");
    }

    unsafe extern "C++Qt" {
        /// The `QAbstractNetworkCache` class provides the interface for cache implementations.
        ///
        /// Qt Documentation: [QAbstractNetworkCache](https://doc.qt.io/qt-6/qabstractnetworkcache.html#details)
        #[qobject]
        #[base = QObject]
        type QAbstractNetworkCache;

        #[doc(hidden)]
        #[rust_name = "cache_size_qint64"]
        fn cacheSize(self: &QAbstractNetworkCache) -> qint64;

        /// Removes all items from the cache. Unless there was failures clearing the cache [`cache_size`](QAbstractNetworkCache::cache_size) should return 0 after a call to clear.
        fn clear(self: Pin<&mut QAbstractNetworkCache>);

        #[doc(hidden)]
        #[rust_name = "data_unsafe"]
        fn data(self: Pin<&mut QAbstractNetworkCache>, url: &QUrl) -> *mut QIODevice;

        /// Inserts the data in `device` and the prepared meta data into the cache. After this function is called the data and meta data should be retrievable using [`data`](QAbstractNetworkCache::data) and [`meta_data`](QAbstractNetworkCache::meta_data).
        ///
        /// To cancel a prepared inserted call [`remove`](QAbstractNetworkCache::remove) on the metadata's url.
        ///
        /// # Safety
        /// `device` must be a pointer returned by [`prepare`](QAbstractNetworkCache::prepare), along with all the usual pointer safety requirements.
        #[rust_name = "insert_unsafe"]
        unsafe fn insert(self: Pin<&mut QAbstractNetworkCache>, device: *mut QIODevice);

        #[doc(hidden)]
        #[rust_name = "meta_data_or_null"]
        fn metaData(self: Pin<&mut QAbstractNetworkCache>, url: &QUrl) -> QNetworkCacheMetaData;

        /// Returns the device that should be populated with the data for the cache item `meta_data`. When all of the data has been written [`insert`](QAbstractNetworkCache::insert) should be called. Returns a null pointer if `meta_data` is invalid or the url in the metadata is invalid.
        ///
        /// The cache owns the device and will take care of deleting it when it is inserted or removed.
        ///
        /// To cancel a prepared inserted call [`remove`](QAbstractNetworkCache::remove) on the metadata's url.
        ///
        /// # Safety
        /// You must ensure that the pointer returned by this function is deleted by calling either
        /// [`insert_unsafe`](QAbstractNetworkCache::insert) or [`remove`](QAbstractNetworkCache::remove)
        // on it. The pointer must not be used after either of those functions is called.
        #[rust_name = "prepare_unsafe"]
        fn prepare(
            self: Pin<&mut QAbstractNetworkCache>,
            meta_data: &QNetworkCacheMetaData,
        ) -> *mut QIODevice;

        /// Removes the cache entry for `url`, returning `true` if successful, otherwise `false`.
        fn remove(self: Pin<&mut QAbstractNetworkCache>, url: &QUrl) -> bool;

        /// Updates the cache meta date for the `meta_data`'s url to `meta_data`.
        ///
        /// If the cache does not contains a cache item for the url then no action is taken.
        #[rust_name = "update_meta_data"]
        fn updateMetaData(self: Pin<&mut QAbstractNetworkCache>, meta_data: &QNetworkCacheMetaData);
    }
}

pub use ffi::QAbstractNetworkCache;

impl fmt::Debug for QAbstractNetworkCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_qobject(f, self)
    }
}

impl QAbstractNetworkCache {
    /// Returns the current size taken up by the cache. Depending upon the cache implementation this might be disk or memory size.
    pub fn cache_size(&self) -> i64 {
        self.cache_size_qint64().into()
    }

    /// Returns the data associated with `url`.
    ///
    /// Returns a null pointer if there is no cache for `url`, the url is invalid, or if there is an internal cache error.
    pub fn data(self: Pin<&mut Self>, url: &QUrl) -> UniquePtr<QIODevice> {
        // SAFETY: `device` is valid and Qt expects us to delete it when done with it.
        unsafe { UniquePtr::from_raw(self.data_unsafe(url)) }
    }

    /// Returns the meta data for the url `url`.
    ///
    /// Returns `None` if the url is invalid or the cache does not contain the data for `url`.
    pub fn meta_data(self: Pin<&mut Self>, url: &QUrl) -> Option<QNetworkCacheMetaData> {
        self.meta_data_or_null(url).nonnull()
    }

    /// Inserts the data in `device` and the prepared meta data into the cache. After this function is called the data and meta data should be retrievable using [`data`](QAbstractNetworkCache::data) and [`meta_data`](QAbstractNetworkCache::meta_data).
    ///
    /// To cancel a prepared inserted call [`remove`](QAbstractNetworkCache::remove) on the metadata's url.
    ///
    /// # Safety
    /// `device` must be a pointer returned by [`prepare`](QAbstractNetworkCache::prepare), along with all the usual pointer safety requirements.
    pub unsafe fn insert<R: Read>(
        self: Pin<&mut Self>,
        meta_data: &QNetworkCacheMetaData,
        mut reader: R,
    ) -> io::Result<()> {
        #[cold]
        fn create_error(meta_data: &QNetworkCacheMetaData) -> io::Error {
            if meta_data.is_valid() {
                io::Error::other("invalid metadata")
            } else {
                io::Error::other("invalid url")
            }
        }
        let Some(mut device) = self.prepare(meta_data) else {
            return Err(create_error(meta_data));
        };
        io::copy(&mut reader, &mut device)?;
        device.insert();
        Ok(())
    }

    /// Returns the device that should be populated with the data for the cache item `meta_data`. When all of the data has been written [`insert`](QAbstractNetworkCache::insert) should be called. Returns `None` if `meta_data` is invalid or the url in the metadata is invalid.
    ///
    /// To cancel a prepared inserted call [`remove`](QAbstractNetworkCache::remove) on the metadata's url.
    pub fn prepare(
        mut self: Pin<&mut Self>,
        meta_data: &QNetworkCacheMetaData,
    ) -> Option<QAbstractNetworkCacheWriter<'_>> {
        // SAFETY: `QAbstractNetworkCacheWriter` will ensure the device is handled.
        let device = self.as_mut().prepare_unsafe(meta_data);
        if device.is_null() {
            return None;
        }
        Some(QAbstractNetworkCacheWriter {
            device,
            cache: self,
            inserted: false,
            url: meta_data.url(),
        })
    }
}

impl Deref for QAbstractNetworkCache {
    type Target = QObject;

    fn deref(&self) -> &Self::Target {
        self.upcast()
    }
}

/// A `QAbstractNetworkCacheWriter` handles writing to a `QAbstractNetworkCache`'s data entry for a
/// URL, as returned by [`QAbstractNetworkCache::prepare`].
///
/// When all data has been written, you must call [`insert`](QAbstractNetworkCacheWriter::insert),
/// otherwise the write will be canceled and the data entry will be removed from the cache.
///
/// Alternatively, you can call [`remove`](QAbstractNetworkCacheWriter::remove) to explicitly cancel
/// the write. This is the same behavior as dropping the writer without calling `insert`.
///
#[must_use = "Immediately dropping a QAbstractNetworkCacheWriter is equivalent to calling cache.remove(&metadata.url())."]
pub struct QAbstractNetworkCacheWriter<'a> {
    device: *mut QIODevice,
    cache: Pin<&'a mut QAbstractNetworkCache>,
    inserted: bool,
    url: QUrl,
}

impl QAbstractNetworkCacheWriter<'_> {
    /// Insert the written data into the cache.
    pub fn insert(mut self) {
        // SAFETY: `self.device` is a valid pointer to a device provided by the cache.
        unsafe { self.cache.as_mut().insert_unsafe(self.device) };
        self.inserted = true;
    }

    /// Cancel the write, removing the data entry from the cache. This is the same behavior as
    /// dropping the writer without calling [`insert`](QAbstractNetworkCacheWriter::insert).
    pub fn remove(self) {
        drop(self);
    }
}

impl Drop for QAbstractNetworkCacheWriter<'_> {
    fn drop(&mut self) {
        if self.inserted {
            return;
        }
        self.cache.as_mut().remove(&self.url);
    }
}

impl Write for QAbstractNetworkCacheWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // SAFETY: Transient pin of immovable device.
        unsafe { Pin::new_unchecked(&mut *self.device) }.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        // SAFETY: Transient pin of immovable device.
        unsafe { Pin::new_unchecked(&mut *self.device) }.flush()
    }
}

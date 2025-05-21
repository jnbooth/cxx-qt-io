use std::borrow::Borrow;
use std::iter::FusedIterator;

use cxx_qt_lib::{QByteArray, QList, QListElement};

use crate::QPair;

#[cxx_qt::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qtypes.h");
        type qsizetype = cxx_qt_lib::qsizetype;

        include!("cxx-qt-io/qlist_qpair_qbytearray_qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        type QPair_QByteArray_QByteArray = crate::QPair<QByteArray, QByteArray>;
        type QList_QPair_QByteArray_QByteArray = cxx_qt_lib::QList<QPair_QByteArray_QByteArray>;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        include!("cxx-qt-io/qlist_qpair.h");

        #[rust_name = "header_list_append"]
        fn qpairlistAppend(
            list: &mut QList_QPair_QByteArray_QByteArray,
            name: &QByteArray,
            value: &QByteArray,
        );

        #[rust_name = "header_list_contains"]
        fn qpairlistContains(list: &QList_QPair_QByteArray_QByteArray, name: &QByteArray) -> bool;

        #[rust_name = "header_list_index_of"]
        fn qpairlistIndexOf(
            list: &QList_QPair_QByteArray_QByteArray,
            name: &QByteArray,
        ) -> qsizetype;

        #[rust_name = "header_list_insert"]
        fn qpairlistInsert(
            list: &mut QList_QPair_QByteArray_QByteArray,
            i: qsizetype,
            name: &QByteArray,
            value: &QByteArray,
        );
    }
}

/// Wrapper around a `QList<QPair<QByteArray, QByteArray>>`.
///
/// Qt Documentation: [QNetworkCacheMetaData::RawHeaderList](https://doc.qt.io/qt-6/qnetworkcachemetadata.html#RawHeaderList-typedef)
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RawHeaderList {
    inner: QList<QPair<QByteArray, QByteArray>>,
}

impl AsRef<QList<QPair<QByteArray, QByteArray>>> for RawHeaderList {
    fn as_ref(&self) -> &QList<QPair<QByteArray, QByteArray>> {
        &self.inner
    }
}

impl RawHeaderList {
    /// Inserts a name-value pair at the end of the list.
    pub fn append(&mut self, name: &QByteArray, value: &QByteArray) {
        ffi::header_list_append(&mut self.inner, name, value);
    }

    /// Removes all elements from the list.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Returns true if the list contains a pair with name `name`; otherwise returns false.
    pub fn contains(&self, name: &QByteArray) -> bool {
        ffi::header_list_contains(&self.inner, name)
    }

    /// Returns the item at `index` position in the list.
    pub fn get(&self, index: isize) -> Option<(&QByteArray, &QByteArray)> {
        let pair = self.inner.get(index)?;
        Some((pair.first(), pair.second()))
    }

    /// Returns the index position of the first occurrence of a pair with name `name` in the list,
    /// searching forward from index position from. Returns -1 if no item matched.
    pub fn index_of(&self, name: &QByteArray) -> isize {
        ffi::header_list_index_of(&self.inner, name).into()
    }

    /// Inserts a name-value pair into the list at the given position.
    pub fn insert(&mut self, pos: isize, name: &QByteArray, value: &QByteArray) {
        ffi::header_list_insert(&mut self.inner, pos.into(), name, value);
    }

    /// Returns true if the list contains no elements; otherwise returns false.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `(&'a QByteArray, &'a QByteArray)`.
    pub fn iter(&self) -> Iter {
        Iter {
            list: &self.inner,
            index: 0,
            len: self.len(),
        }
    }

    /// Returns the number of items in the list.
    pub fn len(&self) -> isize {
        self.inner.len()
    }

    /// Removes the element at index position.
    pub fn remove(&mut self, pos: isize) {
        self.inner.remove(pos);
    }

    /// Reserve the specified capacity to prevent repeated allocations
    /// when the maximum size is known.
    pub fn reserve(&mut self, size: isize) {
        self.inner.reserve(size);
    }

    /// Helper function for handling Rust values.
    fn reserve_usize(&mut self, size: usize) {
        if size != 0 {
            self.inner
                .reserve(isize::try_from(size).unwrap_or(isize::MAX));
        }
    }
}

impl From<RawHeaderList> for QList<QPair<QByteArray, QByteArray>> {
    fn from(value: RawHeaderList) -> Self {
        value.inner
    }
}

impl From<QList<QPair<QByteArray, QByteArray>>> for RawHeaderList {
    fn from(value: QList<QPair<QByteArray, QByteArray>>) -> Self {
        Self { inner: value }
    }
}

impl<K, V> From<&[(K, V)]> for RawHeaderList
where
    K: Borrow<QByteArray>,
    V: Borrow<QByteArray>,
{
    fn from(value: &[(K, V)]) -> Self {
        value.iter().collect()
    }
}

impl<K, V> From<&Vec<(K, V)>> for RawHeaderList
where
    K: Borrow<QByteArray>,
    V: Borrow<QByteArray>,
{
    fn from(value: &Vec<(K, V)>) -> Self {
        value.iter().collect()
    }
}

impl<'a, K, V> Extend<&'a (K, V)> for RawHeaderList
where
    K: Borrow<QByteArray>,
    V: Borrow<QByteArray>,
{
    fn extend<I: IntoIterator<Item = &'a (K, V)>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve_usize(iter.size_hint().0);
        for (name, value) in iter {
            self.append(name.borrow(), value.borrow());
        }
    }
}

impl<'a, K, V> FromIterator<&'a (K, V)> for RawHeaderList
where
    K: Borrow<QByteArray>,
    V: Borrow<QByteArray>,
{
    fn from_iter<I: IntoIterator<Item = &'a (K, V)>>(iter: I) -> Self {
        let mut list = Self::default();
        list.extend(iter);
        list
    }
}

pub struct Iter<'a> {
    list: &'a QList<QPair<QByteArray, QByteArray>>,
    index: isize,
    len: isize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a QByteArray, &'a QByteArray);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }
        let next = unsafe { QListElement::get_unchecked(self.list, self.index) };
        self.index += 1;
        Some((next.first(), next.second()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {
    fn len(&self) -> usize {
        usize::try_from(self.len - self.index).unwrap_or_default()
    }
}

impl<'a> FusedIterator for Iter<'a> {}

impl<'a> IntoIterator for &'a RawHeaderList {
    type Item = (&'a QByteArray, &'a QByteArray);

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

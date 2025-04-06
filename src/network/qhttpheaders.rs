use std::fmt::{self, Debug, Formatter};
use std::mem::MaybeUninit;

use crate::{QPair, QPairPair_QByteArray_QByteArray};
use cxx::{type_id, ExternType};
use cxx_qt_lib::{QAnyStringView, QByteArray, QList};

#[cxx::bridge]
mod ffi {
    /// List of well known headers as per [IANA registry](https://www.iana.org/assignments/http-fields).
    #[repr(i32)]
    #[derive(Debug)]
    enum QHttpHeadersWellKnownHeader {
        // IANA Permanent status:
        AIM,
        Accept,
        AcceptAdditions,
        AcceptCH,
        AcceptDatetime,
        AcceptEncoding,
        AcceptFeatures,
        AcceptLanguage,
        AcceptPatch,
        AcceptPost,
        AcceptRanges,
        AcceptSignature,
        AccessControlAllowCredentials,
        AccessControlAllowHeaders,
        AccessControlAllowMethods,
        AccessControlAllowOrigin,
        AccessControlExposeHeaders,
        AccessControlMaxAge,
        AccessControlRequestHeaders,
        AccessControlRequestMethod,
        Age,
        Allow,
        ALPN,
        AltSvc,
        AltUsed,
        Alternates,
        ApplyToRedirectRef,
        AuthenticationControl,
        AuthenticationInfo,
        Authorization,
        CacheControl,
        CacheStatus,
        CalManagedID,
        CalDAVTimezones,
        CapsuleProtocol,
        CDNCacheControl,
        CDNLoop,
        CertNotAfter,
        CertNotBefore,
        ClearSiteData,
        ClientCert,
        ClientCertChain,
        Close,
        Connection,
        ContentDigest,
        ContentDisposition,
        ContentEncoding,
        ContentID,
        ContentLanguage,
        ContentLength,
        ContentLocation,
        ContentRange,
        ContentSecurityPolicy,
        ContentSecurityPolicyReportOnly,
        ContentType,
        Cookie,
        CrossOriginEmbedderPolicy,
        CrossOriginEmbedderPolicyReportOnly,
        CrossOriginOpenerPolicy,
        CrossOriginOpenerPolicyReportOnly,
        CrossOriginResourcePolicy,
        DASL,
        Date,
        DAV,
        DeltaBase,
        Depth,
        Destination,
        DifferentialID,
        DPoP,
        DPoPNonce,
        EarlyData,
        ETag,
        Expect,
        ExpectCT,
        Expires,
        Forwarded,
        From,
        Hobareg,
        Host,
        If,
        IfMatch,
        IfModifiedSince,
        IfNoneMatch,
        IfRange,
        IfScheduleTagMatch,
        IfUnmodifiedSince,
        IM,
        IncludeReferredTokenBindingID,
        KeepAlive,
        Label,
        LastEventID,
        LastModified,
        Link,
        Location,
        LockToken,
        MaxForwards,
        MementoDatetime,
        Meter,
        MIMEVersion,
        Negotiate,
        NEL,
        ODataEntityId,
        ODataIsolation,
        ODataMaxVersion,
        ODataVersion,
        OptionalWWWAuthenticate,
        OrderingType,
        Origin,
        OriginAgentCluster,
        OSCORE,
        OSLCCoreVersion,
        Overwrite,
        PingFrom,
        PingTo,
        Position,
        Prefer,
        PreferenceApplied,
        Priority,
        ProxyAuthenticate,
        ProxyAuthenticationInfo,
        ProxyAuthorization,
        ProxyStatus,
        PublicKeyPins,
        PublicKeyPinsReportOnly,
        Range,
        RedirectRef,
        Referer,
        Refresh,
        ReplayNonce,
        ReprDigest,
        RetryAfter,
        ScheduleReply,
        ScheduleTag,
        SecPurpose,
        SecTokenBinding,
        SecWebSocketAccept,
        SecWebSocketExtensions,
        SecWebSocketKey,
        SecWebSocketProtocol,
        SecWebSocketVersion,
        Server,
        ServerTiming,
        SetCookie,
        Signature,
        SignatureInput,
        SLUG,
        SoapAction,
        StatusURI,
        StrictTransportSecurity,
        Sunset,
        SurrogateCapability,
        SurrogateControl,
        TCN,
        TE,
        Timeout,
        Topic,
        Traceparent,
        Tracestate,
        Trailer,
        TransferEncoding,
        TTL,
        Upgrade,
        Urgency,
        UserAgent,
        VariantVary,
        Vary,
        Via,
        WantContentDigest,
        WantReprDigest,
        WWWAuthenticate,
        XContentTypeOptions,
        XFrameOptions,
        // IANA Deprecated status:
        AcceptCharset,
        CPEPInfo,
        Pragma,
        ProtocolInfo,
        ProtocolQuery,
    }

    extern "C++" {
        include!("cxx-qt-lib/qanystringview.h");
        type QAnyStringView<'a> = cxx_qt_lib::QAnyStringView<'a>;
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
        include!("cxx-qt-lib/qlist.h");
        type QList_QByteArray = cxx_qt_lib::QList<QByteArray>;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qtypes.h");
        type qsizetype = cxx_qt_lib::qsizetype;
    }

    extern "C++" {
        include!("cxx-qt-io/qlist_qpair_qbytearray_qbytearray.h");
        type QPair_QByteArray_QByteArray = crate::QPair<crate::QPairPair_QByteArray_QByteArray>;
        type QList_QPair_QByteArray_QByteArray = cxx_qt_lib::QList<QPair_QByteArray_QByteArray>;

        include!("cxx-qt-io/qhttpheaders.h");
        type QHttpHeadersWellKnownHeader;
    }

    unsafe extern "C++" {
        type QHttpHeaders = super::QHttpHeaders;

        #[doc(hidden)]
        #[rust_name = "append_by_name"]
        fn append(&mut self, name: QAnyStringView, value: QAnyStringView) -> bool;
        #[doc(hidden)]
        #[rust_name = "append_by_wellknown"]
        fn append(&mut self, name: QHttpHeadersWellKnownHeader, value: QAnyStringView) -> bool;

        /// Clears all header entries.
        fn clear(&mut self);

        #[doc(hidden)]
        #[rust_name = "combined_value_by_name"]
        fn combinedValue(&self, name: QAnyStringView) -> QByteArray;
        #[doc(hidden)]
        #[rust_name = "combined_value_by_wellknown"]
        fn combinedValue(&self, name: QHttpHeadersWellKnownHeader) -> QByteArray;

        #[doc(hidden)]
        #[rust_name = "contains_by_name"]
        fn contains(&self, name: QAnyStringView) -> bool;
        #[doc(hidden)]
        #[rust_name = "contains_by_wellknown"]
        fn contains(&self, name: QHttpHeadersWellKnownHeader) -> bool;

        #[doc(hidden)]
        #[rust_name = "insert_by_name"]
        fn insert(&mut self, i: qsizetype, name: QAnyStringView, value: QAnyStringView) -> bool;
        #[doc(hidden)]
        #[rust_name = "insert_by_wellknown"]
        fn insert(
            &mut self,
            i: qsizetype,
            name: QHttpHeadersWellKnownHeader,
            value: QAnyStringView,
        ) -> bool;

        /// Returns `true` if the headers have size 0; otherwise returns `false`.
        #[rust_name = "is_empty"]
        fn isEmpty(&self) -> bool;

        #[doc(hidden)]
        #[rust_name = "remove_all_by_name"]
        fn removeAll(&mut self, name: QAnyStringView);
        #[doc(hidden)]
        #[rust_name = "remove_all_by_wellknown"]
        fn removeAll(&mut self, name: QHttpHeadersWellKnownHeader);

        #[doc(hidden)]
        #[rust_name = "remove_at_qsizetype"]
        fn removeAt(&mut self, i: qsizetype);

        #[doc(hidden)]
        #[rust_name = "replace_by_name"]
        fn replace(&mut self, i: qsizetype, name: QAnyStringView, value: QAnyStringView) -> bool;
        #[doc(hidden)]
        #[rust_name = "replace_by_wellknown"]
        fn replace(
            &mut self,
            i: qsizetype,
            name: QHttpHeadersWellKnownHeader,
            value: QAnyStringView,
        ) -> bool;

        #[cfg(cxxqt_qt_version_at_least_6_8)]
        #[doc(hidden)]
        #[rust_name = "replace_or_append_by_name"]
        fn replaceOrAppend(&mut self, name: QAnyStringView, new_value: QAnyStringView) -> bool;
        #[cfg(cxxqt_qt_version_at_least_6_8)]
        #[doc(hidden)]
        #[rust_name = "replace_or_append_by_wellknown"]
        fn replaceOrAppend(
            &mut self,
            name: QHttpHeadersWellKnownHeader,
            new_value: QAnyStringView,
        ) -> bool;

        #[doc(hidden)]
        #[rust_name = "reserve_qsizetype"]
        fn reserve(&mut self, size: qsizetype);

        #[doc(hidden)]
        #[rust_name = "size_qsizetype"]
        fn size(&self) -> qsizetype;

        #[rust_name = "to_list_of_pairs"]
        fn toListOfPairs(&self) -> QList_QPair_QByteArray_QByteArray;

        #[doc(hidden)]
        #[rust_name = "values_by_name"]
        fn values(&self, name: QAnyStringView) -> QList_QByteArray;
        #[doc(hidden)]
        #[rust_name = "values_by_wellknown"]
        fn values(&self, name: QHttpHeadersWellKnownHeader) -> QList_QByteArray;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qhttpheaders_from_list_of_pairs"]
        fn qhttpheadersFromListOfPairs(pairs: &QList_QPair_QByteArray_QByteArray) -> QHttpHeaders;

        #[rust_name = "qhttpheaders_well_known_header_name"]
        fn qhttpheadersWellKnownHeaderName(name: QHttpHeadersWellKnownHeader) -> &'static [u8];

        #[rust_name = "qhttpheaders_name_at"]
        fn qhttpheadersNameAt(headers: &QHttpHeaders, i: isize) -> &str;

        #[rust_name = "qhttpheaders_value_by_name"]
        fn qhttpheadersValue<'a>(headers: &'a QHttpHeaders, name: QAnyStringView) -> &'a [u8];
        #[rust_name = "qhttpheaders_value_by_wellknown"]
        fn qhttpheadersValue(headers: &QHttpHeaders, name: QHttpHeadersWellKnownHeader) -> &[u8];

        #[rust_name = "qhttpheaders_value_at"]
        fn qhttpheadersValueAt(headers: &QHttpHeaders, i: isize) -> &[u8];
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhttpheaders_drop"]
        fn drop(headers: &mut QHttpHeaders);

        #[rust_name = "qhttpheaders_init_default"]
        fn construct() -> QHttpHeaders;
        #[rust_name = "qhttpheaders_clone"]
        fn construct(other: &QHttpHeaders) -> QHttpHeaders;
        #[rust_name = "qhttpheaders_to_debug_qstring"]
        fn toDebugQString(value: &QHttpHeaders) -> QString;
    }
}

pub use ffi::QHttpHeadersWellKnownHeader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpHeader<'a> {
    Named(QAnyStringView<'a>),
    WellKnown(QHttpHeadersWellKnownHeader),
}

impl From<QHttpHeadersWellKnownHeader> for HttpHeader<'_> {
    fn from(value: QHttpHeadersWellKnownHeader) -> Self {
        Self::WellKnown(value)
    }
}

impl<'a, T> From<T> for HttpHeader<'a>
where
    T: Into<QAnyStringView<'a>>,
{
    fn from(value: T) -> Self {
        Self::Named(value.into())
    }
}

/// `QHttpHeaders` is a class for holding HTTP headers.
///
/// Qt Documentation: [QHttpHeaders](https://doc.qt.io/qt-6/qhttpheaders.html#details)
#[repr(C)]
pub struct QHttpHeaders {
    _space: MaybeUninit<usize>,
}

impl Clone for QHttpHeaders {
    fn clone(&self) -> Self {
        ffi::qhttpheaders_clone(self)
    }
}

impl Default for QHttpHeaders {
    fn default() -> Self {
        ffi::qhttpheaders_init_default()
    }
}

impl Drop for QHttpHeaders {
    fn drop(&mut self) {
        ffi::qhttpheaders_drop(self);
    }
}

impl Debug for QHttpHeaders {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", ffi::qhttpheaders_to_debug_qstring(self))
    }
}

impl QHttpHeaders {
    /// Returns a header name corresponding to the provided `name` as a view.
    pub fn well_known_header_name(name: QHttpHeadersWellKnownHeader) -> &'static [u8] {
        ffi::qhttpheaders_well_known_header_name(name)
    }

    /// Appends a header entry with `name` and `value` and returns `true` if successful.
    pub fn append<'a, N, V>(&mut self, name: N, value: V) -> bool
    where
        N: Into<HttpHeader<'a>>,
        V: Into<QAnyStringView<'a>>,
    {
        // Reduce monomorphization
        fn inner(headers: &mut QHttpHeaders, name: HttpHeader, value: QAnyStringView) -> bool {
            match name {
                HttpHeader::Named(name) => headers.append_by_name(name, value),
                HttpHeader::WellKnown(name) => headers.append_by_wellknown(name, value),
            }
        }
        inner(self, name.into(), value.into())
    }

    /// Returns the values of header `name` in a comma-combined string. Returns a null `QByteArray` if the header with `name` doesn't exist.
    ///
    /// **Note:** Accessing the value(s) of 'Set-Cookie' header this way may not work as intended. It is a notable exception in the HTTP RFC in that its values cannot be combined this way. Prefer `values()` instead.
    pub fn combined_value<'a, N>(&self, name: N) -> QByteArray
    where
        N: Into<HttpHeader<'a>>,
    {
        match name.into() {
            HttpHeader::Named(name) => self.combined_value_by_name(name),
            HttpHeader::WellKnown(name) => self.combined_value_by_wellknown(name),
        }
    }

    /// Returns whether the headers contain header with `name`.
    pub fn contains<'a, N>(&self, name: N) -> bool
    where
        N: Into<HttpHeader<'a>>,
    {
        match name.into() {
            HttpHeader::Named(name) => self.contains_by_name(name),
            HttpHeader::WellKnown(name) => self.contains_by_wellknown(name),
        }
    }

    /// Inserts a header entry at index `i`, with `name` and `value`. The index must be valid (see `len()`). Returns whether the insert succeeded.
    pub fn insert<'a, N, V>(&mut self, i: isize, name: N, value: V) -> bool
    where
        N: Into<HttpHeader<'a>>,
        V: Into<QAnyStringView<'a>>,
    {
        // Reduce monomorphization
        fn inner(
            headers: &mut QHttpHeaders,
            i: isize,
            name: HttpHeader,
            value: QAnyStringView,
        ) -> bool {
            match name {
                HttpHeader::Named(name) => headers.insert_by_name(i.into(), name, value),
                HttpHeader::WellKnown(name) => headers.insert_by_wellknown(i.into(), name, value),
            }
        }
        inner(self, i, name.into(), value.into())
    }

    /// Returns the header name at index `i`, or `None` if `i` is not valid (see `len()`).
    ///
    /// Header names are case-insensitive, and the returned names are lower-cased.
    pub fn name_at(&self, i: isize) -> Option<&str> {
        if (0..self.len()).contains(&i) {
            Some(ffi::qhttpheaders_name_at(self, i))
        } else {
            None
        }
    }

    /// Removes the header `name`.
    pub fn remove_all<'a, N>(&mut self, name: N)
    where
        N: Into<HttpHeader<'a>>,
    {
        match name.into() {
            HttpHeader::Named(name) => self.remove_all_by_name(name),
            HttpHeader::WellKnown(name) => self.remove_all_by_wellknown(name),
        }
    }

    /// Removes the header at index `i`. The index `i` must be valid (see `len()`).
    pub fn remove_at(&mut self, i: isize) {
        self.remove_at_qsizetype(i.into());
    }

    /// If `QHttpHeaders` already contains `name`, replaces its value with `new_value` and removes possible additional `name` entries. If `name` didn't exist, appends a new entry. Returns `true` if successful.
    ///
    /// This function is a convenience method for setting a unique `name` : `new_value` header. For most headers the relative order does not matter, which allows reusing an existing entry if one exists.
    #[cfg(cxxqt_qt_version_at_least_6_8)]
    pub fn replace_or_append<'a, N, V>(&mut self, name: N, value: V) -> bool
    where
        N: Into<HttpHeader<'a>>,
        V: Into<QAnyStringView<'a>>,
    {
        // Reduce monomorphization
        fn inner(headers: &mut QHttpHeaders, name: HttpHeader, value: QAnyStringView) -> bool {
            match name {
                HttpHeader::Named(name) => headers.replace_or_append_by_name(name, value),
                HttpHeader::WellKnown(name) => headers.replace_or_append_by_wellknown(name, value),
            }
        }
        inner(self, name.into(), value.into())
    }

    /// Attempts to allocate memory for at least `size` header entries.
    ///
    /// If you know in advance how how many header entries there will be, you may call this function to prevent reallocations and memory fragmentation.
    pub fn reserve(&mut self, size: isize) {
        self.reserve_qsizetype(size.into());
    }

    /// Returns the number of header entries.
    pub fn len(&self) -> isize {
        self.size_qsizetype().into()
    }

    /// Returns the value of the (first) header `name`, or `None` if it doesn't exist.
    pub fn value<'a, N>(&self, name: N) -> Option<&[u8]>
    where
        N: Into<HttpHeader<'a>>,
    {
        // Reduce monomorphization
        fn inner<'a>(headers: &'a QHttpHeaders, name: HttpHeader) -> Option<&'a [u8]> {
            match name {
                HttpHeader::Named(name) => {
                    if headers.contains_by_name(name.clone()) {
                        Some(ffi::qhttpheaders_value_by_name(headers, name))
                    } else {
                        None
                    }
                }
                HttpHeader::WellKnown(name) => {
                    if headers.contains_by_wellknown(name) {
                        Some(ffi::qhttpheaders_value_by_wellknown(headers, name))
                    } else {
                        None
                    }
                }
            }
        }
        inner(self, name.into())
    }

    /// Returns the header value at index `i`, or `None` if `i` is not valid (see `len()`).
    pub fn value_at(&self, i: isize) -> Option<&[u8]> {
        if (0..self.len()).contains(&i) {
            Some(ffi::qhttpheaders_value_at(self, i))
        } else {
            None
        }
    }

    /// Returns the values of header `name` in a list. Returns an empty list if header with `name` doesn't exist.
    pub fn values<'a, N>(&self, name: N) -> QList<QByteArray>
    where
        N: Into<HttpHeader<'a>>,
    {
        match name.into() {
            HttpHeader::Named(name) => self.values_by_name(name),
            HttpHeader::WellKnown(name) => self.values_by_wellknown(name),
        }
    }
}

impl From<&QList<QPair<QPairPair_QByteArray_QByteArray>>> for QHttpHeaders {
    fn from(value: &QList<QPair<QPairPair_QByteArray_QByteArray>>) -> Self {
        ffi::qhttpheaders_from_list_of_pairs(value)
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QHttpHeaders {
    type Id = type_id!("QHttpHeaders");
    type Kind = cxx::kind::Trivial;
}

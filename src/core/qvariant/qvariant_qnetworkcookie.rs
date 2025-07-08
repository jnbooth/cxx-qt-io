//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qvariant/generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-io/qvariant_qnetworkcookie.h");
        type QNetworkCookie = crate::QNetworkCookie;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "qvariant_can_convert_QNetworkCookie"]
        fn qvariantCanConvertQNetworkCookie(variant: &QVariant) -> bool;
        #[rust_name = "qvariant_construct_QNetworkCookie"]
        fn qvariantConstruct(value: &QNetworkCookie) -> QVariant;
        #[rust_name = "qvariant_value_or_default_QNetworkCookie"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QNetworkCookie;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::qvariant_can_convert_QNetworkCookie(variant)
}

pub(crate) fn construct(value: &ffi::QNetworkCookie) -> ffi::QVariant {
    ffi::qvariant_construct_QNetworkCookie(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QNetworkCookie {
    ffi::qvariant_value_or_default_QNetworkCookie(variant)
}

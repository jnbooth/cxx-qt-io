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
        #[rust_name = "can_convert_QNetworkCookie"]
        fn qvariantCanConvertQNetworkCookie(variant: &QVariant) -> bool;
        #[rust_name = "construct_QNetworkCookie"]
        fn qvariantConstruct(value: &QNetworkCookie) -> QVariant;
        #[rust_name = "value_or_default_QNetworkCookie"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QNetworkCookie;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QNetworkCookie(variant)
}

pub(crate) fn construct(value: &ffi::QNetworkCookie) -> ffi::QVariant {
    ffi::construct_QNetworkCookie(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QNetworkCookie {
    ffi::value_or_default_QNetworkCookie(variant)
}

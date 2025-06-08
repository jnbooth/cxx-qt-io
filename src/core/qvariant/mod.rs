#![allow(unused)]
use cxx_qt_lib::{QVariant, QVariantValue};

macro_rules! impl_qvariant_value {
    ( $typeName:ident, $module:ident ) => {
        mod $module;

        impl QVariantValue for crate::$typeName {
            fn can_convert(variant: &QVariant) -> bool {
                $module::can_convert(variant)
            }

            fn construct(value: &Self) -> QVariant {
                $module::construct(value)
            }

            fn value_or_default(variant: &QVariant) -> Self {
                $module::value_or_default(variant)
            }
        }
    };
}

#[cfg(feature = "request")]
impl_qvariant_value!(QNetworkCookie, qvariant_qnetworkcookie);

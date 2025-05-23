//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qpair/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qpair_qbytearray_qbytearray.h");
        type QPair_QByteArray_QByteArray = crate::QPair<super::QPairPair_QByteArray_QByteArray>;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qpair_drop_QByteArray_QByteArray"]
        fn drop(_: &mut QPair_QByteArray_QByteArray);
    }
}

#[allow(non_camel_case_types)]
pub struct QPairPair_QByteArray_QByteArray;

pub(crate) fn drop(pair: &mut ffi::QPair_QByteArray_QByteArray) {
    ffi::qpair_drop_QByteArray_QByteArray(pair);
}

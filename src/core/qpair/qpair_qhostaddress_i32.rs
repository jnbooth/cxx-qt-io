//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qpair/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qpair_qhostaddress_i32.h");
        type QPair_QHostAddress_i32 = crate::QPair<super::QPairPair_QHostAddress_i32>;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qpair_drop_QHostAddress_i32"]
        fn drop(_: &mut QPair_QHostAddress_i32);
    }
}

#[allow(non_camel_case_types)]
pub struct QPairPair_QHostAddress_i32;

pub(crate) fn drop(pair: &mut ffi::QPair_QHostAddress_i32) {
    ffi::qpair_drop_QHostAddress_i32(pair);
}

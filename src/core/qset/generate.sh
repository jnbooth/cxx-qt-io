#!/usr/bin/env bash

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_bridge() {
    local LOWER
    LOWER=$(echo "$1" | tr '[:upper:]' '[:lower:]')

    tee "$SCRIPTPATH/../../../include/core/qset/qset_$LOWER.h" <<EOF
//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qset/generate.sh
#pragma once
#include "qset_private.h"
#include <cxx-qt-io/$LOWER.h>

using QSet_$1 = QSet<$1>;
EOF
    clang-format -i "$SCRIPTPATH/../../../include/core/qset/qset_$LOWER.h"

    tee "$SCRIPTPATH/qset_$LOWER.rs" <<EOF
//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qset/generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-io/$LOWER.h");
        type $1 = crate::$1;

        include!("cxx-qt-io/qset_$LOWER.h");
        type QSet_$1 = cxx_qt_lib::QSet<$1>;
    }

    #[namespace = "rust::cxxqtio1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_clear_$1"]
        fn qsetClear(set: &mut QSet_$1);
        #[rust_name = "qset_contains_$1"]
        fn qsetContains(
            set: &QSet_$1,
            _: &$1,
        ) -> bool;
        #[rust_name = "qset_remove_$1"]
        fn qsetRemove(
            set: &mut QSet_$1,
            _: &$1,
        ) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_$1"]
        fn construct(_: &QSet_$1) -> QSet_$1;
        #[rust_name = "qset_default_$1"]
        fn construct() -> QSet_$1;
        #[rust_name = "qset_drop_$1"]
        fn drop(_: &mut QSet_$1);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "qset_get_unchecked_$1"]
        unsafe fn qsetGetUnchecked(set: &QSet_$1, pos: isize) -> &$1;
        #[rust_name = "qset_insert_$1"]
        fn qsetInsert(_: &mut QSet_$1, _: &$1);
        #[rust_name = "qset_len_$1"]
        fn qsetLen(_: &QSet_$1) -> isize;
        #[rust_name = "qset_reserve_$1"]
        fn qsetReserve(_: &mut QSet_$1, size: isize);
    }
}

pub(crate) fn clear(v: &mut ffi::QSet_$1) {
    ffi::qset_clear_$1(v);
}

pub(crate) fn contains(
    v: &ffi::QSet_$1,
    item: &ffi::$1,
) -> bool {
    ffi::qset_contains_$1(v, item)
}

pub(crate) fn remove(
    v: &mut ffi::QSet_$1,
    item: &ffi::$1,
) -> bool {
    ffi::qset_remove_$1(v, item)
}

pub(crate) fn clone(s: &ffi::QSet_$1) -> ffi::QSet_$1 {
    ffi::qset_clone_$1(s)
}

pub(crate) fn default() -> ffi::QSet_$1 {
    ffi::qset_default_$1()
}

pub(crate) fn drop(s: &mut ffi::QSet_$1) {
    ffi::qset_drop_$1(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_$1, pos: isize) -> &ffi::$1 {
    unsafe { ffi::qset_get_unchecked_$1(s, pos) }
}

pub(crate) fn insert(s: &mut ffi::QSet_$1, value: &ffi::$1) {
    ffi::qset_insert_$1(s, value);
}

pub(crate) fn len(s: &ffi::QSet_$1) -> isize {
    ffi::qset_len_$1(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_$1, size: isize) {
  ffi::qset_reserve_$1(s, size);
}
EOF
    rustfmt +nightly "$SCRIPTPATH/qset_$LOWER.rs"
}

generate_bridge "QHostAddress"
generate_bridge "QHttp1Configuration"
generate_bridge "QOcspResponse"
generate_bridge "QSslCertificate"
generate_bridge "QSslDiffieHellmanParameters"
generate_bridge "QSslEllipticCurve"
generate_bridge "QSslError"

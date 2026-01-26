#!/usr/bin/env bash

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_qhash_header(){
    local LOWER
    LOWER="$(echo "$1" | tr '[:upper:]' '[:lower:]')"

    local INCLUDE_1=""
    local INCLUDE_2=""

    if [[ -n $4 ]]; then
      INCLUDE_1="#include <$4>"
    fi

    if [[ -n $5 ]]; then
      INCLUDE_2="#include <$5>"
    fi

  tee "$SCRIPTPATH/../../../include/core/qhash/qhash_$LOWER.h" <<EOF
//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qhash/generate.sh
#pragma once
#include "qhash_private.h"
$INCLUDE_1
$INCLUDE_2
using QHash_$1 = QHash<$2, $3>;
EOF
  clang-format -i "$SCRIPTPATH/../../../include/core/qhash/qhash_$LOWER.h"
}

function generate_bridge() {
    local K="$1"
    local V="$2"
    local SUFFIX="${1}_${2}"
    local LOWER
    LOWER="$(echo "$SUFFIX" | tr '[:upper:]' '[:lower:]')"
    local QHASH="QHash_$SUFFIX"
    local QHASHPAIR="QHashPair_$SUFFIX"

    local FK=$K
    local FV=$V
    local INCLUDE_K=""
    local INCLUDE_V=""

    if [[ -n $3 ]]; then
      FK="ffi::$K"
      INCLUDE_K="include!(\"cxx-qt-lib/$3.h\");type $K = cxx_qt_lib::$K;"
    fi

    if [[ -n $4 ]]; then
      FV="ffi::$V"
      INCLUDE_V="include!(\"cxx-qt-lib/$4.h\");type $V = cxx_qt_lib::$V;"
    fi

    tee "$SCRIPTPATH/qhash_$LOWER.rs" <<EOF
//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qhash/generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        $INCLUDE_K $INCLUDE_V

        include!("cxx-qt-io/qhash_$LOWER.h");
        type $QHASH = cxx_qt_lib::QHash<super::$QHASHPAIR>;
    }

    #[namespace = "rust::cxxqtio1::qhash"]
    unsafe extern "C++" {
        #[rust_name = "qhash_clear_$SUFFIX"]
        fn qhashClear(hash: &mut $QHASH);
        #[rust_name = "qhash_contains_$SUFFIX"]
        fn qhashContains(
            hash: &$QHASH,
            _: &$K,
        ) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhash_clone_$SUFFIX"]
        fn construct(_: &$QHASH) -> $QHASH;
        #[rust_name = "qhash_default_$SUFFIX"]
        fn construct() -> $QHASH;
        #[rust_name = "qhash_drop_$SUFFIX"]
        fn drop(_: &mut $QHASH);
    }

    #[namespace = "rust::cxxqtlib1::qhash"]
    unsafe extern "C++" {
        #[rust_name = "qhash_get_or_default_$SUFFIX"]
        fn qhashGetOrDefault(_: &$QHASH, key: &$K) -> $V;
        #[rust_name = "qhash_get_unchecked_key_$SUFFIX"]
        unsafe fn qhashGetUncheckedKey<'a>(_: &'a $QHASH, pos: isize) -> &'a $K;
        #[rust_name = "qhash_get_unchecked_value_$SUFFIX"]
        unsafe fn qhashGetUncheckedValue(_: &$QHASH, pos: isize) -> &$V;
        #[rust_name = "qhash_insert_$SUFFIX"]
        fn qhashInsert(_: &mut $QHASH, key: &$K, value: &$V);
        #[rust_name = "qhash_len_$SUFFIX"]
        fn qhashLen(_: &$QHASH) -> isize;
        #[rust_name = "qhash_remove_$SUFFIX"]
        fn qhashRemove(_: &mut $QHASH, key: &$K) -> bool;
    }
}

pub(crate) fn clear(hash: &mut ffi::$QHASH) {
    ffi::qhash_clear_$SUFFIX(hash);
}

pub(crate) fn contains(
    hash: &ffi::$QHASH,
    k: &$FK,
) -> bool {
    ffi::qhash_contains_$SUFFIX(hash, k)
}

pub(crate) fn clone(hash: &ffi::$QHASH) -> ffi::$QHASH {
    ffi::qhash_clone_$SUFFIX(hash)
}

pub(crate) fn default() -> ffi::$QHASH {
    ffi::qhash_default_$SUFFIX()
}

pub(crate) fn drop(hash: &mut ffi::$QHASH) {
    ffi::qhash_drop_$SUFFIX(hash);
}

pub(crate) fn get_or_default(hash: &ffi::$QHASH, key: &$FK) -> $FV {
    ffi::qhash_get_or_default_$SUFFIX(hash, key)
}

pub(crate) unsafe fn get_unchecked_key(hash: &ffi::$QHASH, pos: isize) -> &$FK {
    unsafe { ffi::qhash_get_unchecked_key_$SUFFIX(hash, pos) }
}

pub(crate) unsafe fn get_unchecked_value(
    hash: &ffi::$QHASH,
    pos: isize,
) -> &$FV {
    unsafe { ffi::qhash_get_unchecked_value_$SUFFIX(hash, pos) }
}

pub(crate) fn insert(hash: &mut ffi::$QHASH, key: &$FK, value: &$FV) {
    ffi::qhash_insert_$SUFFIX(hash, key, value);
}

pub(crate) fn len(hash: &ffi::$QHASH) -> isize {
    ffi::qhash_len_$SUFFIX(hash)
}

pub(crate) fn remove(hash: &mut ffi::$QHASH, key: &$FK) -> bool {
    ffi::qhash_remove_$SUFFIX(hash, key)
}

pub struct QHashPair_$SUFFIX;

#[cfg(test)]
mod tests {
    #[test]
    fn len() {
        let empty = super::default();
        assert_eq!(super::len(&empty), 0);
        std::mem::drop(empty);
    }
}
EOF
    rustfmt +nightly "$SCRIPTPATH/qhash_$LOWER.rs"
}

generate_qhash_header "i32_QVariant" "::std::int32_t" "::QVariant" "" "QtCore/QVariant"

generate_bridge "i32" "QVariant" "" "qvariant"

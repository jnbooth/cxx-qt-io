#!/usr/bin/env bash

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_qpair_header(){
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

    tee "$SCRIPTPATH/../../../include/core/qpair/qpair_$LOWER.h" <<EOF
//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qpair/generate.sh
#pragma once
#include "qpair_private.h"
$INCLUDE_1
$INCLUDE_2
using QPair_$1 = ::std::pair<$2, $3>;
EOF
}

function generate_bridge() {
    local SUFFIX="${1}_${2}"
    local LOWER
    LOWER="$(echo "$SUFFIX" | tr '[:upper:]' '[:lower:]')"
    local QPAIR="QPair_$SUFFIX"
    local QPAIRPAIR="QPairPair_$SUFFIX"

    tee "$SCRIPTPATH/qpair_$LOWER.rs" <<EOF
//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qpair/generate.sh

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        include!("cxx-qt-io/qpair_$LOWER.h");
        type $QPAIR = crate::QPair<super::$QPAIRPAIR>;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qpair_drop_$SUFFIX"]
        fn drop(_: &mut $QPAIR);
    }
}

#[allow(non_camel_case_types)]
pub struct $QPAIRPAIR;

pub(crate) fn drop(pair: &mut ffi::$QPAIR) {
    ffi::qpair_drop_$SUFFIX(pair);
}
EOF
    rustfmt "$SCRIPTPATH/qpair_$LOWER.rs"
}

generate_qpair_header "QByteArray_QByteArray" "::QByteArray" "::QByteArray" "QtCore/QByteArray" ""
generate_bridge "QByteArray" "QByteArray"

generate_qpair_header "QHostAddress_i32" "::QHostAddress" "::std::int32_t" "QtNetwork/QHostAddress" ""
generate_bridge "QHostAddress" "i32"

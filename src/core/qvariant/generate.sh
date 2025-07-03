#!/usr/bin/env bash

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_bridge_qt() {
    local LOWER
    LOWER=$(echo "$1" | tr '[:upper:]' '[:lower:]')


    tee "$SCRIPTPATH/../../../include/core/qvariant/qvariant_$LOWER.h" <<EOF
//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qvariant/generate.sh

#pragma once
#include <QtCore/QVariant>
#include <cxx-qt-io/$LOWER.h>

namespace rust {
namespace cxxqtlib1 {
namespace qvariant {
bool qvariantCanConvert$1(const QVariant& variant);
}
}
}
EOF

    tee "$SCRIPTPATH/qvariant_$LOWER.rs" <<EOF
//! This is an auto-generated file. Do not edit.
//! Edit instead: src/core/qvariant/generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;

        include!("cxx-qt-io/qvariant_$LOWER.h");
        type $1 = crate::$1;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_$1"]
        fn qvariantCanConvert$1(variant: &QVariant) -> bool;
        #[rust_name = "construct_$1"]
        fn qvariantConstruct(value: &$1) -> QVariant;
        #[rust_name = "value_or_default_$1"]
        fn qvariantValueOrDefault(variant: &QVariant) -> $1;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_$1(variant)
}

pub(crate) fn construct(value: &ffi::$1) -> ffi::QVariant {
    ffi::construct_$1(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::$1 {
    ffi::value_or_default_$1(variant)
}
EOF
    rustfmt +nightly "$SCRIPTPATH/qvariant_$LOWER.rs"
}

generate_bridge_qt "QNetworkCookie"

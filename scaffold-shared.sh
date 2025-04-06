#!/usr/bin/env bash

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

MODULE="$1"
MODULE_LOWER=$(echo "$MODULE" | tr '[:upper:]' '[:lower:]')
CLASS="$2"
CLASS_LOWER=$(echo "$CLASS" | tr '[:upper:]' '[:lower:]')
NOUN="$3"

tee "$SCRIPTPATH/include/$MODULE_LOWER/$CLASS_LOWER.h" <<EOF
#pragma once

#include <Qt$MODULE/$CLASS>

#include "rust/cxx.h"

namespace rust {
template<>
struct IsRelocatable<$CLASS> : ::std::true_type
{};
}

EOF

tee "$SCRIPTPATH/src/$MODULE_LOWER/$CLASS_LOWER.cpp" <<EOF
#include "cxx-qt-io/$CLASS_LOWER.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type($CLASS);

EOF

tee "$SCRIPTPATH/src/$MODULE_LOWER/$CLASS_LOWER.rs" <<EOF
use std::fmt::{self, Debug, Formatter};
use std::mem::MaybeUninit;

use cxx::{type_id, ExternType};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "C++" {
        include!("cxx-qt-io/$CLASS_LOWER.h");
    }

    unsafe extern "C++" {
        type $CLASS = super::$CLASS;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "${CLASS_LOWER}_drop"]
        fn drop($NOUN: &mut $CLASS);

        #[rust_name = "${CLASS_LOWER}_init_default"]
        fn construct() -> $CLASS;
        #[rust_name = "${CLASS_LOWER}_clone"]
        fn construct(other: &$CLASS) -> $CLASS;

        #[rust_name = "${CLASS_LOWER}_eq"]
        fn operatorEq(a: &$CLASS, b: &$CLASS) -> bool;

        #[rust_name = "${CLASS_LOWER}_to_debug_qstring"]
        fn toDebugQString(value: &$CLASS) -> QString;
    }
}

/// The \`$CLASS\` class
///
/// Qt Documentation: [$CLASS](https://doc.qt.io/qt-6/$CLASS_LOWER.html#details)
#[repr(C)]
pub struct $CLASS {
    _space: MaybeUninit<usize>,
}

impl Clone for $CLASS {
    fn clone(&self) -> Self {
        ffi::${CLASS_LOWER}_clone(self)
    }
}

impl Default for $CLASS {
    fn default() -> Self {
        ffi::${CLASS_LOWER}_init_default()
    }
}

impl Drop for $CLASS {
    fn drop(&mut self) {
        ffi::${CLASS_LOWER}_drop(self);
    }
}

impl PartialEq for $CLASS {
    fn eq(&self, other: &Self) -> bool {
        ffi::${CLASS_LOWER}_eq(self, other)
    }
}

impl Eq for $CLASS {}

impl Debug for $CLASS {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", ffi::${CLASS_LOWER}_to_debug_qstring(self))
    }
}

impl $CLASS {
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for $CLASS {
    type Id = type_id!("$CLASS");
    type Kind = cxx::kind::Trivial;
}

EOF

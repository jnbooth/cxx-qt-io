// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-io/qpair.h"

#include <QtCore/QByteArray>

#include <cxx-qt-lib/assertion_utils.h>

#define CXX_QT_IO_QPAIR_ASSERTS(firstTypeName, secondTypeName)                 \
  using QPair_##firstTypeName##_##secondTypeName =                             \
    ::std::pair<firstTypeName, secondTypeName>;                                \
                                                                               \
  assert_alignment_and_size(QPair_##firstTypeName##_##secondTypeName, {        \
    firstTypeName first;                                                       \
    secondTypeName second;                                                     \
  });                                                                          \
                                                                               \
  static_assert(!::std::is_trivially_copy_assignable<                          \
                QPair_##firstTypeName##_##secondTypeName>::value);             \
  static_assert(!::std::is_trivially_copy_constructible<                       \
                QPair_##firstTypeName##_##secondTypeName>::value);             \
  static_assert(!::std::is_trivially_destructible<                             \
                QPair_##firstTypeName##_##secondTypeName>::value);             \
                                                                               \
  static_assert(                                                               \
    QTypeInfo<QPair_##firstTypeName##_##secondTypeName>::isRelocatable);       \
                                                                               \
  static_assert(::std::is_copy_assignable<firstTypeName>::value);              \
  static_assert(::std::is_copy_constructible<firstTypeName>::value);           \
  static_assert(::std::is_copy_assignable<secondTypeName>::value);             \
  static_assert(::std::is_copy_constructible<secondTypeName>::value);

CXX_QT_IO_QPAIR_ASSERTS(QByteArray, QByteArray);

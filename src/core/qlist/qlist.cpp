// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-io/qlist.h"

#include <cxx-qt-lib/assertion_utils.h>

#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
#define CXX_QT_IO_QLIST_ALIGN_AND_SIZE(name)                                   \
  assert_alignment_and_size(QList_##name, {                                    \
    ::std::size_t a0;                                                          \
    ::std::size_t a1;                                                          \
    ::std::size_t a2;                                                          \
  });
#else
#define CXX_QT_IO_QLIST_ALIGN_AND_SIZE(name)                                   \
  assert_alignment_and_size(QList_##name, { ::std::size_t a0; });
#endif

#define CXX_QT_IO_QLIST_ASSERTS(name)                                          \
  CXX_QT_IO_QLIST_ALIGN_AND_SIZE(name);                                        \
                                                                               \
  static_assert(!::std::is_trivially_copy_assignable<QList_##name>::value);    \
  static_assert(!::std::is_trivially_copy_constructible<QList_##name>::value); \
  static_assert(!::std::is_trivially_destructible<QList_##name>::value);       \
                                                                               \
  static_assert(QTypeInfo<QList_##name>::isRelocatable);                       \
                                                                               \
  static_assert(::std::is_copy_assignable<name>::value);                       \
  static_assert(::std::is_copy_constructible<name>::value);

CXX_QT_IO_QLIST_ASSERTS(QPair_QByteArray_QByteArray);

#ifdef CXX_QT_IO_NETWORK_FEATURE
CXX_QT_IO_QLIST_ASSERTS(QHostAddress);
CXX_QT_IO_QLIST_ASSERTS(QNetworkAddressEntry);
CXX_QT_IO_QLIST_ASSERTS(QNetworkInterface);
#endif

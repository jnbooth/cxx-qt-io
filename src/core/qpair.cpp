#include "cxx-qt-io/qpair.h"

#include <cxx-qt-io/assertion_utils.h>

#define CXX_QT_IO_QPAIR_ASSERTS(firstTypeName, secondTypeName, combinedName)   \
  assert_alignment_and_size(QPair_##combinedName, {                            \
    firstTypeName first;                                                       \
    secondTypeName second;                                                     \
  });                                                                          \
                                                                               \
  static_assert(QTypeInfo<QPair_##combinedName>::isRelocatable);               \
                                                                               \
  static_assert(::std::is_copy_assignable<firstTypeName>::value);              \
  static_assert(::std::is_copy_constructible<firstTypeName>::value);           \
  static_assert(::std::is_move_constructible<firstTypeName>::value);           \
  static_assert(::std::is_copy_assignable<secondTypeName>::value);             \
  static_assert(::std::is_copy_constructible<secondTypeName>::value);          \
  static_assert(::std::is_move_constructible<secondTypeName>::value);

CXX_QT_IO_QPAIR_ASSERTS(QByteArray, QByteArray, QByteArray_QByteArray);

#ifdef CXX_QT_IO_NET_FEATURE
CXX_QT_IO_QPAIR_ASSERTS(QHostAddress, ::std::int32_t, QHostAddress_i32);
#endif

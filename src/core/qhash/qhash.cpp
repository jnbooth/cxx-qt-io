#include "cxx-qt-io/qhash.h"

#include <cxx-qt-io/assertion_utils.h>

#define CXX_QT_IO_QHASH_ASSERTS(keyTypeName, valueTypeName, combinedName)      \
  assert_alignment_and_size(QHash_##combinedName, { ::std::size_t a0; });      \
                                                                               \
  static_assert(                                                               \
    !::std::is_trivially_copy_assignable<QHash_##combinedName>::value);        \
  static_assert(                                                               \
    !::std::is_trivially_copy_constructible<QHash_##combinedName>::value);     \
  static_assert(                                                               \
    !::std::is_trivially_destructible<QHash_##combinedName>::value);           \
                                                                               \
  static_assert(QTypeInfo<QHash_##combinedName>::isRelocatable);               \
                                                                               \
  static_assert(::std::is_copy_assignable<keyTypeName>::value);                \
  static_assert(::std::is_copy_constructible<keyTypeName>::value);             \
  static_assert(::std::is_copy_assignable<valueTypeName>::value);              \
  static_assert(::std::is_copy_constructible<valueTypeName>::value);

CXX_QT_IO_QHASH_ASSERTS(::std::int32_t, QVariant, i32_QVariant);

#ifdef CXX_QT_IO_REQUEST_FEATURE
CXX_QT_IO_QHASH_ASSERTS(QNetworkRequest::Attribute,
                        QVariant,
                        QNetworkRequestAttribute_QVariant);
#endif

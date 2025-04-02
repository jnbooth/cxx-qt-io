#include "cxx-qt-io/qset.h"

#include <cxx-qt-io/assertion_utils.h>

#define CXX_QT_IO_QSET_ASSERTS(name)                                           \
  assert_alignment_and_size(QSet_##name, { ::std::size_t a0; });               \
                                                                               \
  static_assert(!::std::is_trivially_copy_assignable<QSet_##name>::value);     \
  static_assert(!::std::is_trivially_copy_constructible<QSet_##name>::value);  \
  static_assert(!::std::is_trivially_destructible<QSet_##name>::value);        \
                                                                               \
  static_assert(QTypeInfo<QSet_##name>::isRelocatable);                        \
                                                                               \
  static_assert(::std::is_copy_assignable<name>::value);                       \
  static_assert(::std::is_copy_constructible<name>::value);

#ifdef CXX_QT_IO_SSL_FEATURE
CXX_QT_IO_QSET_ASSERTS(QOcspResponse);
CXX_QT_IO_QSET_ASSERTS(QSslCertificate);
CXX_QT_IO_QSET_ASSERTS(QSslEllipticCurve);
CXX_QT_IO_QSET_ASSERTS(QSslError);
#endif

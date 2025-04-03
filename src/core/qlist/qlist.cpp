#include "cxx-qt-io/qlist.h"

#include <cxx-qt-io/assertion_utils.h>

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
CXX_QT_IO_QLIST_ASSERTS(QNetworkCookie);
CXX_QT_IO_QLIST_ASSERTS(QNetworkDatagram);
CXX_QT_IO_QLIST_ASSERTS(QNetworkInterface);
CXX_QT_IO_QLIST_ASSERTS(QNetworkProxy);

#if (QT_VERSION >= QT_VERSION_CHECK(6, 7, 0))
CXX_QT_IO_QLIST_ASSERTS(QHttpHeaders);
#endif

#ifdef CXX_QT_IO_SSL_FEATURE
CXX_QT_IO_QLIST_ASSERTS(QOcspResponse);
CXX_QT_IO_QLIST_ASSERTS(QSslCertificate);
CXX_QT_IO_QLIST_ASSERTS(QSslCertificateExtension);
CXX_QT_IO_QLIST_ASSERTS(QSslConfiguration);
CXX_QT_IO_QLIST_ASSERTS(QSslError);
CXX_QT_IO_QLIST_ASSERTS(QSslKey);
CXX_QT_IO_QLIST_ASSERTS(QSslPreSharedKeyAuthenticator);
#endif

#endif

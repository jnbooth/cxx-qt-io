#include "cxx-qt-io/qnetworkaddressentry.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QNetworkAddressEntry, { ::std::size_t a0; });

static_assert(
  !::std::is_trivially_copy_assignable<QNetworkAddressEntry>::value);
static_assert(
  !::std::is_trivially_copy_constructible<QNetworkAddressEntry>::value);

static_assert(!::std::is_trivially_destructible<QNetworkAddressEntry>::value);

static_assert(QTypeInfo<QNetworkAddressEntry>::isRelocatable);

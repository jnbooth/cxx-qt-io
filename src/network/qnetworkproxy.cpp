#include "cxx-qt-io/qnetworkproxy.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QNetworkProxy, { ::std::size_t a0; });

static_assert(!::std::is_trivially_copy_assignable<QNetworkProxy>::value);
static_assert(!::std::is_trivially_copy_constructible<QNetworkProxy>::value);

static_assert(!::std::is_trivially_destructible<QNetworkProxy>::value);

static_assert(QTypeInfo<QNetworkProxy>::isRelocatable);

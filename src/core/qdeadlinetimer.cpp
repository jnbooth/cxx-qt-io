#include "cxx-qt-io/qdeadlinetimer.h"

#include <cxx-qt-io/assertion_utils.h>

assert_alignment_and_size(QDeadlineTimer, {
  ::std::int64_t t1;
  ::std::uint32_t t2;
  ::std::int32_t type;
});

static_assert(::std::is_trivially_copy_assignable<QDeadlineTimer>::value);
static_assert(::std::is_trivially_copy_constructible<QDeadlineTimer>::value);

static_assert(::std::is_trivially_destructible<QDeadlineTimer>::value);

static_assert(QTypeInfo<QDeadlineTimer>::isRelocatable);

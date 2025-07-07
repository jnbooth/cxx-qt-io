#include "cxx-qt-io/qsslcertificate.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QSslCertificate);

namespace rust {
namespace cxxqtio1 {
assert_shared_pointer_type(SubjectAlternativeNamesMap);

assert_alignment_and_size(SubjectAlternativeNamesValues, {
  ::std::size_t iter;
  ::std::size_t end;
});
static_assert(
  !::std::is_trivially_copy_assignable<SubjectAlternativeNamesValues>::value);
static_assert(!::std::is_trivially_copy_constructible<
              SubjectAlternativeNamesValues>::value);
static_assert(
  ::std::is_trivially_destructible<SubjectAlternativeNamesValues>::value);
static_assert(
  ::std::is_move_constructible<SubjectAlternativeNamesValues>::value);
static_assert(QTypeInfo<SubjectAlternativeNamesValues>::isRelocatable);

}
}

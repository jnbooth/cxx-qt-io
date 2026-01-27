#include "cxx-qt-io/qsslcertificate.h"

#include <cxx-qt-io/assertion_utils.h>

assert_shared_pointer_type(QSslCertificate);

namespace rust {
namespace cxxqtio1 {
assert_shared_pointer_type(SubjectAlternativeNamesMap);
assert_iter_type(SubjectAlternativeNamesIter);
assert_iter_type(SubjectAlternativeNamesKeys);
assert_iter_type(SubjectAlternativeNamesValues);
}
}

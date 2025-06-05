#include "cxx-qt-io/qsslconfiguration.h"

#include <cxx-qt-io/assertion_utils.h>
#include <string_view>

assert_shared_pointer_type(QSslConfiguration);

namespace rust {
namespace cxxqtio1 {
::rust::String
qsslconfigurationALPNProtocolHTTP2()
{
  return QSslConfiguration::ALPNProtocolHTTP2;
}

::rust::String
qsslconfigurationNextProtocolHttp1_1()
{
  return QSslConfiguration::NextProtocolHttp1_1;
}

}
}

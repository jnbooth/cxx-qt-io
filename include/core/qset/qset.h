#pragma once

#include <cxx-qt-io/definitions.h>
#include <cxx-qt-lib/qset.h>

#ifdef CXX_QT_IO_NET_FEATURE
#include "qset_qhostaddress.h"
#endif

#ifdef CXX_QT_IO_REQUEST_FEATURE
#include "qset_qhttp1configuration.h"
#endif

#ifdef CXX_QT_IO_SSL_FEATURE
#include "qset_qocspresponse.h"
#include "qset_qsslcertificate.h"
#include "qset_qssldiffiehellmanparameters.h"
#include "qset_qsslellipticcurve.h"
#include "qset_qsslerror.h"
#endif

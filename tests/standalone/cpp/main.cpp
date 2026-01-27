#include <QtCore/QScopedPointer>
#include <QtTest/QTest>

#include "qbuffer.h"
#include "qdeadlinetimer.h"
#include "qdtlsgeneratorparameters.h"
#include "qfile.h"
#include "qsslconfiguration.h"

static int app_argc;
static char** app_argv;

template<typename T>
int
runTest()
{
  QTest::Internal::callInitMain<T>();
  T test;
  return QTest::qExec(&test, app_argc, app_argv);
}

int
main(int argc, char* argv[])
{
  QCoreApplication app(argc, argv);

  return runTest<QBufferTest>() | runTest<QDeadlineTimerTest>() |
         runTest<QDtlsGeneratorParametersTest>() | runTest<QFileTest>() |
         runTest<QSslConfigurationTest>();
}

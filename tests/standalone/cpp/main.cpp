#include <QtCore/QScopedPointer>
#include <QtTest/QTest>

#include "qfile.h"

int
main(int argc, char* argv[])
{
  int status = 0;
  auto runTest = [&status, argc, argv](QScopedPointer<QObject> obj) {
    if (status == 0) {
      status |= QTest::qExec(obj.data(), argc, argv);
    } else {
      qWarning() << "Previous test failed, so skipping:" << obj.data();
    }
  };

  runTest(QScopedPointer<QObject>(new QFileTest));

  return status;
}

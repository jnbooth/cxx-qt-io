#pragma once

#include <QtTest/QTest>

#include "standalone/src/qfile.cxx.h"

class QFileTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const QString name = QStringLiteral("file.txt");
    const auto file = ffi::qfile::construct(name);
    QCOMPARE(file->fileName(), name);
  }
};

#pragma once

#include <QtTest/QTest>

#include "standalone/src/qbuffer.cxx.h"

class QBufferTest : public QObject
{
  Q_OBJECT

private:
private Q_SLOTS:
  void read()
  {
    QByteArray data = QByteArrayLiteral("Test string");
    QBuffer buf(&data);
    buf.open(QIODevice::ReadOnly);
    const QString result = ffi::qbuffer::read(buf);
    QCOMPARE(result.toStdString(), "Test string");
  }

  void write()
  {
    QByteArray data;
    QBuffer buf(&data);
    buf.open(QIODevice::WriteOnly);
    ffi::qbuffer::write(buf);
    QCOMPARE(data, "Test string");
  }
};

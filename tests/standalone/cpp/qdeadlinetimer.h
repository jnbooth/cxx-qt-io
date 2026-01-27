#pragma once

#include <QtTest/QTest>

#include "standalone/src/qdeadlinetimer.cxx.h"

class QDeadlineTimerTest : public QObject
{
  Q_OBJECT

private:
private Q_SLOTS:
  void construct_millis()
  {
    const auto timer = ffi::qdeadlinetimer::from_millis(12345);
    const auto time = timer.remainingTime();
    QCOMPARE_GT(time, 12340);
    QCOMPARE_LT(time, 12350);
  }

  void construct_nanos()
  {
    const auto timer = ffi::qdeadlinetimer::from_nanos(123450000);
    const auto time = timer.remainingTimeNSecs();
    QCOMPARE_GT(time, 123400000);
    QCOMPARE_LT(time, 123500000);
  }
};

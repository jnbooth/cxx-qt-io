#pragma once

#include <QtNetwork/QSslConfiguration>
#include <QtTest/QTest>

#include "standalone/src/qsslconfiguration.cxx.h"

class QSslConfigurationTest : public QObject
{
  Q_OBJECT

private:
private Q_SLOTS:
  void alpn_protocol()
  {
    QCOMPARE(ffi::qsslconfiguration::alpn_protocol(),
             std::string(QSslConfiguration::ALPNProtocolHTTP2));
  }

  void next_protocol()
  {
    QCOMPARE(ffi::qsslconfiguration::next_protocol(),
             std::string(QSslConfiguration::NextProtocolHttp1_1));
  }
};

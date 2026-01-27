#pragma once

#include <QtNetwork/QDtls>
#include <QtTest/QTest>

#include "standalone/src/qdtlsgeneratorparameters.cxx.h"

class QDtlsGeneratorParametersTest : public QObject
{
  Q_OBJECT

private:
  const QCryptographicHash::Algorithm hash = QCryptographicHash::Blake2b_160;
  const QByteArray secret = QByteArrayLiteral("secret");

private Q_SLOTS:
  void construct()
  {
    const QDtls::GeneratorParameters params =
      ffi::qdtlsgeneratorparams::construct(hash, secret);
    QCOMPARE(params.hash, hash);
    QCOMPARE(params.secret, secret);
  }

  void getHash()
  {
    const QDtls::GeneratorParameters params(hash, secret);
    QCOMPARE(ffi::qdtlsgeneratorparams::get_hash(params), hash);
  }

  void getSecret()
  {
    const QDtls::GeneratorParameters params(hash, secret);
    QCOMPARE(ffi::qdtlsgeneratorparams::get_secret(params), secret);
  }
};

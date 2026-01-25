# cxx-qtio

[![Github](https://img.shields.io/badge/github-jnbooth%2Fcxx--qt--io-informational?logo=github)](https://github.com/jnbooth/cxx-qt)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/jnbooth/cxx-qt-io/github-cxx-qt-io-tests.yml)
![License (MIT)](https://img.shields.io/crates/l/cxx-qt-io)

cxx-qt-io is a library of Rust bindings to [Qt](https://www.qt.io/)'s I/O framework, built on top of [cxx](https://cxx.rs) and [cxx-qt-lib](https://github.com/KDAB/cxx-qt). It provides bindings for [QIODevice](https://doc.qt.io/qt/qiodevice.html) and its subclasses, as well as helper classes.

For more information about the CXX-Qt project and how to use Qt in Rust, visit the [CXX-Qt Documentation book](https://kdab.github.io/cxx-qt/book/).

## Supported Qt Versions

cxx-qt-io currently supports all Qt versions from Qt 6.1 onward.

## Bindings Provided

### IO Devices

- QtCore:

  - [QBuffer](https://doc.qt.io/qt/qbuffer.html)
  - [QFile](https://doc.qt.io/qt/qfile.html)
  - [QFileDevice](https://doc.qt.io/qt/qfiledevice.html)
  - [QIODevice](https://doc.qt.io/qt/qiodevice.html)
  - [QSaveFile](https://doc.qt.io/qt/qsavefile.html)
  - [QTemporaryFile](https://doc.qt.io/qt/qtemporaryfile.html)

- QtNetwork:
  - [QAbstractSocket](https://doc.qt.io/qt/qabstractsocket.html)
  - [QLocalSocket](https://doc.qt.io/qt/qhttppart.html)
  - [QNetworkReply](https://doc.qt.io/qt/qnetworkreply.html)
  - [QSslSocket](https://doc.qt.io/qt/qsslsocket.html)
  - [QTcpSocket](https://doc.qt.io/qt/qtcpsocket.html)
  - [QUdpSocket](https://doc.qt.io/qt/qudpsocket.html)

### Helper Classes

- QtCore:

  - [QDeadlineTimer](https://doc.qt.io/qt/qdeadlinetimer.html)
  - [QDir](https://doc.qt.io/qt/qdir.html)

- QtNetwork:
  - [QAbstractNetworkCache](https://doc.qt.io/qt/qabstractnetworkcache.html)
  - [QAuthenticator](https://doc.qt.io/qt/qauthenticator.html)
  - [QCryptographicHashAlgorithm](https://doc.qt.io/qt/qcryptographichashalgorithm.html)
  - [QDtls](https://doc.qt.io/qt/qdtls.html)
  - [QDtlsClientVerifier](https://doc.qt.io/qt/qdtlsclientverifier.html)
  - [QDtlsGeneratorParameters](https://doc.qt.io/qt/qdtlsgeneratorparameters.html)
  - [QHostAddress](https://doc.qt.io/qt/qhostaddress.html)
  - [QHstsPolicy](https://doc.qt.io/qt/qhstspolicy.html)
  - [QHttp1Configuration](https://doc.qt.io/qt/qhttp1configuration.html)
  - [QHttp2Configuration](https://doc.qt.io/qt/qhttp2configuration.html)
  - [QHttpHeaders](https://doc.qt.io/qt/qhttpheaders.html)
  - [QHttpMultiPart](https://doc.qt.io/qt/qhttpmultipart.html)
  - [QHttpPart](https://doc.qt.io/qt/qhttppart.html)
  - [QNetworkAccessManager](https://doc.qt.io/qt/qnetworkaccessmanager.html)
  - [QNetworkAddressEntry](https://doc.qt.io/qt/qnetworkaddressentry.html)
  - [QNetworkCacheMetaData](https://doc.qt.io/qt/qnetworkcachemetadata.html)
  - [QNetworkCookie](https://doc.qt.io/qt/qnetworkcookie.html)
  - [QNetworkCookieJar](https://doc.qt.io/qt/qnetworkcookiejar.html)
  - [QNetworkDatagram](https://doc.qt.io/qt/qnetworkdatagram.html)
  - [QNetworkDiskCache](https://doc.qt.io/qt/qnetworkdiskcache.html)
  - [QNetworkInterface](https://doc.qt.io/qt/qnetworkinterface.html)
  - [QNetworkProxy](https://doc.qt.io/qt/qnetworkproxy.html)
  - [QNetworkRequest](https://doc.qt.io/qt/qnetworkrequest.html)
  - [QOcspResponse](https://doc.qt.io/qt/qocspresponse.html)
  - [QSslCertificate](https://doc.qt.io/qt/qsslcertificate.html)
  - [QSslCertificateExtension](https://doc.qt.io/qt/qsslcertificateextension.html)
  - [QSslCipher](https://doc.qt.io/qt/qsslcipher.html)
  - [QSslConfiguration](https://doc.qt.io/qt/qsslconfiguration.html)
  - [QSslDiffieHellmanParameters](https://doc.qt.io/qt/qssldiffiehellmanparameters.html)
  - [QSslEllipticCurve](https://doc.qt.io/qt/qsslellipticcurve.html)
  - [QSslError](https://doc.qt.io/qt/qsslerror.html)
  - [QSslKey](https://doc.qt.io/qt/qsslkey.html)
  - [QSslPreSharedKeyAuthenticator](https://doc.qt.io/qt/qsslpresharedkeyauthenticator.html)
  - [QSslServer](https://doc.qt.io/qt/qsslserver.html)
  - [QTcpServer](https://doc.qt.io/qt/qtcpserver.html)

### Building

Ensure that you have the following installed

- C++ compiler
- [clang-format](https://clang.llvm.org/docs/ClangFormat.html)
- [CMake](https://cmake.org/)
- [Qt 6](https://www.qt.io/)
- [Rust toolchain](https://www.rust-lang.org/)
- [mold](https://github.com/rui314/mold), [lld](https://lld.llvm.org/), or GNU ld.gold for Linux (lld is included in the XCode toolchain on macOS)

To build the library:

```bash
cargo build
```

### Testing

Testing assumes that `cargo clippy` and `cargo fmt` are available, you may need to install these with `rustup component add clippy rustfmt`.

To test the library:

```bash
cargo test
```

## License

Licensed under [MIT license](https://github.com/jnbooth/cxx-qt-io/LICENSE).

Test utilities are adapted from KDAB's own tests in the [cxx-qt](https://github.com/KDAB/cxx-qt) repository under its [MIT License](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/MIT.txt):

- [CMakeLists.txt](./CMakeLists.txt)
- [github/workflows/github-cxx-qt-io-tests.yml](./.github/workflows/github-cxx-qt-io-tests.yml)

SPDX license attributions are present in all adapted files.

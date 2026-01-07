#![allow(clippy::expect_used)]
#![allow(clippy::unwrap_used)]
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use cxx_qt_build::CxxQtBuilder;
use qt_build_utils::QtBuild;

macro_rules! include_header {
    ($file:literal) => {
        (include_bytes!($file), Path::new($file).file_name().unwrap())
    };
}

struct Features {
    pub fs: bool,
    pub net: bool,
    pub request: bool,
    pub ssl: bool,
}

impl Features {
    fn env(key: &str) -> bool {
        env::var(format!("CARGO_FEATURE_{key}")).is_ok()
    }

    pub fn from_env() -> Self {
        Self {
            fs: Self::env("FS"),
            net: Self::env("NET"),
            request: Self::env("REQUEST"),
            ssl: Self::env("SSL"),
        }
    }

    pub fn definitions_file(&self) -> String {
        let mut definitions = "#pragma once\n".to_owned();
        if self.fs {
            definitions.push_str("#define CXX_QT_IO_FS_FEATURE\n");
        }
        if self.net {
            definitions.push_str("#define CXX_QT_IO_NET_FEATURE\n");
        }
        if self.request {
            definitions.push_str("#define CXX_QT_IO_REQUEST_FEATURE\n");
        }
        if self.ssl {
            definitions.push_str("#define CXX_QT_IO_SSL_FEATURE\n");
        }
        definitions
    }
}

trait CxxQtBuilderExt {
    fn cpp_files(self, cpp_files: &[&str]) -> Self;
}

impl CxxQtBuilderExt for CxxQtBuilder {
    fn cpp_files(mut self, cpp_files: &[&str]) -> Self {
        for cpp_file in cpp_files {
            self = self.cpp_file(cpp_file);
        }
        self
    }
}

trait VersionExt {
    fn at_least(&self, major: u64, minor: u64) -> bool;

    fn find(&self, name: &str, versions: &[(u64, u64)]) -> String {
        for &(major, minor) in versions {
            if self.at_least(major, minor) {
                return format!("{name}/v{major}_{minor}.rs");
            }
        }
        format!("{name}/v6_1.rs")
    }
}

impl VersionExt for semver::Version {
    fn at_least(&self, major: u64, minor: u64) -> bool {
        self.major > major || (self.major == major && self.minor >= minor)
    }
}

#[allow(clippy::needless_borrows_for_generic_args)]
fn main() {
    let features = Features::from_env();

    let mut qt_modules = vec!["Core".to_owned()];
    if features.net {
        qt_modules.push("Network".to_owned());
    }

    let qtbuild = QtBuild::new(qt_modules).expect("Could not find Qt installation");
    let version = qtbuild.version();

    let header_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("include")
        .join("cxx-qt-io");

    fs::create_dir_all(&header_dir).expect("Failed to create include directory");

    fs::write(
        header_dir.join("definitions.h"),
        features.definitions_file(),
    )
    .expect("Failed to write definitions.h");

    let mut headers: Vec<(&[u8], &OsStr)> = vec![
        include_header!("include/assertion_utils.h"),
        include_header!("include/common.h"),
        include_header!("include/core/qbytearray.h"),
        include_header!("include/core/qbuffer.h"),
        include_header!("include/core/qdeadlinetimer.h"),
        include_header!("include/core/qhash/qhash_i32_qvariant.h"),
        include_header!("include/core/qhash/qhash_private.h"),
        include_header!("include/core/qhash/qhash.h"),
        include_header!("include/core/qiodevice.h"),
        include_header!("include/core/qlist/qlist_private.h"),
        include_header!("include/core/qlist/qlist_qdeadlinetimer.h"),
        include_header!("include/core/qlist/qlist_qpair.h"),
        include_header!("include/core/qlist/qlist_qpair_qbytearray_qbytearray.h"),
        include_header!("include/core/qlist/qlist.h"),
        include_header!("include/core/qmap/qmap_private.h"),
        include_header!("include/core/qmap/qmap_qbytearray_qvariant.h"),
        include_header!("include/core/qmap/qmap.h"),
        include_header!("include/core/qmultimap.h"),
        include_header!("include/core/qobject.h"),
        include_header!("include/core/qpair/qpair_private.h"),
        include_header!("include/core/qpair/qpair_qbytearray_qbytearray.h"),
        include_header!("include/core/qpair/qpair.h"),
        include_header!("include/core/qset/qset_private.h"),
        include_header!("include/core/qset/qset.h"),
        include_header!("include/core/qvariant/qvariant.h"),
    ];

    let mut builder = CxxQtBuilder::new()
        // Use a short name due to the Windows file path limit!
        // We don't re-export these headers anyway
        .include_prefix("private")
        .crate_include_root(Some("include/".to_owned()))
        .include_dir(&header_dir)
        .cpp_files(&[
            "src/core/qbuffer.cpp",
            "src/core/qbytearray.cpp",
            "src/core/qdeadlinetimer.cpp",
            "src/core/qhash/qhash.cpp",
            "src/core/qiodevice.cpp",
            "src/core/qlist/qlist.cpp",
            "src/core/qmap/qmap.cpp",
            "src/core/qobject.cpp",
            "src/core/qpair.cpp",
            "src/core/qset/qset.cpp",
            "src/core/qvariant/qvariant.cpp",
        ])
        .files(&[
            "src/core/qbuffer.rs",
            "src/core/qdeadlinetimer.rs",
            "src/core/qhash/qhash_i32_qvariant.rs",
            "src/core/qiodevice.rs",
            "src/core/qlist/qlist_qdeadlinetimer.rs",
            "src/core/qlist/qlist_qpair_qbytearray_qbytearray.rs",
            "src/core/qmap/qmap_qbytearray_qvariant.rs",
            "src/core/qobject.rs",
            "src/core/qt.rs",
        ]);

    if features.fs {
        headers.extend_from_slice(&[
            include_header!("include/core/qdir.h"),
            include_header!("include/core/qfile.h"),
            include_header!("include/core/qfiledevice.h"),
            include_header!("include/core/qsavefile.h"),
            include_header!("include/core/qstandardpaths.h"),
            include_header!("include/core/qtemporaryfile.h"),
        ]);

        builder = builder.cpp_files(&["src/fs/qdir.cpp"]).files(&[
            "src/fs/qdir.rs",
            "src/fs/qfile.rs",
            "src/fs/qfiledevice.rs",
            "src/fs/qsavefile.rs",
            "src/fs/qstandardpaths/mod.rs",
            &version.find("src/fs/qstandardpaths", &[(6, 7), (6, 4)]),
            "src/fs/qtemporaryfile.rs",
        ]);
    }

    if features.net {
        headers.extend_from_slice(&[
            include_header!("include/core/qlist/qlist_qhostaddress.h"),
            include_header!("include/core/qlist/qlist_qnetworkaddressentry.h"),
            include_header!("include/core/qlist/qlist_qnetworkdatagram.h"),
            include_header!("include/core/qlist/qlist_qnetworkinterface.h"),
            include_header!("include/core/qlist/qlist_qnetworkproxy.h"),
            include_header!("include/core/qset/qset_qhostaddress.h"),
            include_header!("include/core/qpair/qpair_qhostaddress_i32.h"),
            include_header!("include/network/qabstractsocket.h"),
            include_header!("include/network/qauthenticator.h"),
            include_header!("include/network/qhostaddress.h"),
            include_header!("include/network/qlocalsocket.h"),
            include_header!("include/network/qnetworkaddressentry.h"),
            include_header!("include/network/qnetworkdatagram.h"),
            include_header!("include/network/qnetworkinterface.h"),
            include_header!("include/network/qnetworkproxy.h"),
            include_header!("include/network/qnetworkrequest.h"),
            include_header!("include/network/qtcpserver.h"),
            include_header!("include/network/qtcpsocket.h"),
            include_header!("include/network/qudpsocket.h"),
        ]);

        builder = builder
            .qt_module("Network")
            .cpp_files(&[
                "src/net/qhostaddress.cpp",
                "src/net/qnetworkaddressentry.cpp",
                "src/net/qnetworkdatagram.cpp",
                "src/net/qnetworkproxy.cpp",
            ])
            .files(&[
                "src/core/qlist/qlist_qhostaddress.rs",
                "src/core/qlist/qlist_qnetworkaddressentry.rs",
                "src/core/qlist/qlist_qnetworkdatagram.rs",
                "src/core/qlist/qlist_qnetworkinterface.rs",
                "src/core/qlist/qlist_qnetworkproxy.rs",
                "src/core/qset/qset_qhostaddress.rs",
                "src/net/raw_header_list.rs",
                "src/net/qabstractsocket.rs",
                "src/net/qauthenticator.rs",
                "src/net/qhostaddress.rs",
                "src/net/qlocalsocket.rs",
                "src/net/qnetworkaddressentry.rs",
                "src/net/qnetworkdatagram.rs",
                "src/net/qnetworkinterface.rs",
                "src/net/qnetworkproxy.rs",
                "src/net/qnetworkrequestknownheaders.rs",
                "src/net/qtcpserver.rs",
                "src/net/qtcpsocket.rs",
                "src/net/qudpsocket.rs",
            ]);

        if version.at_least(6, 7) {
            headers.extend_from_slice(&[
                include_header!("include/core/qlist/qlist_qhttpheaders.h"),
                include_header!("include/network/qhttpheaders.h"),
            ]);
            builder = builder.cpp_files(&["src/net/qhttpheaders.cpp"]).files(&[
                "src/core/qlist/qlist_qhttpheaders.rs",
                "src/net/qhttpheaders.rs",
            ]);
        }
    }

    if features.request {
        headers.extend_from_slice(&[
            include_header!("include/core/qlist/qlist_qhstspolicy.h"),
            include_header!("include/core/qlist/qlist_qhttp2configuration.h"),
            include_header!("include/core/qlist/qlist_qhttppart.h"),
            include_header!("include/core/qlist/qlist_qnetworkcachemetadata.h"),
            include_header!("include/core/qlist/qlist_qnetworkcookie.h"),
            include_header!("include/core/qlist/qlist_qnetworkrequest.h"),
            include_header!("include/core/qvariant/qvariant_qnetworkcookie.h"),
            include_header!("include/network/qabstractnetworkcache.h"),
            include_header!("include/network/qhstspolicy.h"),
            include_header!("include/network/qhttp2configuration.h"),
            include_header!("include/network/qhttpmultipart.h"),
            include_header!("include/network/qhttppart.h"),
            include_header!("include/network/qnetworkaccessmanager.h"),
            include_header!("include/network/qnetworkcachemetadata.h"),
            include_header!("include/network/qnetworkcookie.h"),
            include_header!("include/network/qnetworkcookiejar.h"),
            include_header!("include/network/qnetworkdiskcache.h"),
            include_header!("include/network/qnetworkreply.h"),
        ]);

        builder = builder
            .cpp_files(&[
                "src/request/qabstractnetworkcache.cpp",
                "src/request/qnetworkaccessmanager.cpp",
                "src/request/qnetworkcachemetadata.cpp",
                "src/request/qhstspolicy.cpp",
                "src/request/qhttp2configuration.cpp",
                "src/request/qhttppart.cpp",
                "src/request/qnetworkcachemetadata.cpp",
                "src/request/qnetworkcookie.cpp",
                "src/request/qnetworkrequest/qnetworkrequest.cpp",
            ])
            .files(&[
                "src/core/qlist/qlist_qhstspolicy.rs",
                "src/core/qlist/qlist_qhttp2configuration.rs",
                "src/core/qlist/qlist_qhttppart.rs",
                "src/core/qlist/qlist_qnetworkcachemetadata.rs",
                "src/core/qlist/qlist_qnetworkcookie.rs",
                "src/core/qlist/qlist_qnetworkrequest.rs",
                "src/core/qvariant/qvariant_qnetworkcookie.rs",
                "src/request/qabstractnetworkcache.rs",
                "src/request/qhstspolicy.rs",
                "src/request/qhttp2configuration.rs",
                "src/request/qhttpmultipart.rs",
                "src/request/qhttppart.rs",
                "src/request/qnetworkaccessmanager.rs",
                "src/request/qnetworkcachemetadata.rs",
                "src/request/qnetworkcookie.rs",
                "src/request/qnetworkcookiejar.rs",
                "src/request/qnetworkdiskcache.rs",
                "src/request/qnetworkrequest/mod.rs",
                &version.find(
                    "src/request/qnetworkrequest/attribute",
                    &[(6, 8), (6, 5), (6, 3)],
                ),
                "src/request/qnetworkreply.rs",
            ]);

        if version.at_least(6, 5) {
            headers.extend_from_slice(&[
                include_header!("include/core/qlist/qlist_qhttp1configuration.h"),
                include_header!("include/core/qset/qset_qhttp1configuration.h"),
                include_header!("include/network/qhttp1configuration.h"),
            ]);
            builder = builder
                .cpp_files(&["src/request/qhttp1configuration.cpp"])
                .files(&[
                    "src/core/qlist/qlist_qhttp1configuration.rs",
                    "src/core/qset/qset_qhttp1configuration.rs",
                    "src/request/qhttp1configuration.rs",
                ]);
        }
    }

    if features.ssl {
        headers.extend_from_slice(&[
            include_header!("include/core/qcryptographichash.h"),
            include_header!("include/core/qlist/qlist_qdtlsgeneratorparameters.h"),
            include_header!("include/core/qlist/qlist_qocspresponse.h"),
            include_header!("include/core/qlist/qlist_qsslcertificate.h"),
            include_header!("include/core/qlist/qlist_qsslcertificateextension.h"),
            include_header!("include/core/qlist/qlist_qsslcipher.h"),
            include_header!("include/core/qlist/qlist_qsslconfiguration.h"),
            include_header!("include/core/qlist/qlist_qssldiffiehellmanparameters.h"),
            include_header!("include/core/qlist/qlist_qsslellipticcurve.h"),
            include_header!("include/core/qlist/qlist_qsslerror.h"),
            include_header!("include/core/qlist/qlist_qsslkey.h"),
            include_header!("include/core/qlist/qlist_qsslpresharedkeyauthenticator.h"),
            include_header!("include/core/qset/qset_qocspresponse.h"),
            include_header!("include/core/qset/qset_qsslcertificate.h"),
            include_header!("include/core/qset/qset_qssldiffiehellmanparameters.h"),
            include_header!("include/core/qset/qset_qsslellipticcurve.h"),
            include_header!("include/core/qset/qset_qsslerror.h"),
            include_header!("include/network/qdtls.h"),
            include_header!("include/network/qdtlsclientverifier.h"),
            include_header!("include/network/qdtlsgeneratorparameters.h"),
            include_header!("include/network/qocspresponse.h"),
            include_header!("include/network/qssl.h"),
            include_header!("include/network/qsslcertificate.h"),
            include_header!("include/network/qsslcertificateextension.h"),
            include_header!("include/network/qsslcipher.h"),
            include_header!("include/network/qsslconfiguration.h"),
            include_header!("include/network/qssldiffiehellmanparameters.h"),
            include_header!("include/network/qsslellipticcurve.h"),
            include_header!("include/network/qsslerror.h"),
            include_header!("include/network/qsslkey.h"),
            include_header!("include/network/qsslpresharedkeyauthenticator.h"),
            include_header!("include/network/qsslsocket.h"),
        ]);

        builder = builder
            .qt_module("Network")
            .cpp_files(&[
                "src/ssl/qdtlsgeneratorparameters.cpp",
                "src/ssl/qocspresponse.cpp",
                "src/ssl/qsslcertificate.cpp",
                "src/ssl/qsslcertificateextension.cpp",
                "src/ssl/qsslconfiguration.cpp",
                "src/ssl/qssldiffiehellmanparameters.cpp",
                "src/ssl/qsslellipticcurve.cpp",
                "src/ssl/qsslerror.cpp",
                "src/ssl/qsslkey.cpp",
                "src/ssl/qsslpresharedkeyauthenticator.cpp",
                "src/ssl/qsslsocket.cpp",
            ])
            .files(&[
                "src/core/qlist/qlist_qdtlsgeneratorparameters.rs",
                "src/core/qlist/qlist_qocspresponse.rs",
                "src/core/qlist/qlist_qsslcertificate.rs",
                "src/core/qlist/qlist_qsslcipher.rs",
                "src/core/qlist/qlist_qsslcertificateextension.rs",
                "src/core/qlist/qlist_qsslconfiguration.rs",
                "src/core/qlist/qlist_qssldiffiehellmanparameters.rs",
                "src/core/qlist/qlist_qsslellipticcurve.rs",
                "src/core/qlist/qlist_qsslerror.rs",
                "src/core/qlist/qlist_qsslkey.rs",
                "src/core/qlist/qlist_qsslpresharedkeyauthenticator.rs",
                "src/core/qset/qset_qocspresponse.rs",
                "src/core/qset/qset_qsslcertificate.rs",
                "src/core/qset/qset_qssldiffiehellmanparameters.rs",
                "src/core/qset/qset_qsslellipticcurve.rs",
                "src/core/qset/qset_qsslerror.rs",
                "src/ssl/qcryptographichash.rs",
                "src/ssl/qdtls.rs",
                "src/ssl/qdtlsclientverifier.rs",
                "src/ssl/qdtlsgeneratorparameters.rs",
                "src/ssl/qocspresponse.rs",
                "src/ssl/qssl/mod.rs",
                &version.find("src/ssl/qssl/implemented_class", &[(6, 2)]),
                "src/ssl/qsslcertificate.rs",
                "src/ssl/qsslcertificateextension.rs",
                "src/ssl/qsslcipher.rs",
                "src/ssl/qsslconfiguration.rs",
                "src/ssl/qssldiffiehellmanparameters.rs",
                "src/ssl/qsslellipticcurve.rs",
                "src/ssl/qsslerror.rs",
                "src/ssl/qsslkey.rs",
                "src/ssl/qsslpresharedkeyauthenticator.rs",
                "src/ssl/qsslsocket.rs",
            ]);

        if version.at_least(6, 4) {
            headers.extend_from_slice(&[include_header!("include/network/qsslserver.h")]);
            builder = builder.files(&["src/ssl/qsslserver.rs"]);
        }
    }

    for &(file_contents, file_name) in &headers {
        let out_path = header_dir.join(file_name);
        let mut header = File::create(out_path).expect("Could not create header");
        header
            .write_all(file_contents)
            .expect("Could not write header");
    }

    let interface = builder.build();
    interface.reexport_dependency("cxx-qt-lib").export();
}

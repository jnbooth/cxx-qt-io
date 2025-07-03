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
}

struct HeaderBuilder {
    dir: PathBuf,
}

impl HeaderBuilder {
    pub fn new() -> Self {
        let dir = PathBuf::from(env::var("OUT_DIR").unwrap())
            .join("include")
            .join("cxx-qt-io");
        Self { dir }
    }

    pub fn create_header_dir(&self) {
        fs::create_dir_all(&self.dir).expect("Failed to create include directory");
    }

    pub fn write_definitions(&self, features: &Features) {
        let mut definitions = "#pragma once\n".to_owned();

        if features.fs {
            definitions.push_str("#define CXX_QT_IO_FS_FEATURE\n");
        }

        if features.net {
            definitions.push_str("#define CXX_QT_IO_NET_FEATURE\n");
        }

        if features.request {
            definitions.push_str("#define CXX_QT_IO_REQUEST_FEATURE\n");
        }

        if features.ssl {
            definitions.push_str("#define CXX_QT_IO_SSL_FEATURE\n");
        }

        fs::write(self.dir.join("definitions.h"), definitions)
            .expect("Failed to write definitions.h");
    }

    pub fn write_headers(&self, files: &[(&[u8], &OsStr)]) {
        for &(file_contents, file_name) in files {
            let out_path = self.dir.join(file_name);
            let mut header = File::create(out_path).expect("Could not create header");
            header
                .write_all(file_contents)
                .expect("Could not write header");
        }
    }
}

impl AsRef<Path> for HeaderBuilder {
    fn as_ref(&self) -> &Path {
        &self.dir
    }
}

trait BridgeBuilder {
    fn build_cpp(self, cpp_files: &[&str]) -> Self;
    fn build_rust(self, rust_bridges: &[&str]) -> Self;
}

impl BridgeBuilder for CxxQtBuilder {
    fn build_cpp(self, cpp_files: &[&str]) -> Self {
        self.cc_builder(move |cc| {
            for cpp_file in cpp_files {
                cc.file(format!("src/{cpp_file}.cpp"));
                println!("cargo::rerun-if-changed=src/{cpp_file}.cpp");
            }
        })
    }

    fn build_rust(mut self, rust_bridges: &[&str]) -> Self {
        for rust_source in rust_bridges {
            self = self.file(format!("src/{rust_source}.rs"));
        }
        self
    }
}

trait AtLeast {
    fn at_least(&self, major: u64, minor: u64) -> bool;

    fn find(&self, name: &str, versions: &[(u64, u64)]) -> String {
        for &(major, minor) in versions {
            if self.at_least(major, minor) {
                return format!("{name}/v{major}_{minor}");
            }
        }
        format!("{name}/v6_0")
    }
}

impl AtLeast for semver::Version {
    fn at_least(&self, major: u64, minor: u64) -> bool {
        self.major > major || (self.major == major && self.minor >= minor)
    }
}

fn main() {
    let features = Features::from_env();

    let mut qt_modules = vec!["Core".to_owned()];
    if features.net {
        qt_modules.push("Network".to_owned());
    }

    let qtbuild = QtBuild::new(qt_modules).expect("Could not find Qt installation");
    let version = qtbuild.version();

    let header_dir = HeaderBuilder::new();

    header_dir.create_header_dir();

    header_dir.write_definitions(&features);

    header_dir.write_headers(&[
        include_header!("include/assertion_utils.h"),
        include_header!("include/common.h"),
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
        include_header!("include/core/qpair/qpair_private.h"),
        include_header!("include/core/qpair/qpair_qbytearray_qbytearray.h"),
        include_header!("include/core/qpair/qpair.h"),
        include_header!("include/core/qset/qset_private.h"),
        include_header!("include/core/qset/qset.h"),
        include_header!("include/core/qvariant/qvariant.h"),
        include_header!("include/util.h"),
        include_header!("include/views.h"),
    ]);

    let interface = cxx_qt_build::Interface::default()
        // Disable exporting the standard include directory, as we are exporting custom headers
        .export_include_prefixes([])
        .export_include_directory(&header_dir, "cxx-qt-io")
        .reexport_dependency("cxx-qt-lib");

    let mut builder = CxxQtBuilder::library(interface)
        .build_cpp(&[
            "core/qbuffer",
            "core/qdeadlinetimer",
            "core/qhash/qhash",
            "core/qiodevice",
            "core/qlist/qlist",
            "core/qpair",
            "core/qmap/qmap",
            "core/qset/qset",
            "core/qvariant/qvariant",
        ])
        .build_rust(&[
            "core/qbuffer",
            "core/qdeadlinetimer",
            "core/qiodevice",
            "core/qlist/qlist_qpair_qbytearray_qbytearray",
            "core/qmap/qmap_qbytearray_qvariant",
            "core/qt",
            "util",
        ]);

    if features.fs {
        header_dir.write_headers(&[
            include_header!("include/core/qdir.h"),
            include_header!("include/core/qfile.h"),
            include_header!("include/core/qfiledevice.h"),
            include_header!("include/core/qsavefile.h"),
            include_header!("include/core/qstandardpaths.h"),
            include_header!("include/core/qtemporaryfile.h"),
        ]);

        builder = builder.build_rust(&[
            "fs/qdir",
            "fs/qfile",
            "fs/qfiledevice",
            "fs/qsavefile",
            "fs/qstandardpaths/mod",
            &version.find("fs/qstandardpaths", &[(6, 7), (6, 4)]),
            "fs/qtemporaryfile",
        ]);
    }

    if features.net {
        header_dir.write_headers(&[
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
            .build_cpp(&[
                "net/qhostaddress",
                "net/qnetworkaddressentry",
                "net/qnetworkdatagram",
                "net/qnetworkproxy",
            ])
            .build_rust(&[
                "core/qlist/qlist_qhostaddress",
                "core/qlist/qlist_qnetworkaddressentry",
                "core/qlist/qlist_qnetworkdatagram",
                "core/qlist/qlist_qnetworkinterface",
                "core/qlist/qlist_qnetworkproxy",
                "net/raw_header_list",
                "net/qabstractsocket",
                "net/qauthenticator",
                "net/qhostaddress",
                "net/qlocalsocket",
                "net/qnetworkaddressentry",
                "net/qnetworkdatagram",
                "net/qnetworkinterface",
                "net/qnetworkproxy",
                "net/qnetworkrequestknownheaders",
                "net/qtcpserver",
                "net/qtcpsocket",
                "net/qudpsocket",
            ]);

        if version.at_least(6, 7) {
            header_dir.write_headers(&[
                include_header!("include/core/qlist/qlist_qhttpheaders.h"),
                include_header!("include/network/qhttpheaders.h"),
            ]);
            builder = builder
                .build_cpp(&["net/qhttpheaders"])
                .build_rust(&["core/qlist/qlist_qhttpheaders", "net/qhttpheaders"]);
        }
    }

    if features.request {
        header_dir.write_headers(&[
            include_header!("include/core/qlist/qlist_qhstspolicy.h"),
            include_header!("include/core/qlist/qlist_qhttp2configuration.h"),
            include_header!("include/core/qlist/qlist_qhttppart.h"),
            include_header!("include/core/qlist/qlist_qnetworkcachemetadata.h"),
            include_header!("include/core/qlist/qlist_qnetworkcookie.h"),
            include_header!("include/core/qlist/qlist_qnetworkrequest.h"),
            include_header!("include/core/qset/qset_qhttp1configuration.h"),
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
            .build_cpp(&[
                "request/qabstractnetworkcache",
                "request/qnetworkaccessmanager",
                "request/qnetworkcachemetadata",
                "request/qhstspolicy",
                "request/qhttp2configuration",
                "request/qhttppart",
                "request/qnetworkcachemetadata",
                "request/qnetworkcookie/qnetworkcookie",
                "request/qnetworkrequest/qnetworkrequest",
            ])
            .build_rust(&[
                "core/qlist/qlist_qhstspolicy",
                "core/qlist/qlist_qhttp2configuration",
                "core/qlist/qlist_qhttppart",
                "core/qlist/qlist_qnetworkcachemetadata",
                "core/qlist/qlist_qnetworkcookie",
                "core/qlist/qlist_qnetworkrequest",
                "core/qvariant/qvariant_qnetworkcookie",
                "request/qabstractnetworkcache",
                "request/qhstspolicy",
                "request/qhttp2configuration",
                "request/qhttpmultipart",
                "request/qhttppart",
                "request/qnetworkaccessmanager",
                "request/qnetworkcachemetadata",
                "request/qnetworkcookie/mod",
                "request/qnetworkcookiejar",
                "request/qnetworkdiskcache",
                "request/qnetworkrequest/mod",
                &version.find(
                    "request/qnetworkrequest/attribute",
                    &[(6, 8), (6, 5), (6, 3)],
                ),
                "request/qnetworkreply",
            ]);

        if version.at_least(6, 1) {
            builder = builder.build_rust(&["request/qnetworkcookie/v6_1"]);
        }

        if version.at_least(6, 5) {
            header_dir.write_headers(&[
                include_header!("include/core/qlist/qlist_qhttp1configuration.h"),
                include_header!("include/network/qhttp1configuration.h"),
            ]);
            builder = builder
                .build_cpp(&["request/qhttp1configuration"])
                .build_rust(&[
                    "core/qlist/qlist_qhttp1configuration",
                    "request/qhttp1configuration",
                ]);
        }
    }

    if features.ssl {
        header_dir.write_headers(&[
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
            include_header!("include/network/qsslserver.h"),
        ]);

        builder = builder
            .qt_module("Network")
            .build_cpp(&[
                "ssl/qdtlsgeneratorparameters",
                "ssl/qocspresponse",
                "ssl/qsslcertificate",
                "ssl/qsslcertificateextension",
                "ssl/qsslconfiguration",
                "ssl/qssldiffiehellmanparameters",
                "ssl/qsslellipticcurve",
                "ssl/qsslerror",
                "ssl/qsslkey",
                "ssl/qsslpresharedkeyauthenticator",
                "ssl/qsslsocket",
            ])
            .build_rust(&[
                "core/qlist/qlist_qdtlsgeneratorparameters",
                "core/qlist/qlist_qocspresponse",
                "core/qlist/qlist_qsslcertificate",
                "core/qlist/qlist_qsslcertificateextension",
                "core/qlist/qlist_qsslconfiguration",
                "core/qlist/qlist_qssldiffiehellmanparameters",
                "core/qlist/qlist_qsslellipticcurve",
                "core/qlist/qlist_qsslerror",
                "core/qlist/qlist_qsslkey",
                "core/qlist/qlist_qsslpresharedkeyauthenticator",
                "ssl/qcryptographichash",
                "ssl/qdtls",
                "ssl/qdtlsclientverifier",
                "ssl/qdtlsgeneratorparameters",
                "ssl/qocspresponse",
                "ssl/qssl/mod",
                "ssl/qsslcertificate",
                "ssl/qsslcertificateextension",
                "ssl/qsslcipher",
                "ssl/qsslconfiguration",
                "ssl/qssldiffiehellmanparameters",
                "ssl/qsslellipticcurve",
                "ssl/qsslerror",
                "ssl/qsslkey",
                "ssl/qsslpresharedkeyauthenticator",
                "ssl/qsslsocket",
            ]);

        if version.at_least(6, 1) {
            builder = builder.build_rust(&["ssl/qssl/v6_1"]);
        }

        if version.at_least(6, 4) {
            header_dir.write_headers(&[include_header!("include/network/qsslsocket.h")]);
            builder = builder.build_rust(&["ssl/qsslserver"]);
        }
    }

    builder.build();
}

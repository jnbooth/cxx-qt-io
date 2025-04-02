use cxx_qt_build::CxxQtBuilder;
use qt_build_utils::{QtBuild, SemVer};
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

macro_rules! include_header {
    ($file:literal) => {
        (include_bytes!($file), Path::new($file).file_name().unwrap())
    };
}

struct Features {
    pub network: bool,
    pub ssl: bool,
}

impl Features {
    fn env(key: &str) -> bool {
        env::var(format!("CARGO_FEATURE_{key}")).is_ok()
    }

    pub fn from_env() -> Self {
        Self {
            network: Self::env("QT_NETWORK"),
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

        if features.network {
            definitions.push_str("#define CXX_QT_IO_NETWORK_FEATURE\n");
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
    fn at_least(&self, major: u32, minor: u32) -> bool;

    fn find(&self, name: &str, versions: &[(u32, u32)]) -> String {
        for &(major, minor) in versions {
            if self.at_least(major, minor) {
                return format!("{name}_{major}_{minor}");
            }
        }
        name.to_owned()
    }
}

impl AtLeast for SemVer {
    fn at_least(&self, major: u32, minor: u32) -> bool {
        self.major > major || (self.major == major && self.minor >= minor)
    }
}

fn main() {
    let features = Features::from_env();

    let mut qt_modules = vec!["Core".to_owned()];
    if features.network {
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
        include_header!("include/core/qfiledevice.h"),
        include_header!("include/core/qcryptographichash.h"),
        include_header!("include/core/qiodevice.h"),
        include_header!("include/core/qlist/qlist_private.h"),
        include_header!("include/core/qlist/qlist_qpair_qbytearray_qbytearray.h"),
        include_header!("include/core/qlist/qlist.h"),
        include_header!("include/core/qpair/qpair_private.h"),
        include_header!("include/core/qpair/qpair_qbytearray_qbytearray.h"),
        include_header!("include/core/qpair/qpair.h"),
        include_header!("include/core/qvariant/qvariant.h"),
        include_header!("include/core/qtemporaryfile.h"),
        include_header!("include/views.h"),
    ]);

    let interface = cxx_qt_build::Interface::default()
        // Disable exporting the standard include directory, as we are exporting custom headers
        .export_include_prefixes([])
        .export_include_directory(&header_dir, "cxx-qt-io")
        .reexport_dependency("cxx-qt-lib");

    let mut builder = CxxQtBuilder::library(interface)
        .build_cpp(&[
            "core/qiodevice",
            "core/qlist/qlist",
            "core/qpair/qpair",
            "core/qvariant/qvariant",
        ])
        .build_rust(&[
            &version.find(
                "core/qcryptographichash/cryptographic_hash_algorithm",
                &[(6, 0), (5, 9), (5, 1)],
            ),
            "core/qfile",
            "core/qfiledevice",
            "core/qiodevice",
            "core/qlist/qlist_qpair_qbytearray_qbytearray",
            "core/qsavefile",
            "core/qtemporaryfile",
        ]);

    if features.network {
        header_dir.write_headers(&[
            include_header!("include/core/qlist/qlist_qhostaddress.h"),
            include_header!("include/core/qlist/qlist_qnetworkaddressentry.h"),
            include_header!("include/core/qlist/qlist_qnetworkcookie.h"),
            include_header!("include/core/qvariant/qvariant_qnetworkcookie.h"),
            include_header!("include/core/qlist/qlist_qnetworkinterface.h"),
            include_header!("include/core/qpair/qpair_qhostaddress_i32.h"),
            include_header!("include/network/qabstractsocket.h"),
            include_header!("include/network/qhostaddress.h"),
            include_header!("include/network/qnetworkaddressentry.h"),
            include_header!("include/network/qnetworkcookie.h"),
            include_header!("include/network/qnetworkdatagram.h"),
            include_header!("include/network/qnetworkinterface.h"),
            include_header!("include/network/qnetworkproxy.h"),
            include_header!("include/network/qnetworkrequest.h"),
        ]);

        builder = builder
            .qt_module("Network")
            .build_cpp(&[
                "network/qhostaddress",
                "network/qnetworkaddressentry",
                "network/qnetworkcookie",
                "network/qnetworkdatagram",
                "network/qnetworkproxy",
            ])
            .build_rust(&[
                "core/qlist/qlist_qhostaddress",
                "core/qlist/qlist_qnetworkaddressentry",
                "core/qlist/qlist_qnetworkcookie",
                "core/qlist/qlist_qnetworkinterface",
                "core/qpair/qpair_qhostaddress_i32",
                "core/qvariant/qvariant_qnetworkcookie",
                "network/qabstractsocket",
                "network/qauthenticator",
                "network/qhostaddress",
                "network/qnetworkaddressentry",
                "network/qnetworkcookie",
                "network/qnetworkdatagram",
                "network/qnetworkinterface",
                "network/qnetworkproxy",
                "network/qnetworkrequest",
                "network/qtcpsocket",
                "network/qudpsocket",
            ]);

        if version.at_least(6, 7) {
            header_dir.write_headers(&[include_header!("include/network/qhttpheaders.h")]);
            builder = builder
                .build_cpp(&["network/qhttpheaders"])
                .build_rust(&["network/qhttpheaders"]);
        }
    }
    if features.ssl {
        header_dir.write_headers(&[
            include_header!("include/core/qlist/qlist_qocspresponse.h"),
            include_header!("include/core/qlist/qlist_qsslcertificate.h"),
            include_header!("include/core/qlist/qlist_qsslcertificateextension.h"),
            include_header!("include/core/qlist/qlist_qsslerror.h"),
            include_header!("include/network/qocspresponse.h"),
            include_header!("include/network/qssl.h"),
            include_header!("include/network/qsslcertificate.h"),
            include_header!("include/network/qsslcertificateextension.h"),
            include_header!("include/network/qsslerror.h"),
            include_header!("include/network/qsslkey.h"),
        ]);

        builder = builder
            .qt_module("Network")
            .build_cpp(&[
                "network/qocspresponse",
                "network/qsslcertificate",
                "network/qsslcertificateextension",
                "network/qsslerror",
                "network/qsslkey",
            ])
            .build_rust(&[
                "core/qlist/qlist_qocspresponse",
                "core/qlist/qlist_qsslcertificate",
                "core/qlist/qlist_qsslcertificateextension",
                "core/qlist/qlist_qsslerror",
                "network/qocspresponse",
                "network/qssl/mod",
                &version.find("network/qssl/alternative_name_entry_type", &[(5, 13)]),
                &version.find("network/qssl/protocol", &[(6, 3), (5, 12)]),
                "network/qsslcertificate",
                "network/qsslcertificateextension",
                "network/qsslerror",
                "network/qsslkey",
            ]);
    }

    builder.build();
}

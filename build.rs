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
}

impl Features {
    fn env(key: &str) -> bool {
        env::var(format!("CARGO_FEATURE_{key}")).is_ok()
    }

    pub fn from_env() -> Self {
        Self {
            network: Self::env("QT_NETWORK"),
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
        include_header!("include/common.h"),
        include_header!("include/core/qfiledevice.h"),
        include_header!("include/core/qiodevice.h"),
        include_header!("include/core/qlist/qlist_private.h"),
        include_header!("include/core/qlist/qlist_qpair_qbytearray_qbytearray.h"),
        include_header!("include/core/qlist/qlist.h"),
        include_header!("include/core/qpair/qpair_private.h"),
        include_header!("include/core/qpair/qpair_qbytearray_qbytearray.h"),
        include_header!("include/core/qpair/qpair.h"),
        include_header!("include/core/qtemporaryfile.h"),
    ]);

    let interface = cxx_qt_build::Interface::default()
        // Disable exporting the standard include directory, as we are exporting custom headers
        .export_include_prefixes([])
        .export_include_directory(&header_dir, "cxx-qt-io")
        .reexport_dependency("cxx-qt-lib");

    let mut builder = CxxQtBuilder::library(interface)
        .build_cpp(&["core/qiodevice", "core/qlist/qlist", "core/qpair/qpair"])
        .build_rust(&[
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
            include_header!("include/core/qlist/qlist_qnetworkinterface.h"),
            include_header!("include/core/qpair/qpair_qhostaddress_i32.h"),
            include_header!("include/network/qabstractsocket.h"),
            include_header!("include/network/qhostaddress.h"),
            include_header!("include/network/qnetworkaddressentry.h"),
            include_header!("include/network/qnetworkdatagram.h"),
            include_header!("include/network/qnetworkinterface.h"),
            include_header!("include/network/qnetworkproxy.h"),
            include_header!("include/network/qnetworkrequest.h"),
            include_header!("include/network/qssl.h"),
        ]);

        let qssl_alternative_name_entry_bridge = if version.at_least(5, 13) {
            "network/qssl/alternative_name_entry_type_5_13"
        } else {
            "network/qssl/alternative_name_entry_type"
        };

        let qssl_protocol_bridge = if version.at_least(6, 3) {
            "network/qssl/protocol_6_3"
        } else if version.at_least(5, 12) {
            "network/qssl/protocol_5_12"
        } else {
            "network/qssl/protocol"
        };

        builder = builder
            .qt_module("Network")
            .build_cpp(&[
                "network/qhostaddress",
                "network/qnetworkaddressentry",
                "network/qnetworkdatagram",
                "network/qnetworkproxy",
            ])
            .build_rust(&[
                "core/qpair/qpair_qhostaddress_i32",
                "network/qabstractsocket",
                "network/qauthenticator",
                "network/qhostaddress",
                "network/qnetworkaddressentry",
                "network/qnetworkdatagram",
                "network/qnetworkinterface",
                "network/qnetworkproxy",
                "network/qnetworkrequest",
                "network/qssl/mod",
                qssl_alternative_name_entry_bridge,
                qssl_protocol_bridge,
                "network/qtcpsocket",
                "network/qudpsocket",
            ]);

        if version.at_least(6, 6) {
            header_dir.write_headers(&[include_header!("include/network/qhttpheaders.h")]);
            builder = builder
                .build_cpp(&["network/qhttpheaders"])
                .build_rust(&["network/qhttpheaders"]);
        }
    }

    builder.build();
}

use cxx_qt_build::CxxQtBuilder;
use qt_build_utils::QtBuild;
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

struct HeaderBuilder {
    dir: PathBuf,
}

impl HeaderBuilder {
    fn new() -> Self {
        let dir = PathBuf::from(env::var("OUT_DIR").unwrap())
            .join("include")
            .join("cxx-qt-io");
        Self { dir }
    }

    fn create_header_dir(&self) {
        fs::create_dir_all(&self.dir).expect("Failed to create include directory");
    }

    fn write_headers(&self, files: &[(&[u8], &OsStr)]) {
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

fn main() {
    let include_network = env::var("CARGO_FEATURE_QT_NETWORK").is_ok();

    let mut qt_modules = vec!["Core".to_owned()];
    if include_network {
        qt_modules.push("Network".to_owned());
    }

    let qtbuild = QtBuild::new(qt_modules).expect("Could not find Qt installation");
    let version = qtbuild.version();

    let header_dir = HeaderBuilder::new();

    header_dir.create_header_dir();

    header_dir.write_headers(&[
        include_header!("include/common.h"),
        include_header!("include/core/qfiledevice.h"),
        include_header!("include/core/qlist/qlist_private.h"),
        include_header!("include/core/qlist/qlist_qpair_qbytearray_qbytearray.h"),
        include_header!("include/core/qiodevice.h"),
        include_header!("include/core/qpair.h"),
        include_header!("include/core/qpair/qpair_qbytearray_qbytearray.h"),
        include_header!("include/core/qtemporaryfile.h"),
    ]);

    let interface = cxx_qt_build::Interface::default()
        // Disable exporting the standard include directory, as we are exporting custom headers
        .export_include_prefixes([])
        .export_include_directory(&header_dir, "cxx-qt-io")
        .reexport_dependency("cxx-qt-lib");

    let mut builder = CxxQtBuilder::library(interface)
        .build_cpp(&["core/qiodevice", "core/qpair/qpair"])
        .build_rust(&[
            "core/qiodevice",
            "core/qfiledevice",
            "core/qfile",
            "core/qlist/qlist_qpair_qbytearray_qbytearray",
            "core/qsavefile",
            "core/qtemporaryfile",
        ]);

    if include_network {
        header_dir.write_headers(&[
            include_header!("include/core/qpair/qpair_qhostaddress_i32.h"),
            include_header!("include/network/qabstractsocket.h"),
            include_header!("include/network/qhostaddress.h"),
            include_header!("include/network/qnetworkproxy.h"),
            include_header!("include/network/qnetworkrequest.h"),
        ]);
        builder = builder
            .qt_module("Network")
            .build_cpp(&["network/qhostaddress", "network/qnetworkproxy"])
            .build_rust(&[
                "core/qpair/qpair_qhostaddress_i32",
                "network/qauthenticator",
                "network/qabstractsocket",
                "network/qhostaddress",
                "network/qnetworkproxy",
                "network/qnetworkrequest",
            ]);

        if version.major > 6 || (version.major == 6 && version.minor > 6) {
            header_dir.write_headers(&[include_header!("include/network/qhttpheaders.h")]);
            builder = builder
                .build_cpp(&["network/qhttpheaders"])
                .build_rust(&["network/qhttpheaders"]);
        }
    }

    builder.build();
}

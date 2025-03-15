use cxx_qt_build::CxxQtBuilder;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn header_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
        .join("include")
        .join("cxx-qt-io")
}

macro_rules! include_header {
    ($file:literal) => {
        (include_bytes!($file), Path::new($file).file_name().unwrap())
    };
}

fn create_header_dir(subfolders: &[&str]) {
    let dir = header_dir();
    for subfolder in subfolders {
        let sub_dir = dir.join(subfolder);
        fs::create_dir_all(sub_dir).expect("Failed to create include directory: {sub_dir}");
    }
}

fn write_headers(files: &[(&[u8], &OsStr)]) {
    let out_dir = header_dir();
    std::fs::create_dir_all(out_dir.join("core")).expect("Failed to create include directory");

    for &(file_contents, file_name) in files {
        let out_path = out_dir.join(file_name);
        let mut header = File::create(out_path).expect("Could not create header: {file_name}");
        header
            .write_all(file_contents)
            .expect("Could not write header: {file_name}");
    }
}

#[allow(unused)]
fn build_cpp(builder: CxxQtBuilder, cpp_files: &[&str]) -> CxxQtBuilder {
    builder.cc_builder(move |cc| {
        for cpp_file in cpp_files {
            cc.file(format!("src/{cpp_file}.cpp"));
            println!("cargo::rerun-if-changed=src/{cpp_file}.cpp");
        }
    })
}

fn build_rust_bridges(mut builder: CxxQtBuilder, rust_bridges: &[&str]) -> CxxQtBuilder {
    for rust_source in rust_bridges {
        builder = builder.file(format!("src/{rust_source}.rs"));
    }
    builder
}

fn main() {
    create_header_dir(&["core"]);
    write_headers(&[
        include_header!("include/common.h"),
        include_header!("include/core/qfiledevice.h"),
        include_header!("include/core/qiodevice.h"),
    ]);

    let interface = cxx_qt_build::Interface::default()
        // Disable exporting the standard include directory, as we are exporting custom headers
        .export_include_prefixes([])
        .export_include_directory(header_dir(), "cxx-qt-io")
        .reexport_dependency("cxx-qt-lib");

    let mut builder = CxxQtBuilder::library(interface);

    if std::env::var("CARGO_FEATURE_QT_NETWORK").is_ok() {
        builder = builder.qt_module("Network");
    }

    builder = build_rust_bridges(
        builder,
        &[
            "core/qiodevice",
            "core/qfiledevice",
            "core/qfile",
            "core/qsavefile",
            "core/qtemporaryfile",
            "network/qabstractsocket",
        ],
    );

    // builder = build_cpp(builder, &["core/qfile"]);
    builder.build();
}

use cxx_qt_build::CxxQtBuilder;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let header_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("include")
        .join("cxx-qt-io-test-utils");
    fs::create_dir_all(&header_dir).expect("Failed to create include directory");
    let header_path = header_dir.join("test_utils.h");
    let mut header = File::create(header_path).expect("Could not create header");
    header
        .write_all(include_bytes!("test_utils.h"))
        .expect("Could not write header");

    let interface = cxx_qt_build::Interface::default()
        .export_include_prefixes([])
        .export_include_directory(&header_dir, "cxx-qt-io-test-utils");

    CxxQtBuilder::library(interface)
        .cc_builder(|cc| {
            cc.file("src/test_utils.cpp");
            println!("cargo::rerun-if-changed=src/test_utils.cpp");
        })
        .file("src/run_inside_app.rs")
        .file("src/test_context.rs")
        .build();
}

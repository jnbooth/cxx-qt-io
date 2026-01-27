use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .files([
            "src/qbuffer.rs",
            "src/qdeadlinetimer.rs",
            "src/qdtlsgeneratorparameters.rs",
            "src/qfile.rs",
        ])
        .build();
}

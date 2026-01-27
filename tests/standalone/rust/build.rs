use std::fs;

use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .files(fs::read_dir("src").unwrap().filter_map(|file| {
            let path = file.ok()?.path();
            let s = path.to_str()?;
            if s.starts_with("src/q") && s.ends_with(".rs") {
                Some(path)
            } else {
                None
            }
        }))
        .build();
}

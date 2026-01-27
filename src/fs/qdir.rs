use std::mem::MaybeUninit;

use cxx::{ExternType, type_id};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;
    }

    extern "C++" {
        include!("cxx-qt-io/qdir.h");
    }

    unsafe extern "C++" {
        type QDir = super::QDir;

        /// Adds `path` to the search path for `prefix`.
        #[Self = "QDir"]
        #[rust_name = "add_search_path"]
        fn addSearchPath(prefix: &QString, path: &QString);

        /// Returns `path` with directory separators normalized (that is, platform-native separators converted to `"/"`) and redundant ones removed, and `"."`s and `".."`s resolved (as far as possible).
        ///
        /// Symbolic links are kept. This function does not return the canonical path, but rather the simplest version of the input. For example, `"./local"` becomes `"local"`, `"local/../bin"` becomes `"bin"` and `"/local/usr/../bin"` becomes `"/local/bin"`.
        #[Self = "QDir"]
        #[rust_name = "clean_path"]
        fn cleanPath(path: &QString) -> QString;

        /// Returns the absolute path of the application's current directory. The current directory is the last directory set with [`QDir::set_current()`] or, if that was never called, the directory at which this application was started at by the parent process.
        #[Self = "QDir"]
        #[rust_name = "current_path"]
        fn currentPath() -> QString;

        /// Returns the absolute path of the user's home directory.
        ///
        /// If the directory of the current user's profile does not exist or cannot be retrieved, the following alternatives will be checked (in the given order) until an existing and available path is found:
        ///
        /// * The path specified by the `USERPROFILE` environment variable.
        /// * The path formed by concatenating the `HOMEDRIVE` and `HOMEPATH` environment variables.
        /// * The path specified by the `HOME` environment variable.
        /// * The path returned by [`QDir::root_path()`] (which uses the `SystemDrive` environment variable)
        /// * The C:/ directory.
        ///
        // Under non-Windows operating systems the `HOME` environment variable is used if it exists, otherwise the path returned by the  [`QDir::root_path()`].
        #[Self = "QDir"]
        #[rust_name = "home_path"]
        fn homePath() -> QString;

        /// Returns the absolute path of the root directory.
        ///
        /// For Unix operating systems this returns `"/"`. For Windows file systems this normally returns `"c:/"`.
        #[Self = "QDir"]
        #[rust_name = "root_path"]
        fn rootPath() -> QString;

        /// Returns the search paths for `prefix`.
        #[Self = "QDir"]
        #[rust_name = "search_paths"]
        fn searchPaths(prefix: &QString) -> QStringList;

        /// Sets the application's current working directory to `path`. Returns `true` if the directory was successfully changed; otherwise returns `false`.
        #[Self = "QDir"]
        #[rust_name = "set_current"]
        fn setCurrent(path: &QString) -> bool;

        /// Sets or replaces Qt's search paths for file names with the prefix `prefix` to `search_paths`.
        ///
        /// To specify a prefix for a file name, prepend the prefix followed by a single colon (e.g., `"images:undo.png"`, `"xmldocs:books.xml"`). `prefix` can only contain letters or numbers (e.g., it cannot contain a colon, nor a slash).
        ///
        /// Qt uses this search path to locate files with a known prefix. The search path entries are tested in order, starting with the first entry.
        ///
        /// File name prefix must be at least 2 characters long to avoid conflicts with Windows drive letters.
        ///
        /// Search paths may contain paths to [The Qt Resource System](https://doc.qt.io/qt-6/resources.html).
        #[Self = "QDir"]
        #[rust_name = "set_search_paths"]
        fn setSearchPaths(prefix: &QString, search_paths: &QStringList);

        /// Returns the absolute canonical path of the system's temporary directory.
        ///
        /// On Unix/Linux systems this is the path in the `TMPDIR` environment variable or `/tmp` if `TMPDIR` is not defined. On Windows this is usually the path in the `TEMP` or `TMP` environment variable. The path returned by this method doesn't end with a directory separator unless it is the root directory (of a drive).
        #[Self = "QDir"]
        #[rust_name = "temp_path"]
        fn tempPath() -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        #[rust_name = "qdir_drop"]
        fn drop(dir: &mut QDir);
    }
}

/// The `QDir` class provides access to directory structures and their contents.
///
/// Qt Documentation: [QDir](https://doc.qt.io/qt-6/qdir.html#details)
#[repr(C)]
pub struct QDir {
    _space: MaybeUninit<usize>,
}

impl Drop for QDir {
    fn drop(&mut self) {
        ffi::qdir_drop(self);
    }
}

// SAFETY: Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QDir {
    type Id = type_id!("QDir");
    type Kind = cxx::kind::Trivial;
}

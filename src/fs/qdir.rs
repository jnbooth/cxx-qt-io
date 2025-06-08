use cxx_qt_lib::{QString, QStringList};

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

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qdir_add_search_path"]
        fn qdirAddSearchPath(prefix: &QString, path: &QString);

        #[rust_name = "qdir_clean_path"]
        fn qdirCleanPath(path: &QString) -> QString;

        #[rust_name = "qdir_current_path"]
        fn qdirCurrentPath() -> QString;

        #[rust_name = "qdir_home_path"]
        fn qdirHomePath() -> QString;

        #[rust_name = "qdir_root_path"]
        fn qdirRootPath() -> QString;

        #[rust_name = "qdir_search_paths"]
        fn qdirSearchPaths(prefix: &QString) -> QStringList;

        #[rust_name = "qdir_set_current"]
        fn qdirSetCurrent(path: &QString) -> bool;

        #[rust_name = "qdir_set_search_paths"]
        fn qdirSetSearchPaths(prefix: &QString, search_paths: &QStringList);

        #[rust_name = "qdir_temp_path"]
        fn qdirTempPath() -> QString;
    }
}

/// The `QDir` class provides access to directory structures and their contents.
///
/// Qt Documentation: [QDir](https://doc.qt.io/qt-6/qdir.html#details)
pub struct QDir;

impl QDir {
    /// Adds `path` to the search path for `prefix`.
    pub fn add_search_path(prefix: &QString, path: &QString) {
        ffi::qdir_add_search_path(prefix, path);
    }

    /// Returns `path` with directory separators normalized (that is, platform-native separators converted to `"/"`) and redundant ones removed, and `"."`s and `".."`s resolved (as far as possible).
    ///
    /// Symbolic links are kept. This function does not return the canonical path, but rather the simplest version of the input. For example, `"./local"` becomes `"local"`, `"local/../bin"` becomes `"bin"` and `"/local/usr/../bin"` becomes `"/local/bin"`.
    pub fn clean_path(path: &QString) -> QString {
        ffi::qdir_clean_path(path)
    }

    /// Returns the absolute path of the application's current directory. The current directory is the last directory set with [`QDir::set_current()`] or, if that was never called, the directory at which this application was started at by the parent process.
    pub fn current_path() -> QString {
        ffi::qdir_current_path()
    }

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
    pub fn home_path() -> QString {
        ffi::qdir_home_path()
    }

    /// Returns the absolute path of the root directory.
    ///
    /// For Unix operating systems this returns `"/"`. For Windows file systems this normally returns `"c:/"`.
    pub fn root_path() -> QString {
        ffi::qdir_root_path()
    }

    /// Returns the search paths for `prefix`.
    pub fn search_paths(prefix: &QString) -> QStringList {
        ffi::qdir_search_paths(prefix)
    }

    /// Sets the application's current working directory to `path`. Returns `true` if the directory was successfully changed; otherwise returns `false`.
    pub fn set_current(path: &QString) -> bool {
        ffi::qdir_set_current(path)
    }

    /// Sets or replaces Qt's search paths for file names with the prefix `prefix` to `search_paths`.
    ///
    /// To specify a prefix for a file name, prepend the prefix followed by a single colon (e.g., `"images:undo.png"`, `"xmldocs:books.xml"`). `prefix` can only contain letters or numbers (e.g., it cannot contain a colon, nor a slash).
    ///
    /// Qt uses this search path to locate files with a known prefix. The search path entries are tested in order, starting with the first entry.
    ///
    /// File name prefix must be at least 2 characters long to avoid conflicts with Windows drive letters.
    ///
    /// Search paths may contain paths to [The Qt Resource System](https://doc.qt.io/qt-6/resources.html).
    pub fn set_search_paths(prefix: &QString, search_paths: &QStringList) {
        ffi::qdir_set_search_paths(prefix, search_paths);
    }

    /// Returns the absolute canonical path of the system's temporary directory.
    ///
    /// On Unix/Linux systems this is the path in the `TMPDIR` environment variable or `/tmp` if `TMPDIR` is not defined. On Windows this is usually the path in the `TEMP` or `TMP` environment variable. The path returned by this method doesn't end with a directory separator unless it is the root directory (of a drive).
    pub fn temp_path() -> QString {
        ffi::qdir_temp_path()
    }
}

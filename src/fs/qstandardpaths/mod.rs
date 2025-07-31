#[cfg(cxxqt_qt_version_at_least_6_7)]
mod v6_7;
#[cfg(cxxqt_qt_version_at_least_6_7)]
pub use v6_7::QStandardPaths;

#[cfg(all(cxxqt_qt_version_at_least_6_4, not(cxxqt_qt_version_at_least_6_7)))]
mod v6_4;
#[cfg(all(cxxqt_qt_version_at_least_6_4, not(cxxqt_qt_version_at_least_6_7)))]
pub use v6_4::QStandardPaths;

#[cfg(not(cxxqt_qt_version_at_least_6_4))]
mod v6_1;
#[cfg(not(cxxqt_qt_version_at_least_6_4))]
pub use v6_1::QStandardPaths;

pub type QStandardPathsStandardLocation = QStandardPaths;

use cxx_qt_lib::{QFlags, QString, QStringList};

use crate::util::IsNonNull;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    #[derive(PartialEq, Eq)]
    enum QStandardPathsLocateOption {
        LocateFile,
        LocateDirectory,
    }

    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;
    }

    extern "C++" {
        include!("cxx-qt-io/qstandardpaths.h");
        type QStandardPathsLocateOption;
        type QStandardPathsLocateOptions = super::QStandardPathsLocateOptions;
        type QStandardPathsStandardLocation = super::QStandardPathsStandardLocation;
    }

    #[namespace = "rust::cxxqtio1"]
    unsafe extern "C++" {
        #[rust_name = "qstandardpaths_display_name"]
        fn qstandardpathsDisplayName(location_type: QStandardPathsStandardLocation) -> QString;

        #[rust_name = "qstandardpaths_find_executable"]
        fn qstandardpathsFindExecutable(executable_name: &QString, paths: &QStringList) -> QString;

        #[rust_name = "qstandardpaths_locate"]
        fn qstandardpathsLocate(
            location_type: QStandardPathsStandardLocation,
            file_name: &QString,
            options: QStandardPathsLocateOptions,
        ) -> QString;

        #[rust_name = "qstandardpaths_locate_all"]
        fn qstandardpathsLocateAll(
            location_type: QStandardPathsStandardLocation,
            file_name: &QString,
            options: QStandardPathsLocateOptions,
        ) -> QStringList;

        #[rust_name = "qstandardpaths_standard_locations"]
        fn qstandardpathsStandardLocations(
            location_type: QStandardPathsStandardLocation,
        ) -> QStringList;

        #[rust_name = "qstandardpaths_writable_location"]
        fn qstandardpathsWritableLocation(location_type: QStandardPathsStandardLocation)
        -> QString;
    }
}

use ffi::QStandardPathsLocateOption;

/// [`QFlags`] of [`QStandardPathsLocateOption`].
type QStandardPathsLocateOptions = QFlags<QStandardPathsLocateOption>;
unsafe_impl_qflag!(QStandardPathsLocateOption, "QStandardPathsLocateOptions");

impl QStandardPaths {
    const LOCATE_DIR: QStandardPathsLocateOptions =
        QStandardPathsLocateOptions::from_int(QStandardPathsLocateOption::LocateDirectory.repr);

    const LOCATE_FILE: QStandardPathsLocateOptions =
        QStandardPathsLocateOptions::from_int(QStandardPathsLocateOption::LocateFile.repr);

    /// Returns a localized display name for the given location type or `None` if no relevant location can be found.
    pub fn display_name(self) -> Option<QString> {
        ffi::qstandardpaths_display_name(self).nonnull()
    }

    /// Finds the executable named `executable` in the system paths. On most operating systems the system path is determined by the `PATH` environment variable. Returns the absolute file path to the executable, or `None` if not found.
    ///
    /// If the given `executable_name` is an absolute path pointing to an executable, its clean path is returned.
    ///
    /// Symlinks are not resolved in order to preserve behavior for the case of executables whose behavior depends on the name they are invoked with.
    ///
    /// **Note:** On Windows, the usual executable extensions (from the `PATHEXT` environment variable) are automatically appended. For example, `QStandardPaths::find_executable_in("foo", &paths)` call finds foo.exe or foo.bat if present.
    pub fn find_executable(executable_name: &QString) -> Option<QString> {
        Self::find_executable_in(executable_name, &QStringList::default())
    }

    /// Finds the executable named `executable` in the specified paths. Returns the absolute file path to the executable, or `None` if not found.
    ///
    /// If the given `executable_name` is an absolute path pointing to an executable, its clean path is returned.
    ///
    /// Symlinks are not resolved in order to preserve behavior for the case of executables whose behavior depends on the name they are invoked with.
    ///
    /// **Note:** On Windows, the usual executable extensions (from the `PATHEXT` environment variable) are automatically appended. For example, `QStandardPaths::find_executable("foo")` call finds foo.exe or foo.bat if present.
    pub fn find_executable_in(executable_name: &QString, paths: &QStringList) -> Option<QString> {
        ffi::qstandardpaths_find_executable(executable_name, paths).nonnull()
    }

    /// Finds a file called `file_name` in the standard locations for this type.
    ///
    /// Returns the absolute path to the first file found, otherwise returns `None`.
    pub fn locate_file(self, file_name: &QString) -> Option<QString> {
        ffi::qstandardpaths_locate(self, file_name, Self::LOCATE_FILE).nonnull()
    }

    /// Finds a directory called `dir_name` in the standard locations for this type.
    ///
    /// Returns the absolute path to the first directory found, otherwise returns `None`.
    pub fn locate_dir(self, dir_name: &QString) -> Option<QString> {
        ffi::qstandardpaths_locate(self, dir_name, Self::LOCATE_DIR).nonnull()
    }

    /// Finds all files by the name `file_name` in the standard locations for this type.
    ///
    /// Returns the list of all the files that were found.
    pub fn locate_all_files(self, file_name: &QString) -> QStringList {
        ffi::qstandardpaths_locate_all(self, file_name, Self::LOCATE_FILE)
    }

    /// Finds all directories by the name `dir_name` in the standard locations for this type.
    ///
    /// Returns the list of all the directories that were found.
    pub fn locate_all_dirs(self, file_name: &QString) -> QStringList {
        ffi::qstandardpaths_locate_all(self, file_name, Self::LOCATE_DIR)
    }

    /// Returns all the directories where files of this type belong.
    ///
    /// The list of directories is sorted from high to low priority, starting with [`writable_location`](QStandardPaths::writable_location) if it can be determined. This list is empty if no locations for this type are defined.
    pub fn standard_locations(self) -> QStringList {
        ffi::qstandardpaths_standard_locations(self)
    }

    /// Returns the directory where files of this type should be written to, or `None` if the location cannot be determined.
    ///
    /// **Note:** The storage location returned may not exist; that is, it may need to be created by the system or the user.
    pub fn writable_location(self) -> Option<QString> {
        ffi::qstandardpaths_writable_location(self).nonnull()
    }
}

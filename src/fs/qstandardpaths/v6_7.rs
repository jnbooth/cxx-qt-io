#[cxx::bridge]
mod ffi {
    /// The `QStandardPaths` class provides methods for accessing standard paths.
    ///
    /// Qt Documentation: [QStandardPaths](https://doc.qt.io/qt-6/qstandardpaths.html#details)
    ///
    /// This enum describes the different locations that can be queried using methods such as [`QStandardPaths::writable_location`], [`QStandardPaths::standard_locations`], and [`QStandardPaths::display_name`].
    ///
    /// Some of the values in this enum represent a user configuration. Such enum values will return the same paths in different applications, so they could be used to share data with other applications. Other values are specific to this application. Each enum value in the table below describes whether it's application-specific or generic.
    ///
    /// Application-specific directories should be assumed to be unreachable by other applications. Therefore, files placed there might not be readable by other applications, even if run by the same user. On the other hand, generic directories should be assumed to be accessible by all applications run by this user, but should still be assumed to be unreachable by applications by other users.
    ///
    /// Data interchange with other users is out of the scope of [`QStandardPaths`].
    #[cxx_name = "QStandardPathsStandardLocation"]
    #[repr(i32)]
    enum QStandardPaths {
        /// Returns the user's desktop directory. This is a generic value. On systems with no concept of a desktop, this is the same as [`HomeLocation`](QStandardPaths::HomeLocation).
        DesktopLocation,
        /// Returns the directory containing user document files. This is a generic value. The returned path is never empty.
        DocumentsLocation,
        /// Returns the directory containing user's fonts. This is a generic value. Note that installing fonts may require additional, platform-specific operations.
        FontsLocation,
        /// Returns the directory containing the user applications (either executables, application bundles, or shortcuts to them). This is a generic value. Note that installing applications may require additional, platform-specific operations. Files, folders or shortcuts in this directory are platform-specific.
        ApplicationsLocation,
        /// Returns the directory containing the user's music or other audio files. This is a generic value. If no directory specific for music files exists, a sensible fallback for storing user documents is returned.
        MusicLocation,
        /// Returns the directory containing the user's movies and videos. This is a generic value. If no directory specific for movie files exists, a sensible fallback for storing user documents is returned.
        MoviesLocation,
        /// Returns the directory containing the user's pictures or photos. This is a generic value. If no directory specific for picture files exists, a sensible fallback for storing user documents is returned.
        PicturesLocation,
        /// Returns a directory where temporary files can be stored. The returned value might be application-specific, shared among other applications for this user, or even system-wide. The returned path is never empty.
        TempLocation,
        /// Returns the user's home directory (the same as [`QDir::home_path()`](crate::QDir::home_path)). On Unix systems, this is equal to the `HOME` environment variable. This value might be generic or application-specific, but the returned path is never empty.
        HomeLocation,
        /// Returns the local settings path on the Windows operating system. On all other platforms, it returns the same value as [`AppDataLocation`](QStandardPaths::AppDataLocation).
        AppLocalDataLocation,
        /// Returns a directory location where user-specific non-essential (cached) data should be written. This is an application-specific directory. The returned path is never empty.
        CacheLocation,
        /// Returns a directory location where user-specific non-essential (cached) data, shared across applications, should be written. This is a generic value. Note that the returned path may be empty if the system has no concept of shared cache.
        GenericDataLocation,
        /// Returns a directory location where persistent data shared across applications can be stored. This is a generic value. The returned path is never empty.
        RuntimeLocation,
        /// Returns a directory location where runtime communication files should be written, like Unix local sockets. This is a generic value. The returned path may be empty on some systems.
        ConfigLocation,
        /// Returns a directory location where user-specific configuration files should be written. This may be either a generic value or application-specific, and the returned path is never empty.
        DownloadLocation,
        /// Returns a directory for user's downloaded files. This is a generic value. If no directory specific for downloads exists, a sensible fallback for storing user documents is returned.
        GenericCacheLocation,
        /// Returns a directory location where user-specific configuration files shared between multiple applications should be written. This is a generic value and the returned path is never empty.
        GenericConfigLocation,
        /// Returns a directory location where persistent application data can be stored. This is an application-specific directory. To obtain a path to store data to be shared with other applications, use [`GenericDataLocation`](QStandardPaths::GenericDataLocation). The returned path is never empty. On the Windows operating system, this returns the roaming path.
        AppDataLocation,
        /// Returns a directory location where user-specific configuration files should be written. This is an application-specific directory, and the returned path is never empty.
        AppConfigLocation,
        /// Returns a directory location where user-specific publicly shared files and directories can be stored. This is a generic value. Note that the returned path may be empty if the system has no concept of a publicly shared location.
        ///
        /// Added in Qt 6.4.
        PublicShareLocation,
        /// Returns a directory location where user-specific template files can be stored. This is a generic value. Note that the returned path may be empty if the system has no concept of a templates location.
        ///
        /// Added in Qt 6.4.
        TemplatesLocation,
        /// Returns a directory location where user-specific application state data files should be written. This is an application-specific directory, and the returned path is never empty.
        ///
        /// Added in Qt 6.7.
        StateLocation,
        /// Returns a directory location where shared state data files across applications should be written. This value might be generic or application-specific, but the returned path is never empty.
        /// Added in Qt 6.7.
        GenericStateLocation,
    }

    extern "C++" {
        include!("cxx-qt-io/qstandardpaths.h");
        #[cxx_name = "QStandardPathsStandardLocation"]
        type QStandardPaths;
    }
}

pub use ffi::QStandardPaths;

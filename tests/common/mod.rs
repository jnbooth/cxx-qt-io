#![allow(unused)]

mod connect_errors;
pub use connect_errors::ConnectErrors;

mod ensure_app_is_running;
pub use ensure_app_is_running::ensure_app_is_running;

mod run_inside_app;
pub use run_inside_app::run_inside_app;

#[macro_export]
macro_rules! init_crates {
    () => {
        cxx_qt::init_crate!(cxx_qt_lib);
        cxx_qt::init_crate!(cxx_qt_lib_extras);
        cxx_qt::init_crate!(cxx_qt_io);
    };
}

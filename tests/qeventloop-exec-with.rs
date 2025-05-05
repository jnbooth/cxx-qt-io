mod common;
use common::ensure_app_is_running;
use cxx_qt_io::QEventLoop;

#[test]
pub fn qeventloop_exec_with() {
    ensure_app_is_running();
    let mut increment_count = 0;
    QEventLoop::new().pin_mut().exec_with(|| {
        increment_count += 1;
    });
    assert_eq!(increment_count, 1);
}

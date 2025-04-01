#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
use cxx_qt::Upcast;

use crate::QIODevice;
use std::ffi::c_char;
use std::io;
use std::pin::Pin;

#[allow(unused_variables)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) trait QIO: Upcast<QIODevice> {
    fn flush(self: Pin<&mut Self>) -> bool {
        true
    }

    fn get_error_kind(&self) -> io::ErrorKind {
        io::ErrorKind::Other
    }
}

pub(crate) trait QIOExt {
    fn read(self: Pin<&mut Self>, buf: &mut [u8]) -> io::Result<usize>;

    fn write(self: Pin<&mut Self>, buf: &[u8]) -> io::Result<usize>;

    fn flush(self: Pin<&mut Self>) -> io::Result<()>;
}

#[cold]
fn get_error<T: QIO>(device: &T) -> io::Error {
    let error_kind = device.get_error_kind();
    let error_string = device.upcast().error_string();
    io::Error::new(error_kind, String::from(&error_string))
}

impl<T: QIO> QIOExt for T {
    fn read(mut self: Pin<&mut Self>, buf: &mut [u8]) -> io::Result<usize> {
        let buf_ptr = buf.as_mut_ptr().cast::<c_char>();
        let device = self.as_mut().upcast_pin();
        // SAFETY: buf_ptr is valid and its size is not greater than buf.len().
        let result = unsafe { device.read_unsafe(buf_ptr, buf.len() as i64) };
        if let Ok(n) = usize::try_from(result) {
            return Ok(n);
        }
        Err(get_error(&*self))
    }

    fn write(mut self: Pin<&mut Self>, buf: &[u8]) -> io::Result<usize> {
        let buf_ptr = buf.as_ptr().cast::<c_char>();
        let device = self.as_mut().upcast_pin();
        // SAFETY: buf_ptr is valid and its size is not greater than buf.len().
        let result = unsafe { device.write_unsafe(buf_ptr, buf.len() as i64) };
        if let Ok(n) = usize::try_from(result) {
            return Ok(n);
        }
        Err(get_error(&*self))
    }

    fn flush(mut self: Pin<&mut Self>) -> io::Result<()> {
        if QIO::flush(self.as_mut()) {
            Ok(())
        } else {
            Err(get_error(&*self))
        }
    }
}

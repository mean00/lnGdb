#![no_std]
use gdbstub::conn::Connection;

pub struct CdcConnection {
}

impl CdcConnection {
    pub fn new(port: u16) -> Result<CdcConnection, &'static str> {
        unsafe {

            Ok(CdcConnection {  })
        }
    }

    pub fn read(&mut self) -> Result<u8, &'static str> {
            Err("socket read failed")
    }

    #[allow(dead_code)]
    pub fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            Err("socket peek failed")
    }
}

impl Drop for CdcConnection {
    fn drop(&mut self) {
    }
}

impl Connection for CdcConnection {
    type Error = &'static str;
    fn write(&mut self, b: u8) -> Result<(), &'static str> {
            Err("socket write failed")
    }

    fn flush(&mut self) -> Result<(), &'static str> {
        // huh, apparently flushing isn't a "thing" for Tcp streams.
        // see https://doc.rust-lang.org/src/std/net/tcp.rs.html#592-609
        Ok(())
    }
}

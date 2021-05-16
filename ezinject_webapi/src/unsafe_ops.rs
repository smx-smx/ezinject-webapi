use std::{u8, usize};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

use libc::c_void;
#[cfg(unix)]
use libloading::os::unix::*;
#[cfg(windows)]
use libloading::os::windows::*;

pub fn mem_read(src: *const u8, size: usize) -> Vec<u8> {
    let mut buf :Vec<u8> = Vec::with_capacity(size);
    unsafe {
        buf.set_len(size);
        std::ptr::copy::<u8>(src, buf.as_mut_ptr(), size);
    }
    buf
}

pub fn mem_write(addr: * mut u8, data: Vec<u8>){
    use libc::{PROT_READ, PROT_WRITE, PROT_EXEC};

    unsafe {
        // HAX (no Windows!)
        libc::mprotect(
            addr as * mut c_void,
            data.len(),
            PROT_READ|PROT_WRITE|PROT_EXEC
        );

        std::ptr::copy::<u8>(
            data.as_ptr(),
            addr,
            data.len()
        )
    }
}

pub fn peek<T>(src: *const T) -> T
{
    unsafe {std::ptr::read(src)}
}

pub fn poke<T>(dst: *mut T, value: T) -> Result<()>
{
    unsafe {std::ptr::write(dst, value)}
    Ok(())
}

pub fn dlopen_self() -> Library {
    Library::this()
}

pub fn dlopen(library_name: &str) -> Result<Library> {
    Ok(
        unsafe {Library::new(library_name)?}
    )
}

type PfnGeneric = unsafe extern fn() -> c_void;

pub fn dlsym(handle: libc::uintptr_t, sym: String) -> Result<Symbol<PfnGeneric>> {
    unsafe {
        let lib = Library::from_raw(handle as *mut c_void);
        let func:Symbol<PfnGeneric> = lib.get(sym.as_bytes())?;
        Ok(func)
    }
}
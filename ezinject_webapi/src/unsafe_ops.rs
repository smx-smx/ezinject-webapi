use std::{u8, usize};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[cfg(unix)] type HandleType =  *mut c_void;
#[cfg(windows)] type HandleType = *mut winapi::shared::minwindef::HINSTANCE__;

#[cfg(unix)] use libloading::os::unix::*;
#[cfg(windows)] use libloading::os::windows::*;


pub fn mem_read(src: *const u8, size: usize) -> Vec<u8> {
    let mut buf :Vec<u8> = Vec::with_capacity(size);
    unsafe {
        buf.set_len(size);
        std::ptr::copy::<u8>(src, buf.as_mut_ptr(), size);
    }
    buf
}

pub fn mem_write(addr: * mut u8, data: Vec<u8>){
    unsafe {
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

#[cfg(unix)]
pub fn dlopen_self() -> Library {
    Library::this()
}

#[cfg(windows)]
pub fn dlopen_self() -> Library {
    Library::this().unwrap()
}

pub fn dlopen(library_name: &str) -> Result<Library> {
    Ok(
        unsafe {Library::new(library_name)?}
    )
}

type PfnGeneric = unsafe extern fn() -> libc::c_void;

pub fn dlsym(handle: libc::uintptr_t, sym: String) -> Result<Symbol<PfnGeneric>> {
    unsafe {
        let lib = Library::from_raw(handle as HandleType);
        let func:Symbol<PfnGeneric> = lib.get(sym.as_bytes())?;
        Ok(func)
    }
}
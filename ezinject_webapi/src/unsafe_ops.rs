use libloading;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub fn peek<T>(src: *const T) -> T
{
    unsafe {std::ptr::read(src)}
}

pub fn poke<T>(dst: *mut T, value: T) -> Result<()>
{
    unsafe {std::ptr::write(dst, value)}
    Ok(())
}

pub fn dlopen_self() -> libloading::os::unix::Library {
    libloading::os::unix::Library::this()
}

pub fn dlopen(library_name: &str) -> Result<libloading::Library> {
    Ok(
        unsafe {libloading::Library::new(library_name)?}
    )
}
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
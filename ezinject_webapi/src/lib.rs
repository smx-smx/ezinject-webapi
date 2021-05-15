extern crate rweb;

pub mod server;
mod unsafe_ops;

use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};


#[repr(C)]
pub struct InjcodeUser {
    persist: u8,
}

/// Pre-initialization phase
#[no_mangle]
pub extern "C" fn lib_preinit(user: &mut InjcodeUser) -> i32 {
    user.persist = 1;
    return 0;
}

/// Main entrypoint called by ezinject preloader (fileloader)
#[no_mangle]
pub extern "C" fn lib_main(_argc: i32, _argv: *mut *mut i8) -> i32 {
    println!("Hello from Rust");

    if let Err(_) = std::fs::write("/tmp/rusthook.log", "Lorem ipsum\n") {
        println!("Failed to write file\n");
    } else {
        println!("File successfully written!\n");
    }

    println!("Starting thread...\n");
    std::thread::spawn(|| {
        println!("Starting HTTP server");
        if let Err(err) = server::start_server_task(
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 
            8000
        ) {
            println!("Failed to start HTTP server {:?}\n", err);
        } else {
            println!("HTTP Server started!\n");
        }
    });

    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

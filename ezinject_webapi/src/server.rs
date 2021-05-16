use std::time::Duration;
use std::net::{IpAddr};
use std::convert;
use tokio::sync::oneshot;
use tokio::runtime::Runtime;

//use std::ffi::CString;
//use libc::{dlopen, c_void, c_char};
use crate::unsafe_ops;
use rweb::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[get("/dlopen/self")]
pub fn dlopen_self() -> String {
    let library = unsafe_ops::dlopen_self();
    format!("{:?}", library)
}

#[get("/dlopen/{lib_name}")]
pub fn dlopen_library(lib_name: String) -> String {
    match unsafe_ops::dlopen(&lib_name[..]) {
        Ok(handle) => format!("dlopen_library: {:?} => {:?}", lib_name, handle).to_owned(),
        Err(err) => format!("dlopen_library failed! library={:?}, error={:?}", lib_name, err)
    }
}

#[get("/peek/{addr}")]
pub fn peek(addr: String) -> String {
    if !addr.starts_with("0x") {
        return "Invalid value, expecting arg in hex notation (0x...)".to_owned();
    }

    let addr_primitive = match u64::from_str_radix(&addr[2..], 16) {
        Ok(value) => value,
        Err(_) => {
            return format!("Failed to convert value {:?}", addr);
        }
    };

    let data = unsafe_ops::peek(addr_primitive as *mut u64);
    format!("0x{:x}", data)
}

#[get("/poke")]
pub fn poke() -> String {
    "poke".to_owned()
}

#[router("/api", services(dlopen_self, dlopen_library, peek, poke))]
fn api() {}

pub fn start_server_task(address: IpAddr, port: u16) -> Result<()> {
    // Create the runtime
    let rt = Runtime::new().unwrap();
    // Execute the future, blocking the current thread until completion
    rt.block_on(async {    
        let (tx, rx) = oneshot::channel();

        println!("[+] Creating server");
        let (_, server) = serve(api())
            .bind_with_graceful_shutdown((address, port), async {
                rx.await.ok();
            }
        );
        
        // Spawn the server into a runtime
        println!("[+] Spawning server");
        tokio::task::spawn(server);
        
        println!("[+] Looping infinitely");
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;
        }

        println!("[+] Shutting down server");
        let _ = tx.send(());
    });

    Ok(())
}
use std::time::Duration;
use std::net::{IpAddr};
use tokio::sync::oneshot;
use tokio::runtime::Runtime;

//use std::ffi::CString;
//use libc::{dlopen, c_void, c_char};
use rweb::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[get("/dlopen")]
pub fn dlopen_route() -> String {
    "dlopen".to_owned()
}

#[get("/peek")]
pub fn peek_route() -> String {
    "poke".to_owned()
}

#[get("/poke")]
pub fn poke_route() -> String {
    "poke".to_owned()
}

#[router("/api", services(dlopen_route, peek_route, poke_route))]
fn api_route() {}

pub fn start_server_task(address: IpAddr, port: u16) -> Result<()> {
    // Create the runtime
    let rt = Runtime::new().unwrap();
    // Execute the future, blocking the current thread until completion
    rt.block_on(async {    
        let (tx, rx) = oneshot::channel();

        println!("[+] Creating server");
        let (_, server) = serve(api_route())
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
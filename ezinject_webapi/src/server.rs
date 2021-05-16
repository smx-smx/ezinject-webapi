use std::{convert::TryInto, io::Bytes, num::ParseIntError, time::Duration, u64, usize};
use std::net::{IpAddr};
use std::convert;
use tokio::sync::oneshot;
use tokio::runtime::Runtime;

use serde::Deserialize;

//use std::ffi::CString;
//use libc::{dlopen, c_void, c_char};
use crate::unsafe_ops;
use rweb::*;
use rweb::hyper::*;

type WebResult<T> = std::result::Result<T, Rejection>;
type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

macro_rules! unwrap_or_return {
    ( $e:expr, $res:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return $res //return format!($msg),
        }
    }
}

fn bad_request(text: &str) -> Response<String> {
    Response::builder()
        .header("Content-Type", "text/plain")
        .status(400)
        .body(text.to_string())
        .unwrap()
}

fn bad_data_request(text: &str) -> Response<Vec<u8>> {
    Response::builder()
        .header("Content-Type", "text/plain")
        .status(400)
        .body(text.as_bytes().to_vec())
        .unwrap()
}

#[derive(Debug, Deserialize)]
struct PeekArgs {
    addr: String,
    size: String,
    format: String
}

#[derive(Debug, Deserialize)]
struct PokeArgs {
    addr: String
}

#[derive(Debug, Deserialize)]
struct DlopenArgs {
    library: String
}

#[get("/dlopen/self")]
pub fn dlopen_self() -> String {
    let library = unsafe_ops::dlopen_self();
    format!("{:?}", library)
}

#[get("/dlopen")]
pub fn dlopen_library(qs: Query<DlopenArgs>) -> Response<String> {
    let args = qs.into_inner();
    let lib_name = args.library.as_str();

    let lib = unwrap_or_return!(unsafe_ops::dlopen(lib_name), bad_request("dlopen failed"));
    
    Response::builder()
        .header("Content-Type", "text/plain")
        .body(format!("{:p}", lib.into_raw()))
        .unwrap()
}


fn decode_number(addr: String) -> std::result::Result<u64, ParseIntError> {
    let (input, radix) = match addr.starts_with("0x") {
        true => (&addr[2..], 16),
        false => (addr.as_str(), 10)
    };

    return u64::from_str_radix(input, radix);
}

#[post("/poke")]
pub fn poke(
    qs: Query<PokeArgs>,
    #[body] data: rweb::hyper::body::Bytes
) -> Response<String> {
    let args:PokeArgs = qs.into_inner();

    let addr = unwrap_or_return!(decode_number(args.addr), bad_request("malformed address"));

    println!("Writing {:?} at {:x}", data.len(), addr);

    unsafe_ops::mem_write(addr as *mut u8, data.to_vec());

    Response::builder()
        .status(StatusCode::OK)
        .body("".to_string())
        .unwrap()
}

#[get("/peek")]
pub fn peek(qs: Query<PeekArgs>) -> Response<Vec<u8>>/* Vec<u8>*/ {
    let args:PeekArgs = qs.into_inner();

    let addr = unwrap_or_return!(decode_number(args.addr), bad_data_request("malformed address"));
    let size:usize = unwrap_or_return!(decode_number(args.size), bad_data_request("malformed size")) as usize;

    println!("Reading {:?} at {:x}", size, addr);

    let data = unsafe_ops::mem_read(addr as *mut u8, size);

    let response = match args.format.as_str() {
        "base64" => Response::builder()
            .header("Content-Type", "text/plain")
            .header("Content-Transfer-Encoding", "base64")
            .body(base64::encode(data).as_bytes().to_vec()),
        "bin" => Response::builder()
            .header("Content-Type", "application/octet-stream")
            .body(data),
        "hex" | _ => Response::builder()
            .header("Content-Type", "text/plain")
            .body(data.iter().map(
                |x| format!("{:x}", x)
            ).collect::<String>().as_bytes().to_vec()),
    };
    response.unwrap()
}

#[router("/api/v1", services(dlopen_self, dlopen_library, peek, poke))]
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
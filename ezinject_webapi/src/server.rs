use core::ffi;
use std::{any::Any, convert::TryInto, io::Bytes, num::ParseIntError, time::Duration, u64, usize};
use std::net::{IpAddr};
use std::convert;
use libc::{c_void, uintptr_t};
use libffi::{high::{Arg, CType, Type}, middle::{Cif, CodePtr, FfiAbi, arg, ffi_abi_FFI_DEFAULT_ABI}, raw::ffi_arg};
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

fn internal_error(text: &str) -> Response<String> {
    Response::builder()
        .header("Content-Type", "text/plain")
        .status(500)
        .body(text.to_string())
        .unwrap()
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

#[derive(Debug, Deserialize)]
struct DlsymArgs {
    handle: String,
    sym: String
}

#[derive(Debug, Deserialize)]
struct CallArgs {
    fptr: String,
    args: Vec<String>,
    abi: Option<FfiAbi>
}

#[cfg(unix)] const OS_IDENT:&str = "unix";
#[cfg(windows)] const OS_IDENT:&str = "windows";

#[get("/cfg")]
pub fn cfg_info() -> String {
    OS_IDENT.to_string()
}

#[post("/call")]
pub fn call(#[json] args: CallArgs) -> Response<String> {
    let fptr_value = unwrap_or_return!(decode_number(args.fptr), bad_request("malformed fptr")) as *const c_void;
    let code_ptr = CodePtr::from_ptr(fptr_value);

    let arg_values:Vec<uintptr_t> = args.args.iter().map(
        |x| -> uintptr_t {
            unwrap_or_return!(
                decode_number(x.to_string()),
                0 as uintptr_t
            ) as uintptr_t
        }
    ).collect();

    let arg_types = vec![libffi::middle::Type::usize(); arg_values.len()];
    let mut ffi_args:Vec<libffi::middle::Arg> = Vec::with_capacity(arg_values.len());

    for i in 0..arg_values.len() {
        let arg_ref = arg_values.get(i).unwrap();
        let arg = libffi::middle::Arg::new(arg_ref);
        ffi_args.push(arg);
    }

    let mut cif = libffi::middle::Cif::new(arg_types, libffi::middle::Type::pointer());
    if let Some(value) = args.abi {
        cif.set_abi(value);
    }
    
    let result: *const c_void;
    unsafe {
        result = cif.call(code_ptr, &ffi_args);
    }

    Response::builder()
        .status(StatusCode::OK)
        .body(format!("{:p}", result))
        .unwrap()
}

#[get("/dlsym")]
pub fn dlsym(qs: Query<DlsymArgs>) -> Response<String> {
    let args = qs.into_inner();
    let handle = unwrap_or_return!(decode_number(args.handle), bad_request("malformed handle"));

    let fptr = unwrap_or_return!(unsafe_ops::dlsym(handle, args.sym), internal_error("dlsym failed"));

    Response::builder()
        .status(StatusCode::OK)
        .body(format!("{:p}", fptr.into_raw()))
        .unwrap()
}

#[get("/dlopen/self")]
pub fn dlopen_self() -> Response<String> {
    let lib = unsafe_ops::dlopen_self();
    Response::builder()
        .header("Content-Type", "text/plain")
        .body(format!("{:p}", lib.into_raw()))
        .unwrap()
}


#[get("/dlopen")]
pub fn dlopen_library(qs: Query<DlopenArgs>) -> Response<String> {
    let args = qs.into_inner();
    let lib_name = args.library.as_str();

    let lib = unwrap_or_return!(unsafe_ops::dlopen(lib_name), internal_error("dlopen failed"));
    
    Response::builder()
        .header("Content-Type", "text/plain")
        .body(format!("{:p}", lib.into_raw()))
        .unwrap()
}


fn decode_number(addr: String) -> std::result::Result<libc::uintptr_t, ParseIntError> {
    let (input, radix) = match addr.starts_with("0x") {
        true => (&addr[2..], 16),
        false => (addr.as_str(), 10)
    };

    return uintptr_t::from_str_radix(input, radix);
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

#[router("/api/v1", services(cfg_info, dlopen_self, dlopen_library, dlsym, call, peek, poke))]
fn api() {}

pub fn start_server_task(address: IpAddr, port: u16) -> Result<()> {
    // Create the runtime
    let rt = Runtime::new().unwrap();

    let (tx, rx) = oneshot::channel();

    // Execute the future, blocking the current thread until completion
    rt.block_on(async {    
        println!("[+] Creating server");
        let (_, server) = serve(api())
            .bind_with_graceful_shutdown((address, port), async {
                rx.await.ok();
            }
        );

        // Spawn the server into a runtime
        println!("[+] Spawning server");
        let _ = tokio::task::spawn(server).await.expect("server has panicked");
    });

    println!("[+] Shutting down server");
    let _ = tx.send(());

    return Ok(())
}
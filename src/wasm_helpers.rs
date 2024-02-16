#[cfg(target_family = "wasm")]
use std::os::raw::*;
#[cfg(not(target_family = "wasm"))]
use std::sync::mpsc::Receiver;
#[cfg(target_family = "wasm")]
use std::sync::mpsc::{channel, Receiver, Sender};

#[cfg(not(target_family = "wasm"))]
#[allow(dead_code)]
pub fn save_data(_data: &[u8]) -> std::io::Result<()> {
    Err(std::io::Error::other("Unimplemented for native"))
}

#[cfg(not(target_family = "wasm"))]
#[allow(dead_code)]
pub fn load_data() -> std::io::Result<Receiver<Vec<u8>>> {
    Err(std::io::Error::other("Unimplemented for native"))
}

#[cfg(target_family = "wasm")]
#[no_mangle]
pub extern "C" fn ld45_load_rust_handler(usr: *mut c_void, data: *const c_void, len: c_int) {
    let sender_box: Box<Sender<Vec<u8>>> = unsafe { Box::from_raw(usr as *mut Sender<Vec<u8>>) };

    if data.is_null() || len == 0 {
        (*sender_box).send(Vec::new()).ok();
        drop(sender_box);
        println!("callback: Failed to load data!");
        return;
    }

    let v: Vec<u8> =
        unsafe { std::slice::from_raw_parts(data as *const u8, len as usize).to_owned() };

    (*sender_box).send(v).ok();
    println!("callback: Loaded data!");

    drop(sender_box);
}

#[cfg(target_family = "wasm")]
extern "C" {
    fn ld45_save_async(data: *const c_void, length: c_int);
    fn ld45_load_async(usr: *const c_void);
}

#[cfg(target_family = "wasm")]
pub fn save_data(data: &[u8]) -> std::io::Result<()> {
    unsafe {
        ld45_save_async(data as *const [u8] as *const c_void, data.len() as c_int);
    }
    Ok(())
}

#[cfg(target_family = "wasm")]
pub fn load_data() -> std::io::Result<Receiver<Vec<u8>>> {
    let (tx, rx) = channel();
    let handler = Box::new(tx);

    unsafe {
        let ptr = Box::into_raw(handler);
        ld45_load_async(ptr as *mut c_void);
    }

    Ok(rx)
}

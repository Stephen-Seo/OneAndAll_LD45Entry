use ld45_lib::agnostic_interface;

fn will_close() -> bool {
    unsafe { agnostic_interface::raylib_impl::ffi::WindowShouldClose() }
}

fn main() {
    let state_ptr = ld45_lib::ld45_initialize();

    while !will_close() {
        ld45_lib::ld45_iterate(state_ptr);
    }

    unsafe {
        drop(Box::from_raw(state_ptr as *mut ld45_lib::WasmState));
    }
}

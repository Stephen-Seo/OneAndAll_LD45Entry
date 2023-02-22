use agnostic_interface::raylib_impl::RaylibGame;
use faux_quicksilver::Window;
use original_impl::GameState;

mod agnostic_interface;
mod faux_quicksilver;
mod original_impl;
mod shaders;

struct WasmState {
    pub window: Window,
    pub game_state: GameState,
}

#[no_mangle]
pub extern "C" fn ld45_initialize() -> *mut ::std::os::raw::c_void {
    let game_interface = RaylibGame::new_boxed(800, 600);
    let mut window = Window::new(game_interface);
    let game_state = GameState::new(&mut window).unwrap();

    Box::into_raw(Box::new(WasmState { window, game_state })) as *mut ::std::os::raw::c_void
}

#[no_mangle]
pub extern "C" fn ld45_iterate(context: *mut ::std::os::raw::c_void) {
    let state_ptr = context as *mut WasmState;
    unsafe {
        (*state_ptr).window.update_music().unwrap();
        (*state_ptr)
            .game_state
            .update(&mut (*state_ptr).window)
            .unwrap();
        (*state_ptr)
            .game_state
            .draw(&mut (*state_ptr).window)
            .unwrap();
    }
}

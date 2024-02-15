pub mod agnostic_interface;
mod faux_quicksilver;
mod original_impl;
mod shaders;
mod wasm_helpers;

use agnostic_interface::raylib_impl::RaylibGame;
use faux_quicksilver::Window;
use original_impl::GameState;

pub struct WasmState {
    pub window: Box<Window>,
    pub game_state: Box<GameState>,
}

impl WasmState {
    pub fn get_window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn get_state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
    }
}

#[no_mangle]
pub extern "C" fn ld45_initialize() -> *mut ::std::os::raw::c_void {
    let game_interface = RaylibGame::new_boxed(800, 600);
    let mut window = Box::new(Window::new(game_interface));
    let game_state = Box::new(GameState::new(&mut window).unwrap());

    Box::into_raw(Box::new(WasmState { window, game_state })) as *mut ::std::os::raw::c_void
}

#[no_mangle]
pub extern "C" fn ld45_iterate(context: *mut ::std::os::raw::c_void) {
    let state_ptr = context as *mut WasmState;
    unsafe {
        (*state_ptr).get_window_mut().update_music().unwrap();
        (*state_ptr)
            .get_state_mut()
            .update((*state_ptr).get_window_mut())
            .unwrap();
        (*state_ptr)
            .get_state_mut()
            .draw((*state_ptr).get_window_mut())
            .unwrap();
    }
}

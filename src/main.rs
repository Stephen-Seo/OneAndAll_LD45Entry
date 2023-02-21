use agnostic_interface::raylib_impl::RaylibGame;
use faux_quicksilver::Window;
use original_impl::GameState;

mod agnostic_interface;
mod faux_quicksilver;
mod original_impl;
mod shaders;

fn will_close() -> bool {
    unsafe { agnostic_interface::raylib_impl::ffi::WindowShouldClose() }
}

fn main() {
    // TODO
    //run::<GameState>(
    //    "One And All - a Ludum Dare 45 compo entry",
    //    Vector::new(800, 600),
    //    Settings::default(),
    //);

    let game_interface = RaylibGame::new_boxed(800, 600);
    let mut window = Window::new(game_interface);
    let mut game_state = GameState::new(&mut window).unwrap();

    while !will_close() {
        window.update_music().unwrap();
        game_state.update(&mut window).unwrap();
        game_state.draw(&mut window).unwrap();
    }
}

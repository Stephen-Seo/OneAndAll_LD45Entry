use crate::faux_quicksilver::Color;

pub trait ImageInterface {
    fn draw(&self, x: f32, y: f32) -> Result<(), String>;
}

pub trait FontInterface {
    fn draw(&self, s: &str) -> Result<(), String>;
}

pub trait SoundInterface {
    fn play(&self, vol: f32) -> Result<(), String>;
}

pub trait WindowInterface {
    fn get_dimensions(&self) -> Result<(f32, f32), String>;
    fn get_key_pressed(&self, key: char) -> Result<bool, String>;
    fn get_mouse_pressed(&self) -> Result<Option<(f32, f32)>, String>;
    fn clear_window(&self, color: Color) -> Result<(), String>;
    fn begin_drawing(&self) -> Result<(), String>;
    fn end_drawing(&self) -> Result<(), String>;
}

pub mod raylib_impl;

use std::path::Path;

use crate::faux_quicksilver::{Circle, Color, Rectangle, Transform, Vector};

pub trait ImageInterface {
    fn draw(&mut self, x: f32, y: f32, color: Color) -> Result<(), String>;
    fn draw_sub(&mut self, sub_rect: Rectangle, x: f32, y: f32, color: Color)
        -> Result<(), String>;
    fn draw_transform(
        &mut self,
        x: f32,
        y: f32,
        color: Color,
        transform: Transform,
    ) -> Result<(), String>;
    fn draw_sub_transform(
        &mut self,
        sub_rect: Rectangle,
        x: f32,
        y: f32,
        color: Color,
        transform: Transform,
    ) -> Result<(), String>;
    fn get_w(&self) -> usize;
    fn get_h(&self) -> usize;
    fn get_wh_rect(&self) -> Rectangle;
}

pub trait FontInterface {
    fn draw(&mut self, s: &str, size: u32, x: f32, y: f32, color: Color) -> Result<(), String>;
}

pub trait SoundInterface {
    fn play(&mut self, vol: f32) -> Result<(), String>;
}

pub trait MusicInterface {
    fn play(&mut self, vol: f32) -> Result<(), String>;
    fn pause(&mut self) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;
    fn set_loop(&mut self, loop_enable: bool) -> Result<(), String>;
}

pub trait CameraInterface {
    fn get_view(&self) -> Result<Rectangle, String>;
    fn get_view_xy(&self) -> Result<(f32, f32), String>;
    fn set_view(&mut self, rect: Rectangle) -> Result<(), String>;
    fn set_view_xy(&mut self, x: f32, y: f32) -> Result<(), String>;
}

pub trait GameInterface {
    fn get_dimensions(&self) -> Result<(f32, f32), String>;
    fn get_key_pressed(&mut self, key: char) -> Result<bool, String>;
    fn get_mouse_pressed(&mut self) -> Result<Option<(f32, f32)>, String>;
    fn get_mouse_down(&mut self) -> Result<Option<(f32, f32)>, String>;
    fn get_mouse_xy(&self) -> Result<(f32, f32), String>;
    fn get_mouse_xy_vec(&self) -> Result<Vector, String>;
    fn get_delta_time(&self) -> f32;
    fn clear_window(&mut self, color: Color) -> Result<(), String>;
    fn begin_drawing(&mut self) -> Result<(), String>;
    fn end_drawing(&mut self) -> Result<(), String>;

    fn draw_circle(&mut self, circle: Circle, color: Color) -> Result<(), String>;
    fn draw_circle_ex(
        &mut self,
        circle: Circle,
        color: Color,
        origin: Vector,
        rot: f32,
    ) -> Result<(), String>;
    fn draw_circle_transform(
        &mut self,
        circle: Circle,
        color: Color,
        transform: Transform,
    ) -> Result<(), String>;
    fn draw_rect(&mut self, rect: Rectangle, color: Color) -> Result<(), String>;
    fn draw_rect_ex(
        &mut self,
        rect: Rectangle,
        color: Color,
        origin: Vector,
        rot: f32,
    ) -> Result<(), String>;
    fn draw_rect_transform(
        &mut self,
        rect: Rectangle,
        color: Color,
        transform: Transform,
    ) -> Result<(), String>;

    fn load_image(&mut self, path: &Path) -> Result<Box<dyn ImageInterface>, String>;
    fn load_font(&mut self, path: &Path) -> Result<Box<dyn FontInterface>, String>;
    fn load_sound(&mut self, path: &Path) -> Result<Box<dyn SoundInterface>, String>;
    fn load_music(&mut self, path: &Path) -> Result<Box<dyn MusicInterface>, String>;

    fn get_camera(&mut self) -> Result<Box<dyn CameraInterface>, String>;
    fn get_default_camera(&mut self) -> Result<Box<dyn CameraInterface>, String>;
    fn set_camera(&mut self, camera: &Box<dyn CameraInterface>) -> Result<(), String>;
}

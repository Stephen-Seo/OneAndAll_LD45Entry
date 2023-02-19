mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/raylib_bindings.rs"));
}

use super::GameInterface;

struct RaylibGame {
    width: f32,
    height: f32,
}

impl RaylibGame {
    pub fn new_boxed(width: f32, height: f32) -> Box<dyn GameInterface> {
        Box::new(Self {
            width,
            height,
        })
    }
}

impl GameInterface for RaylibGame {
    fn get_dimensions(&self) -> Result<(f32, f32), String> {
        Ok((self.width, self.height))
    }

    fn get_key_pressed(&mut self, key: char) -> Result<bool, String> {
        todo!()
    }

    fn get_mouse_pressed(&mut self) -> Result<Option<(f32, f32)>, String> {
        todo!()
    }

    fn get_mouse_down(&mut self) -> Result<Option<(f32, f32)>, String> {
        todo!()
    }

    fn get_mouse_xy(&self) -> Result<(f32, f32), String> {
        todo!()
    }

    fn get_mouse_xy_vec(&self) -> Result<crate::faux_quicksilver::Vector, String> {
        todo!()
    }

    fn get_delta_time(&self) -> f32 {
        todo!()
    }

    fn clear_window(&mut self, color: crate::faux_quicksilver::Color) -> Result<(), String> {
        todo!()
    }

    fn begin_drawing(&mut self) -> Result<(), String> {
        todo!()
    }

    fn end_drawing(&mut self) -> Result<(), String> {
        todo!()
    }

    fn draw_circle(&mut self, circle: crate::faux_quicksilver::Circle, color: crate::faux_quicksilver::Color) -> Result<(), String> {
        todo!()
    }

    fn draw_circle_ex(
        &mut self,
        circle: crate::faux_quicksilver::Circle,
        color: crate::faux_quicksilver::Color,
        origin: crate::faux_quicksilver::Vector,
        rot: f32,
    ) -> Result<(), String> {
        todo!()
    }

    fn draw_circle_transform(
        &mut self,
        circle: crate::faux_quicksilver::Circle,
        color: crate::faux_quicksilver::Color,
        transform: crate::faux_quicksilver::Transform,
    ) -> Result<(), String> {
        todo!()
    }

    fn draw_rect(&mut self, rect: crate::faux_quicksilver::Rectangle, color: crate::faux_quicksilver::Color) -> Result<(), String> {
        todo!()
    }

    fn draw_rect_ex(
        &mut self,
        rect: crate::faux_quicksilver::Rectangle,
        color: crate::faux_quicksilver::Color,
        origin: crate::faux_quicksilver::Vector,
        rot: f32,
    ) -> Result<(), String> {
        todo!()
    }

    fn draw_rect_transform(
        &mut self,
        rect: crate::faux_quicksilver::Rectangle,
        color: crate::faux_quicksilver::Color,
        transform: crate::faux_quicksilver::Transform,
    ) -> Result<(), String> {
        todo!()
    }

    fn load_image(&mut self, path: &std::path::Path) -> Result<Box<dyn super::ImageInterface>, String> {
        todo!()
    }

    fn load_font(&mut self, path: &std::path::Path) -> Result<Box<dyn super::FontInterface>, String> {
        todo!()
    }

    fn load_sound(&mut self, path: &std::path::Path) -> Result<Box<dyn super::SoundInterface>, String> {
        todo!()
    }

    fn load_music(&mut self, path: &std::path::Path) -> Result<Box<dyn super::MusicInterface>, String> {
        todo!()
    }

    fn get_camera(&mut self) -> Result<Box<dyn super::CameraInterface>, String> {
        todo!()
    }

    fn get_default_camera(&mut self) -> Result<Box<dyn super::CameraInterface>, String> {
        todo!()
    }

    fn set_camera(&mut self, camera: &Box<dyn super::CameraInterface>) -> Result<(), String> {
        todo!()
    }
}

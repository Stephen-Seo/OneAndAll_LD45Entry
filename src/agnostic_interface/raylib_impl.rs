mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]

    include!(concat!(env!("OUT_DIR"), "/raylib_bindings.rs"));
}

use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::CString,
    os::raw::{c_char, c_int},
    rc::Rc,
};

use crate::faux_quicksilver::Vector;

use super::{FontInterface, GameInterface, ImageInterface, MusicInterface, SoundInterface};

fn fqcolor_to_color(c: crate::faux_quicksilver::Color) -> ffi::Color {
    ffi::Color {
        r: c.r,
        g: c.g,
        b: c.b,
        a: c.a,
    }
}

fn fqrect_to_rect(r: crate::faux_quicksilver::Rectangle) -> ffi::Rectangle {
    ffi::Rectangle {
        x: r.x,
        y: r.y,
        width: r.w,
        height: r.h,
    }
}

fn fqvector_to_vector2(v: crate::faux_quicksilver::Vector) -> ffi::Vector2 {
    ffi::Vector2 { x: v.x, y: v.y }
}

#[derive(Clone, Debug)]
struct RaylibImage {
    image: ffi::Image,
    texture: Option<ffi::Texture>,
}

#[derive(Clone, Debug)]
struct RaylibImageHandler {
    image: Rc<RefCell<RaylibImage>>,
}

impl RaylibImageHandler {
    fn image_to_texture(&mut self) -> Result<(), String> {
        if self.image.borrow().texture.is_none() {
            unsafe {
                self.image.borrow_mut().texture =
                    Some(ffi::LoadTextureFromImage(self.image.borrow().image));
            }
        }
        Ok(())
    }
}

impl ImageInterface for RaylibImageHandler {
    fn draw(
        &mut self,
        x: f32,
        y: f32,
        color: crate::faux_quicksilver::Color,
    ) -> Result<(), String> {
        self.image_to_texture()?;
        unsafe {
            ffi::DrawTexture(
                *self
                    .image
                    .borrow()
                    .texture
                    .as_ref()
                    .ok_or_else(|| String::from("RaylibImage has no Texture!"))?,
                x.round() as i32,
                y.round() as i32,
                fqcolor_to_color(color),
            );
        }
        Ok(())
    }

    fn draw_sub(
        &mut self,
        sub_rect: crate::faux_quicksilver::Rectangle,
        dest_rect: crate::faux_quicksilver::Rectangle,
        color: crate::faux_quicksilver::Color,
    ) -> Result<(), String> {
        self.image_to_texture()?;
        unsafe {
            ffi::DrawTexturePro(
                *self
                    .image
                    .borrow()
                    .texture
                    .as_ref()
                    .ok_or_else(|| String::from("RaylibImage has no Texture!"))?,
                fqrect_to_rect(sub_rect),
                fqrect_to_rect(dest_rect),
                ffi::Vector2 { x: 0.0, y: 0.0 },
                0.0,
                fqcolor_to_color(color),
            );
        }
        Ok(())
    }

    fn draw_transform(
        &mut self,
        x: f32,
        y: f32,
        color: crate::faux_quicksilver::Color,
        transform: crate::faux_quicksilver::Transform,
        origin: Vector,
    ) -> Result<(), String> {
        todo!()
    }

    fn draw_sub_transform(
        &mut self,
        sub_rect: crate::faux_quicksilver::Rectangle,
        dest_rect: crate::faux_quicksilver::Rectangle,
        color: crate::faux_quicksilver::Color,
        transform: crate::faux_quicksilver::Transform,
        origin: Vector,
    ) -> Result<(), String> {
        todo!()
    }

    fn get_w(&self) -> usize {
        self.image.borrow().image.width as usize
    }

    fn get_h(&self) -> usize {
        self.image.borrow().image.height as usize
    }

    fn get_wh_rect(&self) -> crate::faux_quicksilver::Rectangle {
        crate::faux_quicksilver::Rectangle {
            x: 0.0,
            y: 0.0,
            w: self.image.borrow().image.width as f32,
            h: self.image.borrow().image.height as f32,
        }
    }
}

#[derive(Clone, Debug)]
struct RaylibFont {
    font: ffi::Font,
}

#[derive(Clone, Debug)]
struct RaylibFontHandler {
    font: Rc<RaylibFont>,
}

impl FontInterface for RaylibFontHandler {
    fn draw(
        &mut self,
        s: &str,
        size: u32,
        x: f32,
        y: f32,
        color: crate::faux_quicksilver::Color,
    ) -> Result<(), String> {
        unsafe {
            let cstring = CString::from_vec_unchecked(s.as_bytes().into());
            ffi::DrawTextEx(
                self.font.font,
                cstring.as_ptr(),
                ffi::Vector2 { x, y },
                size as f32,
                (size / 10) as f32,
                fqcolor_to_color(color),
            );
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct RaylibSound {
    sound: ffi::Sound,
}

#[derive(Clone, Debug)]
struct RaylibSoundHandler {
    sound: Rc<RaylibSound>,
}

impl SoundInterface for RaylibSoundHandler {
    fn play(&mut self, vol: f32) -> Result<(), String> {
        unsafe {
            ffi::PlaySound(self.sound.sound);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct RaylibMusic {
    music: ffi::Music,
}

impl RaylibMusic {
    pub fn update(&mut self) {
        unsafe {
            ffi::UpdateMusicStream(self.music);
        }
    }
}

#[derive(Clone, Debug)]
struct RaylibMusicHandler {
    music: Rc<RefCell<RaylibMusic>>,
}

impl MusicInterface for RaylibMusicHandler {
    fn play(&mut self, vol: f32) -> Result<(), String> {
        unsafe {
            ffi::SetMusicVolume(self.music.borrow().music, vol);
            ffi::PlayMusicStream(self.music.borrow().music);
        }
        Ok(())
    }

    fn pause(&mut self) -> Result<(), String> {
        unsafe {
            ffi::PauseMusicStream(self.music.borrow().music);
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        unsafe {
            ffi::StopMusicStream(self.music.borrow().music);
        }
        Ok(())
    }

    fn set_loop(&mut self, loop_enable: bool) -> Result<(), String> {
        self.music.borrow_mut().music.looping = loop_enable;
        Ok(())
    }

    fn update(&mut self) -> Result<(), String> {
        self.music.borrow_mut().update();
        Ok(())
    }
}

struct RaylibGame {
    images: HashMap<String, Rc<RefCell<RaylibImage>>>,
    fonts: HashMap<String, Rc<RaylibFont>>,
    sounds: HashMap<String, Rc<RaylibSound>>,
    music: HashMap<String, Rc<RefCell<RaylibMusic>>>,
}

impl RaylibGame {
    pub fn new_boxed(width: u32, height: u32) -> Box<dyn GameInterface> {
        unsafe {
            let string = "One and All LD45\0";
            ffi::InitWindow(
                width as c_int,
                height as c_int,
                string.as_ptr() as *const c_char,
            );
        }
        Box::new(Self {
            images: HashMap::new(),
            fonts: HashMap::new(),
            sounds: HashMap::new(),
            music: HashMap::new(),
        })
    }
}

impl Drop for RaylibGame {
    fn drop(&mut self) {
        unsafe {
            for (_, image) in &self.images {
                if let Some(texture) = image.borrow_mut().texture.take() {
                    ffi::UnloadTexture(texture);
                }
                ffi::UnloadImage(image.borrow().image);
            }
            for (_, font) in &self.fonts {
                ffi::UnloadFont(font.font);
            }
            for (_, sound) in &self.sounds {
                ffi::UnloadSound(sound.sound);
            }
            for (_, music) in &self.music {
                ffi::UnloadMusicStream(music.borrow().music);
            }
            ffi::CloseWindow();
        }
    }
}

impl GameInterface for RaylibGame {
    fn get_dimensions(&self) -> Result<(f32, f32), String> {
        unsafe { Ok((ffi::GetScreenWidth() as f32, ffi::GetScreenHeight() as f32)) }
    }

    fn get_key_pressed(&mut self, key: char) -> Result<bool, String> {
        unsafe { Ok(ffi::IsKeyPressed(key.to_ascii_uppercase() as c_int)) }
    }

    fn get_mouse_pressed(&mut self) -> Result<Option<(f32, f32)>, String> {
        unsafe {
            if ffi::IsMouseButtonPressed(0) {
                Ok(Some((ffi::GetTouchX() as f32, ffi::GetTouchY() as f32)))
            } else {
                Ok(None)
            }
        }
    }

    fn get_mouse_down(&mut self) -> Result<Option<(f32, f32)>, String> {
        unsafe {
            if ffi::IsMouseButtonDown(0) {
                Ok(Some((ffi::GetTouchX() as f32, ffi::GetTouchY() as f32)))
            } else {
                Ok(None)
            }
        }
    }

    fn get_mouse_xy(&self) -> Result<(f32, f32), String> {
        unsafe { Ok((ffi::GetTouchX() as f32, ffi::GetTouchY() as f32)) }
    }

    fn get_mouse_xy_vec(&self) -> Result<crate::faux_quicksilver::Vector, String> {
        unsafe {
            Ok(crate::faux_quicksilver::Vector {
                x: ffi::GetTouchX() as f32,
                y: ffi::GetTouchY() as f32,
            })
        }
    }

    fn get_delta_time(&self) -> f32 {
        unsafe { ffi::GetFrameTime() }
    }

    fn clear_window(&mut self, color: crate::faux_quicksilver::Color) -> Result<(), String> {
        unsafe {
            ffi::ClearBackground(fqcolor_to_color(color));
        }
        Ok(())
    }

    fn begin_drawing(&mut self) -> Result<(), String> {
        unsafe {
            ffi::BeginDrawing();
        }
        Ok(())
    }

    fn end_drawing(&mut self) -> Result<(), String> {
        unsafe {
            ffi::EndDrawing();
        }
        Ok(())
    }

    fn draw_circle(
        &mut self,
        circle: crate::faux_quicksilver::Circle,
        color: crate::faux_quicksilver::Color,
    ) -> Result<(), String> {
        unsafe {
            ffi::DrawCircle(
                circle.x.round() as i32,
                circle.y.round() as i32,
                circle.r,
                fqcolor_to_color(color),
            );
        }
        Ok(())
    }

    fn draw_circle_transform(
        &mut self,
        circle: crate::faux_quicksilver::Circle,
        color: crate::faux_quicksilver::Color,
        transform: crate::faux_quicksilver::Transform,
        origin: Vector,
    ) -> Result<(), String> {
        todo!()
    }

    fn draw_rect(
        &mut self,
        rect: crate::faux_quicksilver::Rectangle,
        color: crate::faux_quicksilver::Color,
    ) -> Result<(), String> {
        unsafe {
            ffi::DrawRectangle(
                rect.x.round() as i32,
                rect.y.round() as i32,
                rect.w.round() as i32,
                rect.h.round() as i32,
                fqcolor_to_color(color),
            );
        }
        Ok(())
    }

    fn draw_rect_ex(
        &mut self,
        rect: crate::faux_quicksilver::Rectangle,
        color: crate::faux_quicksilver::Color,
        origin: crate::faux_quicksilver::Vector,
        rot: f32,
    ) -> Result<(), String> {
        unsafe {
            ffi::DrawRectanglePro(
                fqrect_to_rect(rect),
                fqvector_to_vector2(origin),
                rot,
                fqcolor_to_color(color),
            );
        }
        Ok(())
    }

    fn draw_rect_transform(
        &mut self,
        rect: crate::faux_quicksilver::Rectangle,
        color: crate::faux_quicksilver::Color,
        transform: crate::faux_quicksilver::Transform,
        origin: Vector,
    ) -> Result<(), String> {
        todo!()
    }

    fn load_image(
        &mut self,
        path: &std::path::Path,
    ) -> Result<Box<dyn super::ImageInterface>, String> {
        unsafe {
            let path_str = path
                .to_str()
                .ok_or_else(|| format!("Failed to convert \"{path:?}\" to str!"))?;
            let path_buf: Vec<u8> = path_str.as_bytes().into();
            let cstring: CString = CString::from_vec_unchecked(path_buf);
            let image = ffi::LoadImage(cstring.as_ptr());
            let raylib_image_handler = RaylibImageHandler {
                image: Rc::new(RefCell::new(RaylibImage {
                    image,
                    texture: None,
                })),
            };
            self.images
                .insert(path_str.to_owned(), raylib_image_handler.image.clone());
            Ok(Box::new(raylib_image_handler))
        }
    }

    fn load_font(
        &mut self,
        path: &std::path::Path,
    ) -> Result<Box<dyn super::FontInterface>, String> {
        unsafe {
            let path_str = path
                .to_str()
                .ok_or_else(|| format!("Failed to convert \"{path:?}\" to str!"))?;
            let path_buf: Vec<u8> = path_str.as_bytes().into();
            let cstring: CString = CString::from_vec_unchecked(path_buf);
            let font = ffi::LoadFont(cstring.as_ptr());
            let raylib_font_handler = RaylibFontHandler {
                font: Rc::new(RaylibFont { font }),
            };
            self.fonts
                .insert(path_str.to_owned(), raylib_font_handler.font.clone());
            Ok(Box::new(raylib_font_handler))
        }
    }

    fn load_sound(
        &mut self,
        path: &std::path::Path,
    ) -> Result<Box<dyn super::SoundInterface>, String> {
        unsafe {
            let path_str = path
                .to_str()
                .ok_or_else(|| format!("Failed to convert \"{path:?}\" to str!"))?;
            let path_buf: Vec<u8> = path_str.as_bytes().into();
            let cstring: CString = CString::from_vec_unchecked(path_buf);
            let sound = ffi::LoadSound(cstring.as_ptr());
            let raylib_sound_handler = RaylibSoundHandler {
                sound: Rc::new(RaylibSound { sound }),
            };
            self.sounds
                .insert(path_str.to_owned(), raylib_sound_handler.sound.clone());
            Ok(Box::new(raylib_sound_handler))
        }
    }

    fn load_music(
        &mut self,
        path: &std::path::Path,
    ) -> Result<Box<dyn super::MusicInterface>, String> {
        unsafe {
            let path_str = path
                .to_str()
                .ok_or_else(|| format!("Failed to convert \"{path:?}\" to str!"))?;
            let path_buf: Vec<u8> = path_str.as_bytes().into();
            let cstring: CString = CString::from_vec_unchecked(path_buf);
            let music = ffi::LoadMusicStream(cstring.as_ptr());
            let raylib_music_handler = RaylibMusicHandler {
                music: Rc::new(RefCell::new(RaylibMusic { music })),
            };
            self.music
                .insert(path_str.to_owned(), raylib_music_handler.music.clone());
            Ok(Box::new(raylib_music_handler))
        }
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

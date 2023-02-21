pub mod ffi {
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
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
};

use crate::{
    faux_quicksilver::{Transform, Vector},
    shaders::{get_attrib_location, set_attr_2f, set_transform_3f},
};

use super::{
    CameraInterface, FontInterface, GameInterface, ImageInterface, MusicInterface, ShaderInterface,
    SoundInterface,
};

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
pub struct RaylibShader {
    shader: ffi::Shader,
}

impl RaylibShader {
    pub fn get_shader_id(&self) -> ::std::os::raw::c_uint {
        self.shader.id as ::std::os::raw::c_uint
    }

    fn set_transform_attrib(&mut self, transform: Transform) -> Result<(), String> {
        let transform_cstr = CString::new("transform")
            .map_err(|_| String::from("Failed to create \"transform\" CString!"))?;
        let attr_loc = get_attrib_location(self, &transform_cstr);
        set_transform_3f(attr_loc, transform);
        Ok(())
    }

    fn begin_draw_shader(&self) -> Result<(), String> {
        unsafe {
            ffi::BeginShaderMode(self.shader);
        }
        Ok(())
    }

    fn end_draw_shader(&self) -> Result<(), String> {
        unsafe {
            ffi::EndShaderMode();
        }
        Ok(())
    }

    fn set_origin_attrib(&mut self, origin: Vector) -> Result<(), String> {
        let origin_cstr = CString::new("origin")
            .map_err(|_| String::from("Failed to create \"origin\" CString!"))?;
        let attr_loc = get_attrib_location(self, &origin_cstr);
        set_attr_2f(attr_loc, origin);
        Ok(())
    }

    fn set_camera_attrib(&mut self, camera: Vector) -> Result<(), String> {
        let camera_cstr = CString::new("camera")
            .map_err(|_| String::from("Failed to create \"camera\" CString!"))?;
        let attr_loc = get_attrib_location(self, &camera_cstr);
        set_attr_2f(attr_loc, camera);
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct RaylibShaderHandler {
    shader: Rc<RefCell<RaylibShader>>,
}

impl ShaderInterface for RaylibShaderHandler {
    fn set_transform_attrib(&mut self, transform: Transform) -> Result<(), String> {
        self.shader.borrow_mut().set_transform_attrib(transform)
    }

    fn begin_draw_shader(&self) -> Result<(), String> {
        self.shader.borrow().begin_draw_shader()
    }

    fn end_draw_shader(&self) -> Result<(), String> {
        self.shader.borrow().end_draw_shader()
    }

    fn set_origin_attrib(&mut self, origin: Vector) -> Result<(), String> {
        self.shader.borrow_mut().set_origin_attrib(origin)
    }

    fn set_camera_attrib(&mut self, camera: Vector) -> Result<(), String> {
        self.shader.borrow_mut().set_camera_attrib(camera)
    }
}

impl RaylibShaderHandler {
    pub fn load_shader(vs: &Path, fs: &Path) -> Result<Self, String> {
        unsafe {
            let vs_cstr: CString = CString::from_vec_unchecked(
                vs.to_str()
                    .ok_or_else(|| format!("Cannot convert path \"{vs:?}\" to str!"))?
                    .as_bytes()
                    .to_owned(),
            );
            let fs_cstr: CString = CString::from_vec_unchecked(
                fs.to_str()
                    .ok_or_else(|| format!("Cannot convert path \"{fs:?}\" to str!"))?
                    .as_bytes()
                    .to_owned(),
            );
            Ok(Self {
                shader: Rc::new(RefCell::new(RaylibShader {
                    shader: ffi::LoadShader(vs_cstr.as_ptr(), fs_cstr.as_ptr()),
                })),
            })
        }
    }
}

#[derive(Clone, Debug)]
struct RaylibImage {
    image: ffi::Image,
    texture: Option<ffi::Texture>,
}

#[derive(Clone, Debug)]
struct RaylibImageHandler {
    image: Rc<RefCell<RaylibImage>>,
    tr_or_cam_shader: Rc<RefCell<Option<RaylibShaderHandler>>>,
    cam_shader: Rc<RefCell<Option<RaylibShaderHandler>>>,
    camera: Rc<RefCell<Camera>>,
}

impl RaylibImageHandler {
    fn image_to_texture(&mut self) -> Result<(), String> {
        if self.image.borrow().texture.is_none() {
            unsafe {
                let texture = ffi::LoadTextureFromImage(self.image.borrow().image);
                self.image.borrow_mut().texture = Some(texture);
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
        if let Some(cam_shader) = self.cam_shader.borrow_mut().as_mut() {
            cam_shader.set_camera_attrib(self.camera.borrow().pos)?;
            cam_shader.begin_draw_shader()?;
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
            cam_shader.end_draw_shader()?;
        } else {
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
        if let Some(cam_shader) = self.cam_shader.borrow_mut().as_mut() {
            cam_shader.set_camera_attrib(self.camera.borrow().pos)?;
            cam_shader.begin_draw_shader()?;
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
            cam_shader.end_draw_shader()?;
        } else {
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
        self.image_to_texture()?;
        if let Some(shader) = self.tr_or_cam_shader.borrow_mut().as_mut() {
            shader.set_origin_attrib(origin)?;
            shader.set_transform_attrib(transform)?;
            shader.set_camera_attrib(self.camera.borrow().pos)?;
            shader.begin_draw_shader()?;
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
            shader.end_draw_shader()?;
        } else if let Some(cam_shader) = self.cam_shader.borrow_mut().as_mut() {
            cam_shader.set_camera_attrib(self.camera.borrow().pos)?;
            cam_shader.begin_draw_shader()?;
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
            cam_shader.end_draw_shader()?;
        } else {
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
        }
        Ok(())
    }

    fn draw_sub_transform(
        &mut self,
        sub_rect: crate::faux_quicksilver::Rectangle,
        dest_rect: crate::faux_quicksilver::Rectangle,
        color: crate::faux_quicksilver::Color,
        transform: crate::faux_quicksilver::Transform,
        origin: Vector,
    ) -> Result<(), String> {
        self.image_to_texture()?;
        if let Some(shader) = self.tr_or_cam_shader.borrow_mut().as_mut() {
            shader.set_origin_attrib(origin)?;
            shader.set_transform_attrib(transform)?;
            shader.set_camera_attrib(self.camera.borrow().pos)?;
            shader.begin_draw_shader()?;
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
            shader.end_draw_shader()?;
        } else if let Some(cam_shader) = self.cam_shader.borrow_mut().as_mut() {
            cam_shader.set_camera_attrib(self.camera.borrow().pos)?;
            cam_shader.begin_draw_shader()?;
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
            cam_shader.end_draw_shader()?;
        } else {
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
        }
        Ok(())
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
            ffi::SetSoundVolume(self.sound.sound, vol);
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

#[derive(Debug, Clone)]
struct Camera {
    pos: Vector,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pos: Vector { x: 0.0, y: 0.0 },
        }
    }
}

impl CameraInterface for Camera {
    fn get_view_xy(&self) -> Result<(f32, f32), String> {
        Ok((self.pos.x, self.pos.y))
    }

    fn set_view_xy(&mut self, x: f32, y: f32) -> Result<(), String> {
        self.pos.x = x;
        self.pos.y = y;
        Ok(())
    }
}

pub struct RaylibGame {
    images: HashMap<String, Rc<RefCell<RaylibImage>>>,
    fonts: HashMap<String, Rc<RaylibFont>>,
    sounds: HashMap<String, Rc<RaylibSound>>,
    music: HashMap<String, Rc<RefCell<RaylibMusic>>>,
    shaders: HashMap<String, Rc<RefCell<RaylibShader>>>,
    camera: Rc<RefCell<Camera>>,
}

impl RaylibGame {
    #[cfg(not(target_arch = "wasm32"))]
    fn native_setup() {
        unsafe {
            ffi::SetTargetFPS(60);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn native_setup() {}

    pub fn new_boxed(width: u32, height: u32) -> Box<dyn GameInterface> {
        unsafe {
            let string = "One and All LD45\0";
            ffi::InitWindow(
                width as c_int,
                height as c_int,
                string.as_ptr() as *const c_char,
            );
        }

        Self::native_setup();
        unsafe {
            ffi::InitAudioDevice();
        }

        let mut self_unboxed = Self {
            images: HashMap::new(),
            fonts: HashMap::new(),
            sounds: HashMap::new(),
            music: HashMap::new(),
            shaders: HashMap::new(),
            camera: Rc::new(RefCell::new(Camera::default())),
        };
        if let Err(e) = self_unboxed.load_transform_origin_shader() {
            println!("WARNING: {e:?}");
        }
        if let Err(e) = self_unboxed.load_camera_shader() {
            println!("WARNING: {e:?}");
        }
        Box::new(self_unboxed)
    }
}

impl Drop for RaylibGame {
    fn drop(&mut self) {
        unsafe {
            for (_, shader) in &self.shaders {
                ffi::UnloadShader(shader.borrow().shader);
            }
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

    fn get_mouse_released(&mut self) -> Result<bool, String> {
        unsafe {
            if ffi::IsMouseButtonReleased(0) {
                Ok(true)
            } else {
                Ok(false)
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
        if let Some(cam_shader) = self.shaders.get_mut("camera") {
            cam_shader
                .borrow_mut()
                .set_camera_attrib(self.camera.borrow().pos)?;
            cam_shader.borrow().begin_draw_shader()?;
            unsafe {
                ffi::DrawCircle(
                    circle.x.round() as i32,
                    circle.y.round() as i32,
                    circle.r,
                    fqcolor_to_color(color),
                );
            }
            cam_shader.borrow().end_draw_shader()?;
        } else {
            unsafe {
                ffi::DrawCircle(
                    circle.x.round() as i32,
                    circle.y.round() as i32,
                    circle.r,
                    fqcolor_to_color(color),
                );
            }
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
        if let Some(shader) = self.shaders.get_mut("transform_origin") {
            shader.borrow_mut().set_origin_attrib(origin)?;
            shader.borrow_mut().set_transform_attrib(transform)?;
            shader
                .borrow_mut()
                .set_camera_attrib(self.camera.borrow().pos)?;
            shader.borrow().begin_draw_shader()?;
            unsafe {
                ffi::DrawCircle(
                    circle.x.round() as i32,
                    circle.y.round() as i32,
                    circle.r,
                    fqcolor_to_color(color),
                );
            }
            shader.borrow().end_draw_shader()?;
        } else if let Some(cam_shader) = self.shaders.get_mut("camera") {
            cam_shader
                .borrow_mut()
                .set_camera_attrib(self.camera.borrow().pos)?;
            cam_shader.borrow().begin_draw_shader()?;
            unsafe {
                ffi::DrawCircle(
                    circle.x.round() as i32,
                    circle.y.round() as i32,
                    circle.r,
                    fqcolor_to_color(color),
                );
            }
            cam_shader.borrow().end_draw_shader()?;
        } else {
            unsafe {
                ffi::DrawCircle(
                    circle.x.round() as i32,
                    circle.y.round() as i32,
                    circle.r,
                    fqcolor_to_color(color),
                );
            }
        }
        Ok(())
    }

    fn draw_rect(
        &mut self,
        rect: crate::faux_quicksilver::Rectangle,
        color: crate::faux_quicksilver::Color,
    ) -> Result<(), String> {
        if let Some(cam_shader) = self.shaders.get_mut("camera") {
            cam_shader
                .borrow_mut()
                .set_camera_attrib(self.camera.borrow().pos)?;
            cam_shader.borrow().begin_draw_shader()?;
            unsafe {
                ffi::DrawRectangle(
                    rect.x.round() as i32,
                    rect.y.round() as i32,
                    rect.w.round() as i32,
                    rect.h.round() as i32,
                    fqcolor_to_color(color),
                );
            }
            cam_shader.borrow().end_draw_shader()?;
        } else {
            unsafe {
                ffi::DrawRectangle(
                    rect.x.round() as i32,
                    rect.y.round() as i32,
                    rect.w.round() as i32,
                    rect.h.round() as i32,
                    fqcolor_to_color(color),
                );
            }
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
        if let Some(cam_shader) = self.shaders.get_mut("camera") {
            cam_shader
                .borrow_mut()
                .set_camera_attrib(self.camera.borrow().pos)?;
            cam_shader.borrow().begin_draw_shader()?;
            unsafe {
                ffi::DrawRectanglePro(
                    fqrect_to_rect(rect),
                    fqvector_to_vector2(origin),
                    rot,
                    fqcolor_to_color(color),
                );
            }
            cam_shader.borrow().end_draw_shader()?;
        } else {
            unsafe {
                ffi::DrawRectanglePro(
                    fqrect_to_rect(rect),
                    fqvector_to_vector2(origin),
                    rot,
                    fqcolor_to_color(color),
                );
            }
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
        if let Some(shader) = self.shaders.get_mut("transform_origin") {
            shader.borrow_mut().set_origin_attrib(origin)?;
            shader.borrow_mut().set_transform_attrib(transform)?;
            shader
                .borrow_mut()
                .set_camera_attrib(self.camera.borrow().pos)?;
            shader.borrow().begin_draw_shader()?;
            unsafe {
                ffi::DrawRectangle(
                    rect.x.round() as i32,
                    rect.y.round() as i32,
                    rect.w.round() as i32,
                    rect.h.round() as i32,
                    fqcolor_to_color(color),
                );
            }
            shader.borrow().end_draw_shader()?;
        } else if let Some(cam_shader) = self.shaders.get_mut("camera") {
            cam_shader
                .borrow_mut()
                .set_camera_attrib(self.camera.borrow().pos)?;
            cam_shader.borrow().begin_draw_shader()?;
            unsafe {
                ffi::DrawRectangle(
                    rect.x.round() as i32,
                    rect.y.round() as i32,
                    rect.w.round() as i32,
                    rect.h.round() as i32,
                    fqcolor_to_color(color),
                );
            }
            cam_shader.borrow().end_draw_shader()?;
        } else {
            unsafe {
                ffi::DrawRectangle(
                    rect.x.round() as i32,
                    rect.y.round() as i32,
                    rect.w.round() as i32,
                    rect.h.round() as i32,
                    fqcolor_to_color(color),
                );
            }
        }
        Ok(())
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
            let tr_or_cam_shader: Option<RaylibShaderHandler> =
                if let Some(shader) = self.shaders.get("transform_origin") {
                    Some(RaylibShaderHandler {
                        shader: shader.clone(),
                    })
                } else {
                    None
                };
            let cam_shader: Option<RaylibShaderHandler> =
                if let Some(shader) = self.shaders.get("camera") {
                    Some(RaylibShaderHandler {
                        shader: shader.clone(),
                    })
                } else {
                    None
                };
            let raylib_image_handler = RaylibImageHandler {
                image: Rc::new(RefCell::new(RaylibImage {
                    image,
                    texture: None,
                })),
                tr_or_cam_shader: Rc::new(RefCell::new(tr_or_cam_shader)),
                cam_shader: Rc::new(RefCell::new(cam_shader)),
                camera: self.camera.clone(),
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

    fn load_shader(
        &mut self,
        name: String,
        vs: &Path,
        fs: &Path,
    ) -> Result<Box<dyn ShaderInterface>, String> {
        let raylib_shader = RaylibShaderHandler::load_shader(vs, fs)?;
        self.shaders.insert(name, raylib_shader.shader.clone());
        Ok(Box::new(raylib_shader))
    }

    fn get_camera(&mut self) -> Result<Box<dyn super::CameraInterface>, String> {
        Ok(Box::new(self.camera.borrow().clone()))
    }

    fn get_default_camera(&mut self) -> Result<Box<dyn super::CameraInterface>, String> {
        Ok(Box::new(Camera::default()))
    }

    fn set_camera(&mut self, camera: &Box<dyn super::CameraInterface>) -> Result<(), String> {
        self.camera.borrow_mut().pos = camera.as_ref().get_view_xy()?.into();
        Ok(())
    }

    fn set_camera_xy(&mut self, x: f32, y: f32) -> Result<(), String> {
        self.camera.borrow_mut().pos = Vector { x, y };
        Ok(())
    }

    fn xy_to_world(&self, x: f32, y: f32) -> Result<Vector, String> {
        Ok(Vector {
            x: x + self.camera.borrow().pos.x,
            y: y + self.camera.borrow().pos.y,
        })
    }

    fn vec_to_world(&self, vec: Vector) -> Result<Vector, String> {
        Ok(Vector {
            x: vec.x + self.camera.borrow().pos.x,
            y: vec.y + self.camera.borrow().pos.y,
        })
    }
}

impl RaylibGame {
    fn load_transform_origin_shader(&mut self) -> Result<Box<dyn ShaderInterface>, String> {
        self.load_shader(
            String::from("transform_origin"),
            &PathBuf::from_str("static/transform.vs")
                .map_err(|_| String::from("Failed to convert \"static/transform.vs\" to path!"))?,
            &PathBuf::from_str("static/simple.fs")
                .map_err(|_| String::from("Failed to convert \"static/simple.fs\" to path!"))?,
        )
    }

    fn load_camera_shader(&mut self) -> Result<Box<dyn ShaderInterface>, String> {
        self.load_shader(
            String::from("camera"),
            &PathBuf::from_str("static/camera.vs")
                .map_err(|_| String::from("Failed to convert \"static/camera.vs\" to path!"))?,
            &PathBuf::from_str("static/simple.fs")
                .map_err(|_| String::from("Failed to convert \"static/camera.vs\" to path!"))?,
        )
    }
}

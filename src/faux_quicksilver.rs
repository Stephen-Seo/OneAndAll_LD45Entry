use std::collections::HashMap;
use std::ops::{Add, AddAssign, Mul, Sub};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::agnostic_interface::{
    FontInterface, GameInterface, ImageInterface, MusicInterface, SoundInterface,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };

    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn pos_add_vec(&mut self, v: Vector) {
        self.x += v.x;
        self.y += v.y;
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Self { x, y, r }
    }

    pub fn pos_add_vec(&mut self, v: Vector) {
        self.x += v.x;
        self.y += v.y;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub mat: [f32; 9],
    translate: Vector,
    rotate: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            mat: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            translate: Vector { x: 0.0, y: 0.0 },
            rotate: 0.0,
        }
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: rhs.x * self.mat[0] + rhs.y * self.mat[1] + self.mat[2],
            y: rhs.x * self.mat[3] + rhs.y * self.mat[4] + self.mat[5],
        }
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Self::Output {
        Self {
            mat: [
                self.mat[0] * rhs.mat[0] + self.mat[1] * rhs.mat[3] + self.mat[2] * rhs.mat[6],
                self.mat[0] * rhs.mat[1] + self.mat[1] * rhs.mat[4] + self.mat[2] * rhs.mat[7],
                self.mat[0] * rhs.mat[2] + self.mat[1] * rhs.mat[5] + self.mat[2] * rhs.mat[8],
                self.mat[3] * rhs.mat[0] + self.mat[4] * rhs.mat[3] + self.mat[5] * rhs.mat[6],
                self.mat[3] * rhs.mat[1] + self.mat[4] * rhs.mat[4] + self.mat[5] * rhs.mat[7],
                self.mat[3] * rhs.mat[2] + self.mat[4] * rhs.mat[5] + self.mat[5] * rhs.mat[8],
                self.mat[6] * rhs.mat[0] + self.mat[7] * rhs.mat[3] + self.mat[8] * rhs.mat[6],
                self.mat[6] * rhs.mat[1] + self.mat[7] * rhs.mat[4] + self.mat[8] * rhs.mat[7],
                self.mat[6] * rhs.mat[2] + self.mat[7] * rhs.mat[5] + self.mat[8] * rhs.mat[8],
            ],
            translate: self.translate + rhs.translate,
            rotate: self.rotate + rhs.rotate,
        }
    }
}

impl Transform {
    pub const IDENTITY: Self = Self::default();

    pub fn rotate(rot: f32) -> Self {
        Self {
            mat: [
                rot.cos(),
                -rot.sin(),
                0.0,
                rot.sin(),
                rot.cos(),
                0.0,
                0.0,
                0.0,
                1.0,
            ],
            rotate: rot,
            ..Default::default()
        }
    }

    pub fn translate(x: f32, y: f32) -> Self {
        Self {
            mat: [1.0, 0.0, x, 0.0, 1.0, y, 0.0, 0.0, 1.0],
            translate: Vector { x, y },
            ..Default::default()
        }
    }

    pub fn get_translate(&self) -> Vector {
        self.translate
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotate
    }
}

pub struct Window {
    gi: Box<dyn GameInterface>,
    images: HashMap<String, Box<dyn ImageInterface>>,
    fonts: HashMap<String, Box<dyn FontInterface>>,
    sounds: HashMap<String, Box<dyn SoundInterface>>,
    music: HashMap<String, Box<dyn MusicInterface>>,
}

impl Window {
    pub fn new(gi: Box<dyn GameInterface>) -> Self {
        Self {
            gi,
            images: HashMap::new(),
            fonts: HashMap::new(),
            sounds: HashMap::new(),
            music: HashMap::new(),
        }
    }

    pub fn get_gi(&self) -> &dyn GameInterface {
        self.gi.as_ref()
    }

    pub fn get_gi_mut(&mut self) -> &mut dyn GameInterface {
        self.gi.as_mut()
    }

    pub fn load_image(&mut self, path: &Path, name: String) -> Result<(), String> {
        self.images.insert(name, self.gi.load_image(path)?);

        Ok(())
    }

    pub fn load_font(&mut self, path: &Path, name: String) -> Result<(), String> {
        self.fonts.insert(name, self.gi.load_font(path)?);

        Ok(())
    }

    pub fn load_sound(&mut self, path: &Path, name: String) -> Result<(), String> {
        self.sounds.insert(name, self.gi.load_sound(path)?);

        Ok(())
    }

    pub fn load_music(&mut self, path: &Path, name: String) -> Result<(), String> {
        self.music.insert(name, self.gi.load_music(path)?);

        Ok(())
    }

    pub fn get_image(&self, name: &str) -> Result<&dyn ImageInterface, String> {
        Ok(self
            .images
            .get(name)
            .ok_or_else(|| format!("Image \"{name}\" not found"))?
            .as_ref())
    }

    pub fn get_image_mut(&self, name: &str) -> Result<&mut dyn ImageInterface, String> {
        Ok(self
            .images
            .get_mut(name)
            .ok_or_else(|| format!("Image \"{name}\" not found"))?
            .as_mut())
    }

    pub fn get_font(&self, name: &str) -> Result<&dyn FontInterface, String> {
        Ok(self
            .fonts
            .get(name)
            .ok_or_else(|| format!("Font \"{name}\" not found"))?
            .as_ref())
    }

    pub fn get_font_mut(&self, name: &str) -> Result<&mut dyn FontInterface, String> {
        Ok(self
            .fonts
            .get_mut(name)
            .ok_or_else(|| format!("Font \"{name}\" not found"))?
            .as_mut())
    }

    pub fn get_sound(&self, name: &str) -> Result<&dyn SoundInterface, String> {
        Ok(self
            .sounds
            .get(name)
            .ok_or_else(|| format!("Sound \"{name}\" not found"))?
            .as_ref())
    }

    pub fn get_sound_mut(&self, name: &str) -> Result<&mut dyn SoundInterface, String> {
        Ok(self
            .sounds
            .get_mut(name)
            .ok_or_else(|| format!("Sound \"{name}\" not found"))?
            .as_mut())
    }

    pub fn get_music(&self, name: &str) -> Result<&dyn MusicInterface, String> {
        Ok(self
            .music
            .get(name)
            .ok_or_else(|| format!("Music \"{name}\" not found"))?
            .as_ref())
    }

    pub fn get_music_mut(&self, name: &str) -> Result<&mut dyn MusicInterface, String> {
        Ok(self
            .music
            .get_mut(name)
            .ok_or_else(|| format!("Music \"{name}\" not found"))?
            .as_mut())
    }
}

pub struct Key {}

pub struct Event {}

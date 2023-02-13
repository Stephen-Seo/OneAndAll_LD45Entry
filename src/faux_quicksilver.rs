use std::ops::{Mul, Add, AddAssign, Sub};

use serde::{Deserialize, Serialize};

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
        Self {
            r,
            g,
            b,
            a,
        }
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
        Self {
            x,
            y,
            w,
            h,
        }
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
        Self {
            x,
            y,
            r,
        }
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
        Self {
            x,
            y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub mat: [f32; 9],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            mat: [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0,
            ],
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
            ]
        }
    }
}

impl Transform {
    pub const IDENTITY: Self = Self::default();

    pub fn rotate(rot: f32) -> Self {
        Self {
            mat: [
                rot.cos(),  rot.sin(), 0.0,
                -rot.sin(), rot.cos(), 0.0,
                0.0,        0.0,       1.0,
            ]
        }
    }

    pub fn translate(x: f32, y: f32) -> Self {
        Self {
            mat: [
                1.0, 0.0, x,
                0.0, 1.0, y,
                0.0, 0.0, 1.0,
            ]
        }
    }
}

pub struct View {
}

pub struct Window {
}

pub struct Key {
}

pub struct Event {
}

pub struct Sound {
}

pub struct Font {
}

pub struct FontStyle {
}

pub struct Image {
    image_w: usize,
    image_h: usize,
}

impl Image {
    pub fn area_rect(&self) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            w: self.image_w as f32,
            h: self.image_h as f32,
        }
    }
}

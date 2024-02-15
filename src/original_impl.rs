use std::{fs::File, io::Result as IOResult, path::PathBuf, str::FromStr};

use crate::agnostic_interface::CameraInterface;
use crate::faux_quicksilver::{Circle, Color, Rectangle, Transform, Vector, Window};
use rand::prelude::*;

const WIDTH_F: f32 = 800.0;
const HEIGHT_F: f32 = 600.0;
const MUSIC2_LENGTH: f32 = 2.0 * 60.0 * 1000.0;
const TEXT_RATE: f32 = 0.1;
const PP_GEN_RATE: f32 = 0.075;
const PARTICLE_RAND_VEL_RANGE: f32 = 80.0;
const PARTICLE_RAND_VEL_DIST: f32 = 0.2828427; // dist where x and y = 0.2
const PARTICLE_RAND_ROT_RANGE: f32 = 5.0;
const JOINING_OPACITY_RATE: f32 = 0.13;
const JOINING_FAR_DIST: f32 = 700.0;
const JOINING_NEAR_DIST: f32 = 150.0;
const DOUBLE_CLICK_TIME: f32 = 0.350;
const SL_NOTIF_TIME: f32 = 7.0;
const MAX_MOONS: usize = 5;
const SAVE_FILENAME: &'static str = "LudumDare45_OneAndAll_SaveFile.bin";

fn interp_sq_inv(x: f32) -> f32 {
    if x < 0.0 {
        return 0.0;
    } else if x > 1.0 {
        return 1.0;
    }
    let y = x - 1.0;
    -y * y + 1.0
}

fn interp_sq(x: f32) -> f32 {
    if x < 0.0 {
        return 0.0;
    } else if x > 1.0 {
        return 1.0;
    }
    x * x
}

enum MenuItemType {
    Button {
        text: &'static str,
        text_c: Color,
        h_c: Color,
        c: Color,
    },
    AppearingText {
        text: &'static str,
        text_idx: usize,
        text_size: f32,
        text_c: Color,
        timer: f32,
    },
    InstantText {
        text: &'static str,
        text_size: f32,
        text_color: Color,
    },
    Pause {
        timer: f32,
        length: f32,
    },
}

struct MenuItem {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    item_type: MenuItemType,
    is_hover: bool,
    is_focus: bool,
    is_loaded: bool,
}

impl MenuItem {
    fn is_inside(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}

struct Menu {
    items: Vec<MenuItem>,
}

impl Menu {
    fn button(
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        s: &'static str,
        t_color: Color,
        box_color: Color,
        boxh_color: Color,
        first: bool,
    ) -> MenuItem {
        MenuItem {
            x,
            y,
            w,
            h,
            item_type: MenuItemType::Button {
                text: s,
                text_c: t_color,
                h_c: boxh_color,
                c: box_color,
            },
            is_hover: false,
            is_focus: false,
            is_loaded: !first,
        }
    }

    fn start() -> Menu {
        let item = MenuItem {
            x: WIDTH_F / 2.0 - 120.0,
            y: 150.0,
            w: 240.0,
            h: 150.0,
            item_type: MenuItemType::Button {
                text: "Start the Game",
                text_c: Color::WHITE,
                h_c: Color::from_rgba(0x66, 0xFF, 0xFF, 255),
                c: Color::from_rgba(0x33, 0xDD, 0xDD, 255),
            },
            is_hover: false,
            is_focus: false,
            is_loaded: false,
        };

        Menu {
            items: vec![
                item,
                Menu::instant_text(
                    70.0,
                    50.0,
                    45.0,
                    true,
                    "One And All - A Ludum Dare 45 Entry",
                ),
                Menu::instant_text(
                    25.0,
                    HEIGHT_F - 100.0,
                    27.0,
                    true,
                    "Made with Raylib which is licensed with the zlib license",
                ),
                Menu::instant_text(
                    25.0,
                    HEIGHT_F - 50.0,
                    27.0,
                    true,
                    "Uses Clear-Sans which is licensed with Apache License Version 2.0",
                ),
            ],
        }
    }

    fn instant_text(x: f32, y: f32, text_size: f32, first: bool, s: &'static str) -> MenuItem {
        MenuItem {
            x,
            y,
            w: 0.0,
            h: 0.0,
            item_type: MenuItemType::InstantText {
                text: s,
                text_size,
                text_color: Color::WHITE,
            },
            is_hover: false,
            is_focus: false,
            is_loaded: !first,
        }
    }

    fn text(x: f32, y: f32, text_size: f32, first: bool, s: &'static str) -> MenuItem {
        MenuItem {
            x,
            y,
            w: 0.0,
            h: 0.0,
            item_type: MenuItemType::AppearingText {
                text: s,
                text_size,
                text_c: Color::WHITE,
                timer: 0.0,
                text_idx: 0,
            },
            is_hover: false,
            is_focus: false,
            is_loaded: !first,
        }
    }

    fn pause(length: f32, first: bool) -> MenuItem {
        MenuItem {
            x: 0.0,
            y: 0.0,
            w: 0.0,
            h: 0.0,
            item_type: MenuItemType::Pause { timer: 0.0, length },
            is_hover: false,
            is_focus: false,
            is_loaded: !first,
        }
    }

    fn s_01() -> Menu {
        Menu {
            items: vec![
                Menu::pause(0.500, true),
                Menu::text(50.0, HEIGHT_F - 140.0, 40.0, false, "This is how it is."),
                Menu::pause(0.500, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 100.0,
                    40.0,
                    false,
                    "Nothing is, and everything is nothing.",
                ),
                Menu::pause(0.500, false),
                Menu::text(50.0, HEIGHT_F - 60.0, 40.0, false, "...until you appeared."),
                Menu::pause(0.100, false),
                Menu::text(
                    570.0,
                    HEIGHT_F - 50.0,
                    30.0,
                    false,
                    "(Click to continue...)",
                ),
            ],
        }
    }

    fn s_02() -> Menu {
        Menu {
            items: vec![
                Menu::text(
                    50.0,
                    HEIGHT_F - 150.0,
                    40.0,
                    true,
                    "Just by being, you brought light into existence.",
                ),
                Menu::pause(0.500, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 110.0,
                    40.0,
                    false,
                    "What brings you here? What drives you?",
                ),
                Menu::pause(0.500, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 70.0,
                    40.0,
                    false,
                    "Please tell me, what fuels you?",
                ),
                Menu::button(
                    100.0,
                    30.0,
                    200.0,
                    85.0,
                    "Hope",
                    Color::WHITE,
                    Color::BLACK,
                    Color::from_rgba(0x33, 0x33, 0x33, 255),
                    false,
                ),
                Menu::button(
                    500.0,
                    30.0,
                    200.0,
                    85.0,
                    "Miracles",
                    Color::WHITE,
                    Color::BLACK,
                    Color::from_rgba(0x33, 0x33, 0x33, 255),
                    false,
                ),
                Menu::button(
                    100.0,
                    150.0,
                    200.0,
                    85.0,
                    "Kindness",
                    Color::WHITE,
                    Color::BLACK,
                    Color::from_rgba(0x33, 0x33, 0x33, 255),
                    false,
                ),
                Menu::button(
                    500.0,
                    150.0,
                    200.0,
                    85.0,
                    "Determination",
                    Color::WHITE,
                    Color::BLACK,
                    Color::from_rgba(0x33, 0x33, 0x33, 255),
                    false,
                ),
            ],
        }
    }

    // choose hope
    fn s_03() -> Menu {
        Menu {
            items: vec![
                Menu::text(
                    50.0,
                    HEIGHT_F - 170.0,
                    40.0,
                    true,
                    "Hope... hope that your actions will inspire others..",
                ),
                Menu::pause(0.500, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 130.0,
                    40.0,
                    false,
                    "Hope that a brighter future will come tomorrow...",
                ),
                Menu::pause(0.500, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 90.0,
                    40.0,
                    false,
                    ".. With your appearance, perhaps it shall...",
                ),
            ],
        }
    }

    // choose miracles
    fn s_04() -> Menu {
        Menu {
            items: vec![
                Menu::text(
                    30.0,
                    HEIGHT_F - 170.0,
                    40.0,
                    true,
                    "Miracles huh?.. I see, then your appearance is special.",
                ),
                Menu::pause(0.500, false),
                Menu::text(
                    30.0,
                    HEIGHT_F - 130.0,
                    40.0,
                    false,
                    "With your appearance, things may change for the better..",
                ),
                Menu::pause(0.500, false),
                Menu::text(
                    30.0,
                    HEIGHT_F - 90.0,
                    40.0,
                    false,
                    "Now I am certain that this meeting was not by chance.",
                ),
            ],
        }
    }

    // choose kindness
    fn s_05() -> Menu {
        Menu {
            items: vec![
                Menu::text(
                    50.0,
                    HEIGHT_F - 170.0,
                    40.0,
                    true,
                    "Kindness?.. I am in your debt.",
                ),
                Menu::pause(0.250, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 130.0,
                    40.0,
                    false,
                    "It has been a long time since I have encountered",
                ),
                Menu::text(50.0, HEIGHT_F - 90.0, 40.0, false, "another being..."),
                Menu::pause(0.500, false),
                Menu::text(270.0, HEIGHT_F - 90.0, 40.0, false, "... Thank you..."),
            ],
        }
    }

    // choose determination
    fn s_06() -> Menu {
        Menu {
            items: vec![
                Menu::text(
                    50.0,
                    HEIGHT_F - 170.0,
                    40.0,
                    true,
                    "Determination.. I see...",
                ),
                Menu::pause(0.500, false),
                Menu::text(
                    400.0,
                    HEIGHT_F - 170.0,
                    40.0,
                    false,
                    "I do not doubt it, for it",
                ),
                Menu::text(
                    50.0,
                    HEIGHT_F - 130.0,
                    40.0,
                    false,
                    "must have been difficult to come here..",
                ),
                Menu::pause(0.500, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 90.0,
                    40.0,
                    false,
                    "Your resolve is evident by your mere presence..",
                ),
            ],
        }
    }

    fn s_07() -> Menu {
        Menu {
            items: vec![
                Menu::text(
                    50.0,
                    HEIGHT_F - 130.0,
                    40.0,
                    true,
                    "Now that you are here, it must mean a new era of",
                ),
                Menu::text(
                    50.0,
                    HEIGHT_F - 90.0,
                    40.0,
                    false,
                    "creation for all that will be.",
                ),
                Menu::pause(0.200, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 50.0,
                    40.0,
                    false,
                    "Try double-clicking the void to create something...",
                ),
            ],
        }
    }

    fn s_08() -> Menu {
        Menu {
            items: vec![Menu::instant_text(
                50.0,
                HEIGHT_F - 90.0,
                35.0,
                true,
                "(Try double-clicking now...)",
            )],
        }
    }

    fn s_09() -> Menu {
        Menu {
            items: vec![
                Menu::pause(0.400, true),
                Menu::text(
                    50.0,
                    HEIGHT_F - 140.0,
                    40.0,
                    false,
                    "A new planet... It has most certainly been a while.",
                ),
                Menu::pause(0.500, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 100.0,
                    40.0,
                    false,
                    "Please, go out and create the new universe, and again..",
                ),
                Menu::pause(0.300, false),
                Menu::text(50.0, HEIGHT_F - 60.0, 40.0, false, "Thank you."),
            ],
        }
    }

    fn s_10() -> Menu {
        Menu {
            items: vec![
                Menu::instant_text(
                    20.0,
                    HEIGHT_F - 40.0,
                    20.0,
                    true,
                    "Single click to move, Double-click to create something",
                ),
                Menu::instant_text(
                    20.0,
                    HEIGHT_F - 20.0,
                    20.0,
                    true,
                    "S - save; L - load (can load from the start); R - reset",
                ),
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Particle {
    rect: Rectangle,
    circle: Circle,
    is_rect: bool,
    velx: f32,
    vely: f32,
    velr: f32,
    r: f32,
    lifetime: f32,
    life_timer: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            rect: Rectangle::new(0.0, 0.0, 1.0, 1.0),
            circle: Circle::new(0.0, 0.0, 1.0),
            is_rect: true,
            velx: 0.0,
            vely: 0.0,
            velr: 0.0,
            r: 1.0,
            lifetime: 1.0,
            life_timer: 0.0,
        }
    }
}

impl Particle {
    pub fn deserialize(data: &[u8], offset: usize) -> Result<(Particle, usize), ()> {
        let mut idx: usize = 0;
        let mut particle = Particle::default();

        let (rect, rect_size) = Rectangle::deserialize(data, offset + idx)?;
        particle.rect = rect;
        idx += rect_size;

        let (circle, circle_size) = Circle::deserialize(data, offset + idx)?;
        particle.circle = circle;
        idx += circle_size;

        if data.len() < offset + idx + 1 {
            return Err(());
        }
        particle.is_rect = data[offset + idx] != 0;
        idx += 1;

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        particle.velx = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        particle.vely = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        particle.velr = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        particle.r = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        particle.lifetime = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        particle.life_timer = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        Ok((particle, idx))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.append(&mut self.rect.serialize());
        bytes.append(&mut self.circle.serialize());
        bytes.push(if self.is_rect { 1 } else { 0 });
        bytes.append(&mut self.velx.to_be_bytes().into());
        bytes.append(&mut self.vely.to_be_bytes().into());
        bytes.append(&mut self.velr.to_be_bytes().into());
        bytes.append(&mut self.r.to_be_bytes().into());
        bytes.append(&mut self.lifetime.to_be_bytes().into());
        bytes.append(&mut self.life_timer.to_be_bytes().into());

        bytes
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ParticleSystem {
    particles: Vec<Particle>,
    spawn_timer: f32,
    spawn_time: f32,
    lifetime: f32,
    host_rect: Rectangle,
    host_circle: Circle,
    is_rect: bool,
    direction: Vector,
    color: Color,
    opacity: f32,
    vel_multiplier: f32,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            particles: Vec::new(),
            spawn_timer: 0.0,
            spawn_time: 1.0,
            lifetime: 1.0,
            host_rect: Rectangle::default(),
            host_circle: Circle::default(),
            is_rect: true,
            direction: Vector::default(),
            color: Color::default(),
            opacity: 1.0,
            vel_multiplier: 1.0,
        }
    }
}

impl ParticleSystem {
    fn new(
        spawn_time: f32,
        lifetime: f32,
        host_rect: Rectangle,
        host_circle: Circle,
        is_rect: bool,
        direction: Vector,
        color: Color,
        opacity: f32,
        vel_multiplier: f32,
    ) -> Self {
        Self {
            particles: Vec::new(),
            spawn_timer: 0.0,
            spawn_time,
            lifetime,
            host_rect,
            host_circle,
            is_rect,
            direction,
            color,
            opacity,
            vel_multiplier,
        }
    }

    fn update(&mut self, dt: f32) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].life_timer += dt;
            if self.particles[i].life_timer > self.particles[i].lifetime {
                self.particles.swap_remove(i);
            } else if self.is_rect {
                self.particles[i].rect.x += self.particles[i].velx * dt;
                self.particles[i].rect.y += self.particles[i].vely * dt;
                self.particles[i].r += self.particles[i].velr * dt;
            } else {
                self.particles[i].circle.x += self.particles[i].velx * dt;
                self.particles[i].circle.y += self.particles[i].vely * dt;
            }
        }

        self.spawn_timer += dt;
        if self.spawn_timer > self.spawn_time {
            self.spawn_timer -= self.spawn_time;
            self.particles.push(Particle {
                rect: self.host_rect,
                circle: self.host_circle,
                is_rect: self.is_rect,
                velx: (rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_VEL_RANGE..PARTICLE_RAND_VEL_RANGE)
                    + self.direction.x)
                    * self.vel_multiplier,
                vely: (rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_VEL_RANGE..PARTICLE_RAND_VEL_RANGE)
                    + self.direction.y)
                    * self.vel_multiplier,
                // velx: self.direction.x,
                // vely: self.direction.y,
                velr: rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_ROT_RANGE..PARTICLE_RAND_ROT_RANGE)
                    * self.vel_multiplier,
                r: rand::thread_rng().gen_range(0.0..90.0),
                lifetime: self.lifetime,
                life_timer: 0.0,
            });
        }
    }

    fn draw(&mut self, window: &mut Window, transform: Transform) {
        if self.opacity == 0.0 {
            return;
        }
        for particle in &mut self.particles {
            self.color.a =
                ((1.0 - particle.life_timer / particle.lifetime) * self.opacity * 255.0) as u8;
            if particle.is_rect {
                let pre_transform =
                    Transform::translate(particle.rect.w / 2.0, particle.rect.h / 2.0)
                        * Transform::rotate(particle.r);
                window
                    .get_gi_mut()
                    .draw_rect_transform(
                        particle.rect,
                        self.color,
                        transform * pre_transform,
                        Vector {
                            x: particle.rect.x + particle.rect.w / 2.0,
                            y: particle.rect.y + particle.rect.h / 2.0,
                        },
                    )
                    .ok();
            } else {
                window
                    .get_gi_mut()
                    .draw_circle_transform(
                        particle.circle,
                        self.color,
                        transform,
                        Vector {
                            x: particle.circle.x,
                            y: particle.circle.y,
                        },
                    )
                    .ok();
            }
        }
    }

    fn force_spawn(&mut self, count: usize) {
        for i in 0..count {
            self.particles.push(Particle {
                rect: self.host_rect,
                circle: self.host_circle,
                is_rect: self.is_rect,
                velx: (rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_VEL_RANGE..PARTICLE_RAND_VEL_RANGE)
                    + self.direction.x)
                    * self.vel_multiplier,
                vely: (rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_VEL_RANGE..PARTICLE_RAND_VEL_RANGE)
                    + self.direction.y)
                    * self.vel_multiplier,
                // velx: self.direction.x,
                // vely: self.direction.y,
                velr: rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_ROT_RANGE..PARTICLE_RAND_ROT_RANGE)
                    * self.vel_multiplier,
                r: rand::thread_rng().gen_range(0.0..90.0),
                lifetime: self.lifetime,
                life_timer: 0.0,
            });
        }
    }

    pub fn deserialize(data: &[u8], offset: usize) -> Result<(Self, usize), ()> {
        let mut idx: usize = 0;
        let mut psystem = ParticleSystem::default();

        if data.len() < offset + idx + std::mem::size_of::<usize>() {
            return Err(());
        }
        let particles_len: usize = usize::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<usize>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<usize>();

        let mut particles: Vec<Particle> = Vec::new();
        for _ in 0..particles_len {
            let (particle, particle_size) = Particle::deserialize(data, offset + idx)?;
            particles.push(particle);
            idx += particle_size;
        }
        psystem.particles = particles;

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        psystem.spawn_timer = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        psystem.spawn_time = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        psystem.lifetime = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        let (rect, rect_size) = Rectangle::deserialize(data, offset + idx)?;
        psystem.host_rect = rect;
        idx += rect_size;

        let (circle, circle_size) = Circle::deserialize(data, offset + idx)?;
        psystem.host_circle = circle;
        idx += circle_size;

        if data.len() < offset + idx + 1 {
            return Err(());
        }
        psystem.is_rect = data[offset + idx] != 0;
        idx += 1;

        let (vector, vector_size) = Vector::deserialize(data, offset + idx)?;
        psystem.direction = vector;
        idx += vector_size;

        let (color, color_size) = Color::deserialize(data, offset + idx)?;
        psystem.color = color;
        idx += color_size;

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        psystem.opacity = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        psystem.vel_multiplier = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        Ok((psystem, idx))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let particle_count: usize = self.particles.len();
        bytes.append(&mut particle_count.to_be_bytes().into());

        for particle in self.particles.iter() {
            bytes.append(&mut particle.serialize());
        }

        bytes.extend(self.spawn_timer.to_be_bytes().into_iter());
        bytes.extend(self.spawn_time.to_be_bytes().into_iter());
        bytes.extend(self.lifetime.to_be_bytes().into_iter());

        bytes.append(&mut self.host_rect.serialize());

        bytes.append(&mut self.host_circle.serialize());

        bytes.push(if self.is_rect { 1 } else { 0 });

        bytes.append(&mut self.direction.serialize());

        bytes.append(&mut self.color.serialize());

        bytes.extend(self.opacity.to_be_bytes().into_iter());
        bytes.extend(self.vel_multiplier.to_be_bytes().into_iter());

        bytes
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RotatingParticleSystem {
    particle_system: ParticleSystem,
    r: f32,
    velr: f32,
    offset: f32,
}

impl Default for RotatingParticleSystem {
    fn default() -> Self {
        Self {
            particle_system: ParticleSystem::default(),
            r: 0.0,
            velr: 0.2,
            offset: 0.0,
        }
    }
}

impl RotatingParticleSystem {
    fn new(
        spawn_time: f32,
        lifetime: f32,
        host_rect: Rectangle,
        host_circle: Circle,
        is_rect: bool,
        direction: Vector,
        color: Color,
        opacity: f32,
        rotation: f32,
        velr: f32,
        offset: f32,
        vel_multiplier: f32,
    ) -> Self {
        RotatingParticleSystem {
            particle_system: ParticleSystem::new(
                spawn_time,
                lifetime,
                host_rect,
                host_circle,
                is_rect,
                direction,
                color,
                opacity,
                vel_multiplier,
            ),
            r: rotation,
            velr,
            offset,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.particle_system.is_rect {
            let saved_rect = self.particle_system.host_rect;
            self.particle_system
                .host_rect
                .pos_add_vec(Transform::rotate(self.r) * Vector::new(self.offset, 0.0));
            self.particle_system.update(dt);
            self.particle_system.host_rect = saved_rect;
        } else {
            let saved_cir = self.particle_system.host_circle;
            self.particle_system
                .host_circle
                .pos_add_vec(Transform::rotate(self.r) * Vector::new(self.offset, 0.0));
            self.particle_system.update(dt);
            self.particle_system.host_circle = saved_cir;
        }
        self.r += self.velr * dt * 10.0;
    }

    fn draw(&mut self, window: &mut Window, transform: Transform) {
        if self.particle_system.opacity == 0.0 {
            return;
        }
        self.particle_system.direction =
            Transform::rotate(self.r) * Vector::new(0.0, -PARTICLE_RAND_VEL_DIST);
        self.particle_system.draw(window, transform);
        if self.particle_system.is_rect {
            let mut moved_rect = self.particle_system.host_rect;
            moved_rect.pos_add_vec(Transform::rotate(self.r) * Vector::new(self.offset, 0.0));
            let mut solid_color = self.particle_system.color;
            solid_color.a = (self.particle_system.opacity * 255.0) as u8;
            window
                .get_gi_mut()
                .draw_rect_transform(
                    moved_rect,
                    solid_color,
                    transform * Transform::rotate(self.r * 1.3),
                    Vector {
                        x: moved_rect.x,
                        y: moved_rect.y,
                    },
                )
                .ok();
        } else {
            let mut moved_cir = self.particle_system.host_circle;
            moved_cir.pos_add_vec(Transform::rotate(self.r) * Vector::new(self.offset, 0.0));
            let mut solid_color = self.particle_system.color;
            solid_color.a = (self.particle_system.opacity * 255.0) as u8;
            window
                .get_gi_mut()
                .draw_circle_transform(
                    moved_cir,
                    solid_color,
                    transform,
                    Vector {
                        x: moved_cir.x,
                        y: moved_cir.y,
                    },
                )
                .ok();
        }
    }

    pub fn deserialize(data: &[u8], offset: usize) -> Result<(Self, usize), ()> {
        let mut idx: usize = 0;
        let mut rpsystem = RotatingParticleSystem::default();

        let (psystem, psystem_size) = ParticleSystem::deserialize(data, offset + idx)?;
        rpsystem.particle_system = psystem;
        idx += psystem_size;

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        rpsystem.r = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        rpsystem.velr = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        rpsystem.offset = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        Ok((rpsystem, idx))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.append(&mut self.particle_system.serialize());

        bytes.extend(self.r.to_be_bytes().into_iter());
        bytes.extend(self.velr.to_be_bytes().into_iter());
        bytes.extend(self.offset.to_be_bytes().into_iter());

        bytes
    }
}

struct ExplConvCircleParticle {
    circle: Circle,
    offset: f32,
    r: f32,
}

struct ExplConvParticleSystem {
    particles: Vec<ExplConvCircleParticle>,
    lifetime: f32,
    host_circle: Circle,
    color: Color,
    opacity: f32,
    life_timer: f32,
}

impl ExplConvParticleSystem {
    fn new(lifetime: f32, host_circle: Circle, color: Color, opacity: f32) -> Self {
        ExplConvParticleSystem {
            particles: Vec::new(),
            lifetime,
            host_circle,
            color,
            opacity,
            life_timer: 0.0,
        }
    }

    fn activate(&mut self, count: usize, offset: f32) {
        self.life_timer = 0.0;
        for i in 0..count {
            self.particles.push(ExplConvCircleParticle {
                circle: self.host_circle,
                offset,
                r: rand::thread_rng().gen_range(0.0..360.0),
            });
        }
    }

    // returns true if finished
    fn update(&mut self, dt: f32, planets: &mut Vec<Planet>) -> bool {
        self.life_timer += dt;
        if self.life_timer >= self.lifetime {
            if !self.particles.is_empty() {
                self.particles.clear();
                planets.push(Planet::new(self.host_circle, self.color));
                return true;
            }
            return false;
        }

        if self.life_timer < self.lifetime / 2.0 {
            let amount = interp_sq_inv(self.life_timer / self.lifetime * 2.0);
            for particle in &mut self.particles {
                let dir =
                    Transform::rotate(particle.r) * Vector::new(particle.offset * amount, 0.0);
                particle.circle.x = dir.x + self.host_circle.x;
                particle.circle.y = dir.y + self.host_circle.y;
            }
        } else {
            let amount = 1.0 - interp_sq((self.life_timer / self.lifetime - 0.5) * 2.0);
            for particle in &mut self.particles {
                let dir =
                    Transform::rotate(particle.r) * Vector::new(particle.offset * amount, 0.0);
                particle.circle.x = dir.x + self.host_circle.x;
                particle.circle.y = dir.y + self.host_circle.y;
            }
        }
        false
    }

    fn draw(&mut self, window: &mut Window, transform: Transform) {
        if self.opacity == 0.0 {
            return;
        }
        for particle in &mut self.particles {
            self.color.a =
                ((self.life_timer / self.lifetime / 2.0 + 0.5) * self.opacity * 255.0) as u8;
            window
                .get_gi_mut()
                .draw_circle_transform(
                    particle.circle,
                    self.color,
                    transform,
                    Vector {
                        x: particle.circle.x,
                        y: particle.circle.y,
                    },
                )
                .ok();
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Planet {
    circle: Circle,
    color: Color,
    particle_system: ParticleSystem,
    moons: Vec<RotatingParticleSystem>,
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            circle: Circle::default(),
            color: Color::default(),
            particle_system: ParticleSystem::default(),
            moons: Vec::new(),
        }
    }
}

impl Planet {
    fn new(circle: Circle, color: Color) -> Self {
        let mut smaller_circle = circle;
        smaller_circle.r /= 4.0;
        let mut planet = Planet {
            circle,
            color,
            particle_system: ParticleSystem::new(
                rand::thread_rng().gen_range(2.0..3.8),
                0.9,
                Rectangle::new(0.0, 0.0, 1.0, 1.0),
                circle,
                false,
                Vector::new(0.0, 0.0),
                color,
                1.0,
                0.3,
            ),
            moons: Vec::with_capacity(MAX_MOONS),
        };

        let r: f32 = rand::thread_rng().gen_range(0.0..360.0);
        let clockwise = rand::thread_rng().gen_bool(0.5);
        for i in 0..rand::thread_rng().gen_range(0..MAX_MOONS) {
            planet.moons.push(RotatingParticleSystem::new(
                rand::thread_rng().gen_range(1.0..2.6),
                0.6,
                Rectangle::new(0.0, 0.0, 1.0, 1.0),
                smaller_circle,
                false,
                Vector::new(0.0, 0.0),
                color,
                1.0,
                r,
                if clockwise {
                    rand::thread_rng().gen_range(0.05..0.15)
                } else {
                    rand::thread_rng().gen_range(-0.15..-0.05)
                },
                rand::thread_rng().gen_range(35.0..200.0),
                0.2,
            ));
        }

        planet
    }

    fn update(&mut self, dt: f32) {
        self.particle_system.host_circle.x = self.circle.x;
        self.particle_system.host_circle.y = self.circle.y;
        self.particle_system.update(dt);
        for moon in &mut self.moons {
            moon.particle_system.host_circle.x = self.circle.x;
            moon.particle_system.host_circle.y = self.circle.y;
            moon.update(dt);
        }
    }

    fn draw(&mut self, window: &mut Window, transform: Transform) {
        self.particle_system.draw(window, transform);
        window
            .get_gi_mut()
            .draw_circle_transform(
                self.circle,
                self.color,
                transform,
                Vector {
                    x: self.circle.x,
                    y: self.circle.y,
                },
            )
            .ok();
        for moon in &mut self.moons {
            moon.draw(window, transform);
        }
    }

    pub fn deserialize(data: &[u8], offset: usize) -> Result<(Planet, usize), ()> {
        let mut idx: usize = 0;
        let mut planet = Planet::default();

        let (circle, circle_size) = Circle::deserialize(data, offset + idx)?;
        planet.circle = circle;
        idx += circle_size;

        let (color, color_size) = Color::deserialize(data, offset + idx)?;
        planet.color = color;
        idx += color_size;

        let (psystem, psystem_size) = ParticleSystem::deserialize(data, offset + idx)?;
        planet.particle_system = psystem;
        idx += psystem_size;

        if data.len() < offset + idx + std::mem::size_of::<usize>() {
            return Err(());
        }
        let moons_size = usize::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<usize>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<usize>();
        for _ in 0..moons_size {
            let (rpsystem, rps_size) = RotatingParticleSystem::deserialize(data, offset + idx)?;
            planet.moons.push(rpsystem);
            idx += rps_size;
        }

        Ok((planet, idx))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.append(&mut self.circle.serialize());
        bytes.append(&mut self.color.serialize());
        bytes.append(&mut self.particle_system.serialize());

        let moons_size = self.moons.len();
        bytes.extend(moons_size.to_be_bytes().into_iter());
        for i in 0..moons_size {
            bytes.append(&mut self.moons[i].serialize());
        }

        bytes
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Star {
    color: Color,
    particle_system: ParticleSystem,
    velr: f32,
    r: f32,
}

impl Default for Star {
    fn default() -> Self {
        Self {
            color: Color::default(),
            particle_system: ParticleSystem::default(),
            velr: 0.0,
            r: 0.0,
        }
    }
}

impl Star {
    fn new(circle: Circle, color: Color, velr: f32, r: f32) -> Self {
        let mut star = Star {
            color,
            particle_system: ParticleSystem::new(
                rand::thread_rng().gen_range(0.08..0.2),
                0.85,
                Rectangle::new(0.0, 0.0, 1.0, 1.0),
                circle,
                false,
                Vector::new(0.0, 0.0),
                color,
                1.0,
                1.0,
            ),
            velr,
            r,
        };

        if star.color.r < (0.75 * 255.0) as u8 {
            star.color.r = (0.75 * 255.0) as u8;
        }
        if star.color.g < (0.75 * 255.0) as u8 {
            star.color.g = (0.75 * 255.0) as u8;
        }
        if star.color.b < (0.75 * 255.0) as u8 {
            star.color.b = (0.75 * 255.0) as u8;
        }
        star.particle_system
            .force_spawn(rand::thread_rng().gen_range(20..45));

        star
    }

    fn update(&mut self, dt: f32) {
        self.particle_system.update(dt);
        self.r += self.velr * dt;
    }

    fn draw(&mut self, image: &str, window: &mut Window, transform: Transform) {
        self.particle_system.draw(window, transform);
        let image = window.get_image_mut(image).expect("Should be loaded image");
        let mut image_rect = image.get_wh_rect();
        image_rect.x = self.particle_system.host_circle.x - image_rect.w / 2.0;
        image_rect.y = self.particle_system.host_circle.y - image_rect.h / 2.0;
        image
            .draw_transform(
                image_rect.x,
                image_rect.y,
                self.color,
                transform * Transform::rotate(self.r),
                Vector {
                    x: image_rect.x + image_rect.w / 2.0,
                    y: image_rect.y + image_rect.h / 2.0,
                },
            )
            .ok();
    }

    pub fn deserialize(data: &[u8], offset: usize) -> Result<(Star, usize), ()> {
        let mut idx: usize = 0;
        let mut star = Star::default();

        let (color, color_size) = Color::deserialize(data, offset + idx)?;
        star.color = color;
        idx += color_size;

        let (psystem, psystem_size) = ParticleSystem::deserialize(data, offset + idx)?;
        star.particle_system = psystem;
        idx += psystem_size;

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        star.velr = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        star.r = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        Ok((star, idx))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.append(&mut self.color.serialize());
        bytes.append(&mut self.particle_system.serialize());

        bytes.extend(self.velr.to_be_bytes().into_iter());
        bytes.extend(self.r.to_be_bytes().into_iter());

        bytes
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Fish {
    pos: Vector,
    r: f32,
    swim_time: f32,
    swim_timer: f32,
    swim_v: f32,
    anim_timer: f32,
    anim_time: f32,
    color: Color,
    body_rect: Rectangle,
    tail_rect: Rectangle,
}

enum FishState {
    Idle,
    Swim,
}

impl Default for Fish {
    fn default() -> Self {
        Self {
            pos: Vector::default(),
            r: 0.0,
            swim_time: 1.0,
            swim_timer: 0.0,
            swim_v: 0.0,
            anim_timer: 0.0,
            anim_time: 1.0,
            color: Color::default(),
            body_rect: Rectangle::default(),
            tail_rect: Rectangle::default(),
        }
    }
}

impl Fish {
    fn new(pos: Vector, r: f32, color: Color) -> Self {
        let anim_timer = rand::thread_rng().gen_range(0.8..1.0);
        Self {
            pos,
            r,
            swim_time: 0.8,
            swim_timer: 0.8,
            swim_v: 0.2,
            anim_timer,
            anim_time: anim_timer,
            color,
            body_rect: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 32.0,
                h: 16.0,
            },
            tail_rect: Rectangle {
                x: 32.0,
                y: 0.0,
                w: 16.0,
                h: 16.0,
            },
        }
    }

    fn set_next(&mut self, state: FishState) {
        match state {
            FishState::Idle => {
                self.swim_time = rand::thread_rng().gen_range(1.1..2.4);
                self.swim_timer = self.swim_time;
                self.anim_timer = 2.8;
                self.anim_time = 1.6;
                self.swim_v = 0.0;
            }
            FishState::Swim => {
                self.swim_time = rand::thread_rng().gen_range(1.4..2.3);
                self.swim_timer = self.swim_time;
                self.r = rand::thread_rng().gen_range(0.0..std::f32::consts::PI * 2.0);
                self.anim_timer = rand::thread_rng().gen_range(1.6..2.0);
                self.anim_time = self.anim_timer;
                self.swim_v = self.anim_timer / 8.0;
            }
        }
    }

    fn update(&mut self, dt: f32) {
        self.swim_time -= dt;
        if self.swim_time < 0.22 {
            self.swim_v /= 1.1;
        }
        if self.swim_time <= 0.0 {
            if rand::thread_rng().gen_bool(0.4) {
                self.set_next(FishState::Idle);
            } else {
                self.set_next(FishState::Swim);
            }
        }

        self.anim_timer -= dt;
        if self.anim_timer <= 0.0 {
            self.anim_timer = self.anim_time;
        }

        self.pos -= Transform::rotate(self.r) * Vector::new(self.swim_v, 0.0) * dt * 200.0;
    }

    fn draw(&mut self, i_fish: &str, window: &mut Window, transform: Transform) {
        let fish_img = window
            .get_image_mut(i_fish)
            .expect("\"fish\" Image should be loaded");
        let anim_angle = ((self.anim_timer / self.anim_time) * std::f32::consts::PI * 2.0).sin();
        let mut body_rect = self.body_rect;
        body_rect.x = self.pos.x - self.body_rect.w / 2.0;
        body_rect.y = self.pos.y - self.body_rect.h / 2.0;
        let body_tr = Transform::rotate(anim_angle + self.r);
        fish_img
            .draw_sub_transform(
                self.body_rect,
                body_rect,
                self.color,
                transform * body_tr,
                Vector {
                    x: self.pos.x,
                    y: self.pos.y,
                },
            )
            .ok();
        let mut tail_rect = self.tail_rect;
        tail_rect.x = self.pos.x + body_rect.w / 2.0;
        tail_rect.y = self.pos.y - body_rect.h / 2.0;
        let anim_angle = ((self.anim_timer / self.anim_time) * std::f32::consts::PI * 2.0
            - std::f32::consts::PI / 3.0)
            .sin();
        let tail_tr = body_tr
            * Transform::translate(-body_rect.w / 2.0, 0.0)
            * Transform::rotate(-anim_angle)
            * Transform::translate(body_rect.w / 2.0, 0.0);
        fish_img
            .draw_sub_transform(
                self.tail_rect,
                tail_rect,
                self.color,
                transform * tail_tr,
                Vector {
                    x: self.pos.x,
                    y: self.pos.y,
                },
            )
            .ok();
    }

    pub fn deserialize(data: &[u8], offset: usize) -> Result<(Fish, usize), ()> {
        let mut idx: usize = 0;
        let mut fish = Fish::default();

        let (pos, pos_size) = Vector::deserialize(data, offset + idx)?;
        fish.pos = pos;
        idx += pos_size;

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        fish.r = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        fish.swim_time = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        fish.swim_timer = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        fish.swim_v = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        fish.anim_timer = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        if data.len() < offset + idx + std::mem::size_of::<f32>() {
            return Err(());
        }
        fish.anim_time = f32::from_be_bytes(
            data[(offset + idx)..(offset + idx + std::mem::size_of::<f32>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<f32>();

        let (color, color_size) = Color::deserialize(data, offset + idx)?;
        fish.color = color;
        idx += color_size;

        let (body_rect, body_rect_size) = Rectangle::deserialize(data, offset + idx)?;
        fish.body_rect = body_rect;
        idx += body_rect_size;

        let (tail_rect, tail_rect_size) = Rectangle::deserialize(data, offset + idx)?;
        fish.tail_rect = tail_rect;
        idx += tail_rect_size;

        Ok((fish, idx))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.append(&mut self.pos.serialize());

        bytes.extend(self.r.to_be_bytes().into_iter());
        bytes.extend(self.swim_time.to_be_bytes().into_iter());
        bytes.extend(self.swim_timer.to_be_bytes().into_iter());
        bytes.extend(self.swim_v.to_be_bytes().into_iter());
        bytes.extend(self.anim_timer.to_be_bytes().into_iter());
        bytes.extend(self.anim_time.to_be_bytes().into_iter());

        bytes.append(&mut self.color.serialize());
        bytes.append(&mut self.body_rect.serialize());
        bytes.append(&mut self.tail_rect.serialize());

        bytes
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SaveData {
    planets: Vec<Planet>,
    stars: Vec<Star>,
    fishes: Vec<Fish>,
    player: Rectangle,
    joining_particles: RotatingParticleSystem,
}

const SAVE_DATA_IDENTIFIER: [u8; 4] = [0x53, 0x41, 0x56, 0x45];

impl Default for SaveData {
    fn default() -> Self {
        Self {
            planets: Vec::new(),
            stars: Vec::new(),
            fishes: Vec::new(),
            player: Rectangle::default(),
            joining_particles: RotatingParticleSystem::default(),
        }
    }
}

impl SaveData {
    pub fn deserialize(data: &[u8]) -> Result<(SaveData, usize), ()> {
        let mut idx: usize = 0;
        let mut save_data = SaveData::default();

        if data.len() < idx + SAVE_DATA_IDENTIFIER.len() {
            return Err(());
        }
        for i in 0..SAVE_DATA_IDENTIFIER.len() {
            if data[idx + i] != SAVE_DATA_IDENTIFIER[i] {
                return Err(());
            }
        }
        idx += SAVE_DATA_IDENTIFIER.len();

        if data.len() < idx + std::mem::size_of::<usize>() {
            return Err(());
        }
        let planets_size = usize::from_be_bytes(
            data[idx..(idx + std::mem::size_of::<usize>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<usize>();

        for _ in 0..planets_size {
            let (planet, planet_size) = Planet::deserialize(data, idx)?;
            save_data.planets.push(planet);
            idx += planet_size;
        }

        if data.len() < idx + std::mem::size_of::<usize>() {
            return Err(());
        }
        let stars_size = usize::from_be_bytes(
            data[idx..(idx + std::mem::size_of::<usize>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<usize>();

        for _ in 0..stars_size {
            let (star, star_size) = Star::deserialize(data, idx)?;
            save_data.stars.push(star);
            idx += star_size;
        }

        if data.len() < idx + std::mem::size_of::<usize>() {
            return Err(());
        }
        let fishes_size = usize::from_be_bytes(
            data[idx..(idx + std::mem::size_of::<usize>())]
                .try_into()
                .map_err(|_| ())?,
        );
        idx += std::mem::size_of::<usize>();

        for _ in 0..fishes_size {
            let (fish, fish_size) = Fish::deserialize(data, idx)?;
            save_data.fishes.push(fish);
            idx += fish_size;
        }

        let (p_rect, p_rect_size) = Rectangle::deserialize(data, idx)?;
        save_data.player = p_rect;
        idx += p_rect_size;

        let (jp, jp_size) = RotatingParticleSystem::deserialize(data, idx)?;
        save_data.joining_particles = jp;
        idx += jp_size;

        Ok((save_data, idx))
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(SAVE_DATA_IDENTIFIER.iter());

        bytes.extend(self.planets.len().to_be_bytes().into_iter());
        for planet in &self.planets {
            bytes.append(&mut planet.serialize());
        }

        bytes.extend(self.stars.len().to_be_bytes().into_iter());
        for star in &self.stars {
            bytes.append(&mut star.serialize());
        }

        bytes.extend(self.fishes.len().to_be_bytes().into_iter());
        for fish in &self.fishes {
            bytes.append(&mut fish.serialize());
        }

        bytes.append(&mut self.player.serialize());

        bytes.append(&mut self.joining_particles.serialize());

        bytes
    }
}

enum SaveLoadNotification {
    Save { text: Option<String>, timer: f32 },
    Load { text: Option<String>, timer: f32 },
}

pub struct GameState {
    s_boom: String,
    s_get: String,
    s_power_up: String,
    s_tap: String,
    s_speak_m: String,
    s_speak_f: String,
    font: String,
    music2: String,
    i_star: String,
    i_fish: String,
    music_on: bool,
    menu: Menu,
    state: u32,
    state_dirty: bool,
    selection_mode: bool,
    current_item: Option<usize>,
    current_finished: bool,
    player: Rectangle,
    player_r: f32,
    player_particles: ParticleSystem,
    joining_particles: RotatingParticleSystem,
    is_create_mode: bool,
    click_release_time: f32,
    dbl_click_timeout: Option<f32>,
    click_time: Option<f32>,
    click_pos: Vector,
    mouse_pos: Vector,
    expl_conv_p_systems: Vec<ExplConvParticleSystem>,
    planets: Vec<Planet>,
    stars: Vec<Star>,
    fishes: Vec<Fish>,
    camera: Box<dyn CameraInterface>,
    move_to: Vector,
    save_load_notification: Option<SaveLoadNotification>,
}

impl GameState {
    pub fn new(window: &mut Window) -> Result<Self, String> {
        let s_boom = String::from("boom.mp3");
        window.load_sound(
            &PathBuf::from_str("static/boom.mp3")
                .map_err(|_| String::from("Failed to load \"static/boom.mp3\""))?,
            s_boom.clone(),
        )?;
        let s_get = String::from("get.mp3");
        window.load_sound(
            &PathBuf::from_str("static/get.mp3")
                .map_err(|_| String::from("Failed to load \"static/get.mp3\""))?,
            s_get.clone(),
        )?;
        let s_power_up = String::from("power_up.mp3");
        //window.load_sound(
        //    &PathBuf::from_str("static/power_up.mp3")
        //        .map_err(|_| String::from("Failed to load \"static/power_up.mp3\""))?,
        //    s_power_up.clone(),
        //)?;
        let s_tap = String::from("tap.mp3");
        window.load_sound(
            &PathBuf::from_str("static/tap.mp3")
                .map_err(|_| String::from("Failed to load \"static/tap.mp3\""))?,
            s_tap.clone(),
        )?;
        let s_speak_m = String::from("speak_m.mp3");
        //window.load_sound(
        //    &PathBuf::from_str("static/speak_m.mp3")
        //        .map_err(|_| String::from("Failed to load \"static/speak_m.mp3\""))?,
        //    s_speak_m.clone(),
        //)?;
        let s_speak_f = String::from("speak_f.mp3");
        //window.load_sound(
        //    &PathBuf::from_str("static/speak_f.mp3")
        //        .map_err(|_| String::from("Failed to load \"static/speak_f.mp3\""))?,
        //    s_speak_f.clone(),
        //)?;

        let font = String::from("ClearSans-Regular.ttf");
        window.load_font(
            &PathBuf::from_str("static/ClearSans-Regular.ttf")
                .map_err(|_| String::from("Failed to load \"static/ClearSans-Regular.ttf\""))?,
            font.clone(),
        )?;

        let music2 = String::from("music2.mp3");
        window.load_music(
            &PathBuf::from_str("static/music2.mp3")
                .map_err(|_| String::from("Failed to load \"static/music2.mp3\""))?,
            music2.clone(),
        )?;

        let i_star = String::from("star.png");
        window.load_image(
            &PathBuf::from_str("static/star.png")
                .map_err(|_| String::from("Failed to load \"static/star.png\""))?,
            i_star.clone(),
        )?;

        let i_fish = String::from("fish.png");
        window.load_image(
            &PathBuf::from_str("static/fish.png")
                .map_err(|_| String::from("Failed to load \"static/fish.png\""))?,
            i_fish.clone(),
        )?;

        let mut camera = window.get_gi_mut().get_default_camera()?;
        camera.set_view_xy(0.0, 0.0)?;
        Ok(Self {
            s_boom,
            s_get,
            s_power_up,
            s_tap,
            s_speak_m,
            s_speak_f,
            font,
            music2,
            i_star,
            i_fish,
            music_on: false,
            menu: Menu::start(),
            state: 0,
            state_dirty: false,
            selection_mode: true,
            current_item: None,
            current_finished: true,
            player: Rectangle::new(400.0, 300.0, 32.0, 32.0),
            player_r: 0.0,
            player_particles: ParticleSystem::new(
                PP_GEN_RATE,
                1.0,
                Rectangle::new(400.0, 300.0, 32.0, 32.0),
                Circle::new(100.0, 100.0, 32.0),
                true,
                Vector::new(0.0, 0.0),
                Color::WHITE,
                0.0,
                1.0,
            ),
            joining_particles: RotatingParticleSystem::new(
                PP_GEN_RATE,
                1.0,
                Rectangle::new(400.0, 300.0, 16.0, 16.0),
                Circle::new(100.0, 100.0, 32.0),
                true,
                Vector::new(0.0, 0.0),
                Color::GREEN,
                0.0,
                0.0,
                0.1,
                JOINING_FAR_DIST,
                1.0,
            ),
            is_create_mode: false,
            click_release_time: 0.0,
            dbl_click_timeout: None,
            click_time: None,
            click_pos: Vector::new(0.0, 0.0),
            mouse_pos: Vector::new(0.0, 0.0),
            expl_conv_p_systems: Vec::new(),
            planets: Vec::new(),
            stars: Vec::new(),
            fishes: Vec::new(),
            camera,
            move_to: Vector::new(400.0, 300.0),
            save_load_notification: None,
        })
    }

    pub fn update(&mut self, window: &mut Window) -> Result<(), String> {
        let dt = window.get_gi().get_delta_time();

        // check mouse pos
        {
            self.mouse_pos = window.get_gi().get_mouse_xy_vec()?;
            //self.mouse_pos = window.get_gi().vec_to_world(self.mouse_pos)?;
            let mut hovered = false;
            for i in 0..self.menu.items.len() {
                if self.menu.items[i].is_inside(self.mouse_pos.x, self.mouse_pos.y) {
                    self.menu.items[i].is_hover = true;
                    self.current_item = Some(i);
                    hovered = true;
                } else {
                    self.menu.items[i].is_hover = false;
                }
            }
            if !hovered {
                self.current_item = None;
            }
        }

        // check mouse down
        if window.get_gi_mut().get_mouse_released()? {
            if self.dbl_click_timeout.is_none() {
                self.click_release_time = 0.0;
            }
        } else if window.get_gi_mut().get_mouse_pressed()?.is_some() {
            if self.current_finished {
                if self.is_create_mode {
                    let click_pos = window.get_gi().vec_to_world(self.mouse_pos)?;
                    if self.click_release_time < DOUBLE_CLICK_TIME {
                        self.click_release_time = DOUBLE_CLICK_TIME;
                        self.dbl_click_timeout = Some(0.0);
                        self.click_time = None;
                        if self.state == 8 {
                            let mut expl_conv_system = ExplConvParticleSystem::new(
                                1.5,
                                Circle::new(click_pos.x, click_pos.y, 20.0),
                                Color::from_rgba(0x99, 0xFF, 0x99, 255),
                                1.0,
                            );
                            expl_conv_system.activate(30, 200.0);
                            self.expl_conv_p_systems.push(expl_conv_system);
                            self.state = 9;
                            self.state_dirty = true;
                            window.get_sound_mut(&self.s_boom)?.play(0.8)?;
                        } else if self.state == 10 {
                            let mut rng = rand::thread_rng();
                            let rand_out = rng.gen_range(0.0..1.0);
                            if rand_out < 0.6 {
                                // spawn planet
                                let mut expl_conv_system = ExplConvParticleSystem::new(
                                    rng.gen_range(1.2..1.6),
                                    Circle::new(
                                        click_pos.x,
                                        click_pos.y,
                                        rng.gen_range(15.0..25.0),
                                    ),
                                    Color::from_rgba(
                                        rng.gen_range(0x44..0xFF),
                                        rng.gen_range(0x44..0xFF),
                                        rng.gen_range(0x44..0xFF),
                                        255,
                                    ),
                                    1.0,
                                );
                                expl_conv_system
                                    .activate(rng.gen_range(13..40), rng.gen_range(150.0..300.0));
                                self.expl_conv_p_systems.push(expl_conv_system);
                            } else if rand_out < 0.85 {
                                // spawn star
                                let rot_clockwise = rng.gen_bool(0.5);
                                self.stars.push(Star::new(
                                    Circle::new(click_pos.x, click_pos.y, rng.gen_range(3.0..7.0)),
                                    Color::from_rgba(
                                        rng.gen_range(0x58..0xFF),
                                        rng.gen_range(0x58..0xFF),
                                        rng.gen_range(0x58..0xFF),
                                        255,
                                    ),
                                    if rot_clockwise {
                                        rng.gen_range(0.1..0.3)
                                    } else {
                                        rng.gen_range(-0.3..-0.1)
                                    },
                                    rng.gen_range(0.0..90.0),
                                ));
                            } else {
                                // spawn fish
                                for i in 0..rng.gen_range(1..4) {
                                    self.fishes.push(Fish::new(
                                        click_pos,
                                        rng.gen_range(0.0..360.0),
                                        Color::from_rgba(
                                            rng.gen_range(0x44..0xFF),
                                            rng.gen_range(0x44..0xFF),
                                            rng.gen_range(0x44..0xFF),
                                            255,
                                        ),
                                    ));
                                }
                            }
                            window.get_sound_mut(&self.s_boom)?.play(0.8)?;
                        }
                    } else if self.state == 10 {
                        self.click_time = Some(0.0);
                        self.click_pos = click_pos;
                    }
                } else if self.selection_mode {
                    if let Some(idx) = self.current_item {
                        match self.state {
                            0 => {
                                self.state += 1;
                                self.state_dirty = true;
                            }
                            2 => {
                                if idx == 5 {
                                    // hope
                                    self.state = 3;
                                    self.state_dirty = true;
                                    self.joining_particles.particle_system.color =
                                        Color::from_rgba(0xAA, 0xCC, 0xFF, 255);
                                } else if idx == 6 {
                                    // miracles
                                    self.state = 4;
                                    self.state_dirty = true;
                                    self.joining_particles.particle_system.color =
                                        Color::from_rgba(0xFF, 0xFF, 0xAA, 255);
                                } else if idx == 7 {
                                    // kindness
                                    self.state = 5;
                                    self.state_dirty = true;
                                    self.joining_particles.particle_system.color =
                                        Color::from_rgba(0xBB, 0xFF, 0xBB, 255);
                                } else {
                                    // determination
                                    self.state = 6;
                                    self.state_dirty = true;
                                    self.joining_particles.particle_system.color =
                                        Color::from_rgba(0xFF, 0xAA, 0xAA, 255);
                                }
                                window.get_sound_mut(&self.s_get)?.play(0.7)?;
                            }
                            _ => {
                                self.state = 0;
                                self.state_dirty = true;
                            }
                        }
                    }
                } else {
                    match self.state {
                        0 | 1 => self.state += 1,
                        3 | 4 | 5 | 6 => self.state = 7,
                        7 => self.state = 8,
                        9 => self.state = 10,
                        _ => self.state = 0,
                    }
                    self.state_dirty = true;
                }
            } else {
                for mi in &mut self.menu.items {
                    match &mut mi.item_type {
                        MenuItemType::AppearingText {
                            text,
                            text_idx,
                            text_size,
                            text_c,
                            timer,
                        } => {
                            *text_idx = text.len();
                        }
                        MenuItemType::Button {
                            text,
                            text_c,
                            h_c,
                            c,
                        } => {
                            //let style = FontStyle::new(42.0, *text_c);
                        }
                        MenuItemType::Pause { timer, length } => (),
                        MenuItemType::InstantText {
                            text,
                            text_size,
                            text_color,
                        } => {}
                    }
                    mi.is_loaded = true;
                }
                self.current_finished = true;
            }
        }

        // check pressed keys
        if window.get_gi_mut().get_key_pressed('s')? {
            if self.state == 10 {
                let save_result = self.save().map_err(|e| e.to_string());
                if let Err(s) = save_result {
                    self.save_load_notification = Some(SaveLoadNotification::Save {
                        text: Some(format!("Failed to save! {}", s)),
                        timer: SL_NOTIF_TIME,
                    });
                }
            }
        } else if window.get_gi_mut().get_key_pressed('l')? {
            let load_result = self.load().map_err(|e| e.to_string());
            if let Err(s) = load_result {
                self.save_load_notification = Some(SaveLoadNotification::Load {
                    text: Some(format!("Failed to load! {}", s)),
                    timer: SL_NOTIF_TIME,
                });
            }
        } else if window.get_gi_mut().get_key_pressed('r')? && self.state == 10 {
            self.state = 0;
            self.state_dirty = true;
            window.get_music_mut(&self.music2)?.stop()?;
            self.music_on = false;
        }

        self.click_release_time += dt;
        if let Some(t) = &mut self.click_time {
            *t += dt;
            if *t > DOUBLE_CLICK_TIME {
                self.move_to = self.click_pos; // - Vector::new(WIDTH_F / 2.0, HEIGHT_F / 2.0);
            }
        }

        if let Some(t) = &mut self.dbl_click_timeout {
            *t += dt;
            if *t > 0.3 {
                self.dbl_click_timeout = None;
            }
        }

        self.player.x += (self.move_to.x - self.player.x) / 20.0;
        self.player.y += (self.move_to.y - self.player.y) / 20.0;
        self.player_particles.host_rect.x = self.player.x;
        self.player_particles.host_rect.y = self.player.y;
        self.joining_particles.particle_system.host_rect.x +=
            (self.player.x - self.joining_particles.particle_system.host_rect.x) / 30.0;
        self.joining_particles.particle_system.host_rect.y +=
            (self.player.y - self.joining_particles.particle_system.host_rect.y) / 30.0;
        let (cx, cy) = self.camera.get_view_xy()?;
        self.camera.set_view_xy(
            cx + (self.player.x - WIDTH_F / 2.0 - cx) / 40.0,
            cy + (self.player.y - HEIGHT_F / 2.0 - cy) / 40.0,
        )?;
        window.get_gi_mut().set_camera(self.camera.as_ref())?;

        self.player_r += dt / 10.0;

        if self.state_dirty {
            self.state_dirty = false;
            if self.state > 1 && !self.music_on {
                let music = window.get_music_mut(&self.music2)?;
                music.set_loop(true)?;
                music.play(0.5)?;
                self.music_on = true;
            }
            match self.state {
                1 => {
                    self.menu = Menu::s_01();
                    self.current_finished = false;
                    self.selection_mode = false;
                }
                2 => {
                    self.menu = Menu::s_02();
                    self.current_finished = false;
                    self.selection_mode = true;
                }
                3 => {
                    self.menu = Menu::s_03();
                    self.current_finished = false;
                    self.selection_mode = false;
                }
                4 => {
                    self.menu = Menu::s_04();
                    self.current_finished = false;
                    self.selection_mode = false;
                }
                5 => {
                    self.menu = Menu::s_05();
                    self.current_finished = false;
                    self.selection_mode = false;
                }
                6 => {
                    self.menu = Menu::s_06();
                    self.current_finished = false;
                    self.selection_mode = false;
                }
                7 => {
                    self.menu = Menu::s_07();
                    self.current_finished = false;
                    self.selection_mode = false;
                }
                8 => {
                    self.menu = Menu::s_08();
                    self.current_finished = true;
                    self.selection_mode = false;
                    self.is_create_mode = true;
                }
                9 => {
                    self.menu = Menu::s_09();
                    self.current_finished = false;
                    self.selection_mode = false;
                    self.is_create_mode = false;
                }
                10 => {
                    self.menu = Menu::s_10();
                    self.current_finished = false;
                    self.selection_mode = false;
                    self.is_create_mode = true;
                }
                _ => {
                    self.menu = Menu::start();
                    self.current_item = None;
                    self.selection_mode = true;
                    self.is_create_mode = false;
                    self.state = 0;
                    self.player_particles.opacity = 0.0;
                    self.joining_particles.particle_system.opacity = 0.0;
                    self.expl_conv_p_systems.clear();
                    self.planets.clear();
                    self.stars.clear();
                    self.fishes.clear();
                    self.player.x = WIDTH_F / 2.0;
                    self.player.y = HEIGHT_F / 2.0;
                    self.move_to = Vector::new(WIDTH_F / 2.0, HEIGHT_F / 2.0);
                    self.camera.set_view_xy(0.0, 0.0)?;
                    self.click_time = None;
                }
            }
        }

        if self.joining_particles.particle_system.opacity < 1.0 && self.state > 2 {
            self.joining_particles.particle_system.opacity += JOINING_OPACITY_RATE * dt;
            if self.joining_particles.particle_system.opacity > 1.0 {
                self.joining_particles.particle_system.opacity = 1.0;
            }
            self.joining_particles.offset =
                (1.0 - self.joining_particles.particle_system.opacity / 1.0) * JOINING_FAR_DIST
                    + self.joining_particles.particle_system.opacity / 1.0 * JOINING_NEAR_DIST;
        }

        if self.player_particles.opacity < 1.0 && self.state > 1 {
            self.player_particles.opacity += dt / 7.0;
            if self.player_particles.opacity > 1.0 {
                self.player_particles.opacity = 1.0;
            }
        }

        if self.music_on {
        } else if self.state == 10 {
            let music = window.get_music_mut(&self.music2)?;
            music.set_loop(true)?;
            music.play(0.5)?;
            self.music_on = true;
        }

        for i in 0..self.menu.items.len() {
            let mi: &mut MenuItem = &mut self.menu.items[i];
            if !mi.is_loaded {
                match &mut mi.item_type {
                    MenuItemType::Button {
                        text,
                        text_c,
                        h_c,
                        c,
                    } => {
                        //self.font.execute(|font| {
                        //    let style = FontStyle::new(42.0, *text_c);
                        //    *text_image = Some(font.render(text, &style)?);
                        //    Ok(())
                        //})?;
                        //if text_image.is_some() {
                        mi.is_loaded = true;
                        if i + 1 < self.menu.items.len() {
                            self.menu.items[i + 1].is_loaded = false;
                        } else {
                            self.current_finished = true;
                        }
                        //}
                    }
                    MenuItemType::AppearingText {
                        text,
                        text_idx,
                        text_size,
                        text_c,
                        timer,
                    } => {
                        *timer += dt;
                        if *timer > TEXT_RATE {
                            *timer -= TEXT_RATE;
                            *text_idx += 1;
                            window.get_sound_mut(&self.s_tap).unwrap().play(0.2)?;
                            if *text_idx >= text.len() {
                                mi.is_loaded = true;
                                if i + 1 < self.menu.items.len() {
                                    self.menu.items[i + 1].is_loaded = false;
                                } else {
                                    self.current_finished = true;
                                }
                                continue;
                            }
                            //self.font.execute(|font| {
                            //    let style = FontStyle::new(*text_size, *text_c);
                            //    *text_image = Some(font.render(current_text, &style)?);
                            //    Ok(())
                            //})?;
                        }
                    }
                    MenuItemType::Pause { timer, length } => {
                        *timer += dt;
                        if timer > length {
                            mi.is_loaded = true;
                            if i + 1 < self.menu.items.len() {
                                self.menu.items[i + 1].is_loaded = false;
                            } else {
                                self.current_finished = true;
                            }
                        }
                    }
                    MenuItemType::InstantText {
                        text,
                        text_size,
                        text_color,
                    } => {
                        //if text_image.is_none() {
                        //    self.font.execute(|f| {
                        //        let style = FontStyle::new(*text_size, *text_color);
                        //        *text_image = Some(f.render(text, &style)?);
                        //        Ok(())
                        //    })?;
                        //}
                        //if text_image.is_some() {
                        mi.is_loaded = true;
                        if i + 1 < self.menu.items.len() {
                            self.menu.items[i + 1].is_loaded = false;
                        } else {
                            self.current_finished = true;
                        }
                        //}
                    }
                }
            }
        }

        self.player_particles.host_rect = self.player;
        self.player_particles.update(dt);
        self.joining_particles.update(dt);

        for i in (0..self.expl_conv_p_systems.len()).rev() {
            if self.expl_conv_p_systems[i].update(dt, &mut self.planets) {
                self.expl_conv_p_systems.swap_remove(i);
            }
        }
        for planet in &mut self.planets {
            planet.update(dt);
        }
        for star in &mut self.stars {
            star.update(dt);
        }

        if let Some(sl) = &mut self.save_load_notification {
            match sl {
                SaveLoadNotification::Save { text, timer } => {
                    *timer -= dt;
                    println!("save, timer is {}", timer);
                    if *timer <= 0.0 {
                        self.save_load_notification = None;
                    } else if text.is_none() {
                        *text = Some(String::from("Saved the Game!"));
                    }
                }
                SaveLoadNotification::Load { text, timer } => {
                    *timer -= dt;
                    if *timer <= 0.0 {
                        self.save_load_notification = None;
                    } else if text.is_none() {
                        *text = Some(String::from("Loaded the Game!"));
                    }
                }
            }
        }

        for fish in &mut self.fishes {
            fish.update(dt);
        }

        Ok(())
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<(), String> {
        window.get_gi_mut().begin_drawing()?;
        window.get_gi_mut().clear_window(Color::BLACK)?;
        let mut rect = Rectangle::default();
        for mi in &mut self.menu.items {
            rect.x = mi.x;
            rect.y = mi.y;
            rect.w = mi.w;
            rect.h = mi.h;
            match &mut mi.item_type {
                MenuItemType::Button {
                    text,
                    text_c,
                    h_c,
                    c,
                } => {
                    if mi.is_hover {
                        window.get_gi_mut().draw_rect(rect, *h_c)?;
                    } else {
                        window.get_gi_mut().draw_rect(rect, *c)?;
                    }
                    window
                        .get_font_mut(&self.font)?
                        .draw(text, 20, rect.x, rect.y, *text_c)?;
                }
                MenuItemType::AppearingText {
                    text,
                    text_idx,
                    text_size,
                    text_c,
                    timer,
                } => {
                    window.get_font_mut(&self.font)?.draw(
                        if *text_idx < text.len() {
                            &text[0..*text_idx]
                        } else {
                            text
                        },
                        20,
                        rect.x,
                        rect.y,
                        *text_c,
                    )?;
                }
                MenuItemType::InstantText {
                    text,
                    text_size,
                    text_color,
                } => {
                    window.get_font_mut(&self.font)?.draw(
                        text,
                        text_size.round() as u32,
                        rect.x,
                        rect.y,
                        *text_color,
                    )?;
                }
                MenuItemType::Pause { timer, length } => (),
            }
        }
        self.player_particles.draw(window, Transform::IDENTITY);
        window.get_gi_mut().draw_rect_transform(
            self.player,
            Color::from_rgba(255, 255, 255, (self.player_particles.opacity * 255.0) as u8),
            Transform::translate(self.player.w / 2.0, self.player.h / 2.0)
                * Transform::rotate(self.player_r),
            Vector {
                x: self.player.x + self.player.w / 2.0,
                y: self.player.y + self.player.h / 2.0,
            },
        )?;
        self.joining_particles.draw(window, Transform::IDENTITY);
        for expl_conv_ps in &mut self.expl_conv_p_systems {
            expl_conv_ps.draw(window, Transform::IDENTITY);
        }
        for planet in &mut self.planets {
            planet.draw(window, Transform::IDENTITY);
        }

        for star in &mut self.stars {
            star.draw(&self.i_star, window, Transform::IDENTITY);
        }

        for fish in &mut self.fishes {
            fish.draw(&self.i_fish, window, Transform::IDENTITY);
        }

        if let Some(sl) = &mut self.save_load_notification {
            match sl {
                SaveLoadNotification::Save { text, timer }
                | SaveLoadNotification::Load { text, timer } => {
                    if let Some(s) = text {
                        window.get_font_mut(&self.font)?.draw(
                            s,
                            20,
                            20.0,
                            20.0,
                            Color::from_rgba(
                                255,
                                255,
                                255,
                                ((*timer / SL_NOTIF_TIME) as f32 * 255.0) as u8,
                            ),
                        )?;
                    }
                }
            }
        }
        window.get_gi_mut().end_drawing()?;

        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn save(&mut self) -> IOResult<()> {
        use std::io::Write;

        let save_bytes = SaveData {
            planets: self.planets.clone(),
            stars: self.stars.clone(),
            fishes: self.fishes.clone(),
            player: self.player,
            joining_particles: self.joining_particles.clone(),
        }
        .serialize();
        let mut file = File::create(SAVE_FILENAME)?;
        file.write_all(&save_bytes)?;
        self.save_load_notification = Some(SaveLoadNotification::Save {
            text: None,
            timer: SL_NOTIF_TIME,
        });

        Ok(())
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn load(&mut self) -> IOResult<()> {
        use std::io::Read;

        let mut bytes = Vec::new();
        {
            let mut file = File::open(SAVE_FILENAME)?;

            file.read_to_end(&mut bytes)?;
        }
        let (save_data, _) = SaveData::deserialize(&bytes).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to deserialize into SaveData!",
            )
        })?;

        self.planets = save_data.planets;
        self.stars = save_data.stars;
        self.fishes = save_data.fishes;
        self.player = save_data.player;
        self.joining_particles = save_data.joining_particles;
        self.expl_conv_p_systems.clear();
        self.move_to = Vector::new(self.player.x, self.player.y);
        self.camera
            .set_view_xy(
                self.player.x - WIDTH_F / 2.0,
                self.player.y - HEIGHT_F / 2.0,
            )
            .ok();
        self.dbl_click_timeout = None;
        self.click_time = None;
        self.click_release_time = DOUBLE_CLICK_TIME;
        self.state = 10;
        self.state_dirty = true;
        self.save_load_notification = Some(SaveLoadNotification::Load {
            text: None,
            timer: SL_NOTIF_TIME,
        });

        Ok(())
    }

    #[cfg(target_family = "wasm")]
    pub fn save(&mut self) -> IOResult<()> {
        Ok(())
    }

    #[cfg(target_family = "wasm")]
    pub fn load(&mut self) -> IOResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de_serialize_particle() {
        let mut particle = Particle::default();
        particle.rect.x = 1.0;
        particle.rect.y = 2.0;
        particle.rect.w = 3.0;
        particle.rect.h = 4.0;
        particle.circle.x = 5.0;
        particle.circle.y = 6.0;
        particle.circle.r = 7.0;
        particle.is_rect = true;
        particle.velx = 8.0;
        particle.vely = 9.0;
        particle.velr = 10.0;
        particle.r = 11.0;
        particle.lifetime = 12.0;
        particle.life_timer = 13.0;
        let bytes = particle.serialize();
        let (des_particle, size) =
            Particle::deserialize(&bytes, 0).expect("Should be able to deserialize Particle!");
        assert_eq!(particle, des_particle);
        assert_eq!(bytes.len(), size);

        let mut particle = Particle::default();
        particle.rect.x = 14.0;
        particle.rect.y = 15.0;
        particle.rect.w = 16.0;
        particle.rect.h = 17.0;
        particle.circle.x = 18.0;
        particle.circle.y = 19.0;
        particle.circle.r = 20.0;
        particle.is_rect = false;
        particle.velx = 21.0;
        particle.vely = 22.0;
        particle.velr = 23.0;
        particle.r = 24.0;
        particle.lifetime = 25.0;
        particle.life_timer = 26.0;
        let bytes = particle.serialize();
        let (des_particle, size) =
            Particle::deserialize(&bytes, 0).expect("Should be able to deserialize Particle!");
        assert_eq!(particle, des_particle);
        assert_eq!(bytes.len(), size);
    }

    #[test]
    fn test_de_serialize_particle_system() {
        let mut ps = ParticleSystem::default();
        let mut particle = Particle::default();
        particle.rect.x = 1.0;
        particle.rect.y = 2.0;
        particle.rect.w = 3.0;
        particle.rect.h = 4.0;
        particle.circle.x = 5.0;
        particle.circle.y = 6.0;
        particle.circle.r = 7.0;
        particle.is_rect = true;
        particle.velx = 8.0;
        particle.vely = 9.0;
        particle.velr = 10.0;
        particle.r = 11.0;
        particle.lifetime = 12.0;
        particle.life_timer = 13.0;
        ps.particles.push(particle);

        let mut particle = Particle::default();
        particle.rect.x = 14.0;
        particle.rect.y = 15.0;
        particle.rect.w = 16.0;
        particle.rect.h = 17.0;
        particle.circle.x = 18.0;
        particle.circle.y = 19.0;
        particle.circle.r = 20.0;
        particle.is_rect = false;
        particle.velx = 21.0;
        particle.vely = 22.0;
        particle.velr = 23.0;
        particle.r = 24.0;
        particle.lifetime = 25.0;
        particle.life_timer = 26.0;
        ps.particles.push(particle);

        ps.spawn_timer = 27.0;
        ps.spawn_time = 28.0;
        ps.lifetime = 29.0;
        ps.host_rect.x = 30.0;
        ps.host_rect.y = 31.0;
        ps.host_rect.w = 32.0;
        ps.host_rect.h = 33.0;
        ps.host_circle.x = 34.0;
        ps.host_circle.y = 35.0;
        ps.host_circle.r = 36.0;
        ps.is_rect = true;
        ps.direction.x = 37.0;
        ps.direction.y = 38.0;
        ps.color.r = 39;
        ps.color.g = 40;
        ps.color.b = 41;
        ps.color.a = 42;
        ps.opacity = 43.0;
        ps.vel_multiplier = 44.0;

        let bytes = ps.serialize();
        let (des_ps, size) = ParticleSystem::deserialize(&bytes, 0)
            .expect("Should be able to deserialize ParticleSystem!");
        assert_eq!(ps, des_ps);
        assert_eq!(bytes.len(), size);

        let mut ps = ParticleSystem::default();
        let mut particle = Particle::default();
        particle.rect.x = 45.0;
        particle.rect.y = 46.0;
        particle.rect.w = 47.0;
        particle.rect.h = 48.0;
        particle.circle.x = 49.0;
        particle.circle.y = 50.0;
        particle.circle.r = 51.0;
        particle.is_rect = true;
        particle.velx = 52.0;
        particle.vely = 53.0;
        particle.velr = 54.0;
        particle.r = 55.0;
        particle.lifetime = 56.0;
        particle.life_timer = 57.0;
        ps.particles.push(particle);

        let mut particle = Particle::default();
        particle.rect.x = 58.0;
        particle.rect.y = 59.0;
        particle.rect.w = 60.0;
        particle.rect.h = 61.0;
        particle.circle.x = 62.0;
        particle.circle.y = 63.0;
        particle.circle.r = 64.0;
        particle.is_rect = false;
        particle.velx = 65.0;
        particle.vely = 66.0;
        particle.velr = 67.0;
        particle.r = 68.0;
        particle.lifetime = 69.0;
        particle.life_timer = 70.0;
        ps.particles.push(particle);

        ps.spawn_timer = 71.0;
        ps.spawn_time = 72.0;
        ps.lifetime = 73.0;
        ps.host_rect.x = 74.0;
        ps.host_rect.y = 75.0;
        ps.host_rect.w = 76.0;
        ps.host_rect.h = 77.0;
        ps.host_circle.x = 78.0;
        ps.host_circle.y = 79.0;
        ps.host_circle.r = 80.0;
        ps.is_rect = false;
        ps.direction.x = 81.0;
        ps.direction.y = 82.0;
        ps.color.r = 83;
        ps.color.g = 84;
        ps.color.b = 85;
        ps.color.a = 86;
        ps.opacity = 87.0;
        ps.vel_multiplier = 88.0;

        let bytes = ps.serialize();
        let (des_ps, size) = ParticleSystem::deserialize(&bytes, 0)
            .expect("Should be able to deserialize ParticleSystem!");
        assert_eq!(ps, des_ps);
        assert_eq!(bytes.len(), size);
    }

    #[test]
    fn test_de_serialize_rotating_particle_system() {
        let mut ps = ParticleSystem::default();
        let mut particle = Particle::default();
        particle.rect.x = 1.0;
        particle.rect.y = 2.0;
        particle.rect.w = 3.0;
        particle.rect.h = 4.0;
        particle.circle.x = 5.0;
        particle.circle.y = 6.0;
        particle.circle.r = 7.0;
        particle.is_rect = true;
        particle.velx = 8.0;
        particle.vely = 9.0;
        particle.velr = 10.0;
        particle.r = 11.0;
        particle.lifetime = 12.0;
        particle.life_timer = 13.0;
        ps.particles.push(particle);

        let mut particle = Particle::default();
        particle.rect.x = 14.0;
        particle.rect.y = 15.0;
        particle.rect.w = 16.0;
        particle.rect.h = 17.0;
        particle.circle.x = 18.0;
        particle.circle.y = 19.0;
        particle.circle.r = 20.0;
        particle.is_rect = false;
        particle.velx = 21.0;
        particle.vely = 22.0;
        particle.velr = 23.0;
        particle.r = 24.0;
        particle.lifetime = 25.0;
        particle.life_timer = 26.0;
        ps.particles.push(particle);

        ps.spawn_timer = 27.0;
        ps.spawn_time = 28.0;
        ps.lifetime = 29.0;
        ps.host_rect.x = 30.0;
        ps.host_rect.y = 31.0;
        ps.host_rect.w = 32.0;
        ps.host_rect.h = 33.0;
        ps.host_circle.x = 34.0;
        ps.host_circle.y = 35.0;
        ps.host_circle.r = 36.0;
        ps.is_rect = true;
        ps.direction.x = 37.0;
        ps.direction.y = 38.0;
        ps.color.r = 39;
        ps.color.g = 40;
        ps.color.b = 41;
        ps.color.a = 42;
        ps.opacity = 43.0;
        ps.vel_multiplier = 44.0;

        let mut rps = RotatingParticleSystem::default();
        rps.particle_system = ps;
        rps.r = 45.0;
        rps.velr = 46.0;
        rps.offset = 47.0;

        let bytes = rps.serialize();
        let (des_rps, size) = RotatingParticleSystem::deserialize(&bytes, 0)
            .expect("Should be able to deserialize RotatingParticleSystem!");
        assert_eq!(rps, des_rps);
        assert_eq!(bytes.len(), size);
    }

    #[test]
    fn test_de_serialize_planet() {
        let mut planet = Planet::default();
        planet.circle.x = 1.0;
        planet.circle.y = 2.0;
        planet.circle.r = 3.0;
        planet.color.r = 4;
        planet.color.g = 5;
        planet.color.b = 6;
        planet.color.a = 7;

        let mut particle = Particle::default();
        particle.rect.x = 8.0;
        particle.rect.y = 9.0;
        particle.rect.w = 10.0;
        particle.rect.h = 11.0;
        particle.circle.x = 12.0;
        particle.circle.y = 13.0;
        particle.circle.r = 14.0;
        particle.is_rect = true;
        particle.velx = 15.0;
        particle.vely = 16.0;
        particle.velr = 17.0;
        particle.r = 18.0;
        particle.lifetime = 19.0;
        particle.life_timer = 20.0;
        planet.particle_system.particles.push(particle);

        let mut particle = Particle::default();
        particle.rect.x = 21.0;
        particle.rect.y = 22.0;
        particle.rect.w = 23.0;
        particle.rect.h = 24.0;
        particle.circle.x = 25.0;
        particle.circle.y = 26.0;
        particle.circle.r = 27.0;
        particle.is_rect = false;
        particle.velx = 28.0;
        particle.vely = 29.0;
        particle.velr = 30.0;
        particle.r = 31.0;
        particle.lifetime = 32.0;
        particle.life_timer = 33.0;
        planet.particle_system.particles.push(particle);

        planet.particle_system.spawn_timer = 34.0;
        planet.particle_system.spawn_time = 35.0;
        planet.particle_system.lifetime = 36.0;
        planet.particle_system.host_rect.x = 37.0;
        planet.particle_system.host_rect.y = 38.0;
        planet.particle_system.host_rect.w = 39.0;
        planet.particle_system.host_rect.h = 40.0;
        planet.particle_system.host_circle.x = 41.0;
        planet.particle_system.host_circle.y = 42.0;
        planet.particle_system.host_circle.r = 43.0;
        planet.particle_system.is_rect = true;
        planet.particle_system.direction.x = 44.0;
        planet.particle_system.direction.y = 45.0;
        planet.particle_system.color.r = 46;
        planet.particle_system.color.g = 47;
        planet.particle_system.color.b = 48;
        planet.particle_system.color.a = 49;
        planet.particle_system.opacity = 50.0;
        planet.particle_system.vel_multiplier = 51.0;

        let mut rps = RotatingParticleSystem::default();
        rps.particle_system = planet.particle_system.clone();
        rps.r = 52.0;
        rps.velr = 53.0;
        rps.offset = 54.0;
        planet.moons.push(rps);

        let mut rps = RotatingParticleSystem::default();
        rps.particle_system = ParticleSystem::default();
        rps.r = 55.0;
        rps.velr = 56.0;
        rps.offset = 57.0;
        planet.moons.push(rps);

        let bytes = planet.serialize();
        let (des_planet, size) =
            Planet::deserialize(&bytes, 0).expect("Should be able to deserialize Planet!");
        assert_eq!(planet, des_planet);
        assert_eq!(bytes.len(), size);
    }

    #[test]
    fn test_de_serialize_star() {
        let mut star = Star::default();
        star.color.r = 1;
        star.color.g = 2;
        star.color.b = 3;
        star.color.a = 4;

        let mut particle = Particle::default();
        particle.rect.x = 5.0;
        particle.rect.y = 6.0;
        particle.rect.w = 7.0;
        particle.rect.h = 8.0;
        particle.circle.x = 9.0;
        particle.circle.y = 10.0;
        particle.circle.r = 11.0;
        particle.is_rect = true;
        particle.velx = 12.0;
        particle.vely = 13.0;
        particle.velr = 14.0;
        particle.r = 15.0;
        particle.lifetime = 16.0;
        particle.life_timer = 17.0;
        star.particle_system.particles.push(particle);

        let mut particle = Particle::default();
        particle.rect.x = 18.0;
        particle.rect.y = 19.0;
        particle.rect.w = 20.0;
        particle.rect.h = 21.0;
        particle.circle.x = 22.0;
        particle.circle.y = 23.0;
        particle.circle.r = 24.0;
        particle.is_rect = false;
        particle.velx = 25.0;
        particle.vely = 26.0;
        particle.velr = 27.0;
        particle.r = 28.0;
        particle.lifetime = 29.0;
        particle.life_timer = 30.0;
        star.particle_system.particles.push(particle);

        star.particle_system.spawn_timer = 31.0;
        star.particle_system.spawn_time = 32.0;
        star.particle_system.lifetime = 33.0;
        star.particle_system.host_rect.x = 34.0;
        star.particle_system.host_rect.y = 35.0;
        star.particle_system.host_rect.w = 36.0;
        star.particle_system.host_rect.h = 37.0;
        star.particle_system.host_circle.x = 38.0;
        star.particle_system.host_circle.y = 39.0;
        star.particle_system.host_circle.r = 40.0;
        star.particle_system.is_rect = true;
        star.particle_system.direction.x = 41.0;
        star.particle_system.direction.y = 42.0;
        star.particle_system.color.r = 43;
        star.particle_system.color.g = 44;
        star.particle_system.color.b = 45;
        star.particle_system.color.a = 46;
        star.particle_system.opacity = 47.0;
        star.particle_system.vel_multiplier = 48.0;

        star.velr = 49.0;
        star.r = 50.0;

        let bytes = star.serialize();
        let (des_star, size) =
            Star::deserialize(&bytes, 0).expect("Should be able to deserialize Star!");
        assert_eq!(star, des_star);
        assert_eq!(bytes.len(), size);
    }

    #[test]
    fn test_de_serialize_fish() {
        let mut fish = Fish::default();
        fish.pos.x = 1.0;
        fish.pos.y = 2.0;
        fish.r = 3.0;
        fish.swim_time = 4.0;
        fish.swim_timer = 5.0;
        fish.swim_v = 6.0;
        fish.anim_timer = 7.0;
        fish.anim_time = 8.0;
        fish.color.r = 9;
        fish.color.g = 10;
        fish.color.b = 11;
        fish.color.a = 12;
        fish.body_rect.x = 13.0;
        fish.body_rect.y = 14.0;
        fish.body_rect.w = 15.0;
        fish.body_rect.h = 16.0;
        fish.tail_rect.x = 17.0;
        fish.tail_rect.y = 18.0;
        fish.tail_rect.w = 19.0;
        fish.tail_rect.h = 20.0;

        let bytes = fish.serialize();
        let (des_fish, size) =
            Fish::deserialize(&bytes, 0).expect("Should be able to deserialize Fish!");
        assert_eq!(fish, des_fish);
        assert_eq!(bytes.len(), size);
    }

    #[test]
    fn test_de_serialize_save_data() {
        let mut save_data = SaveData::default();
        let mut planet = Planet::default();
        planet.circle.x = 1.0;
        planet.circle.y = 2.0;
        planet.circle.r = 3.0;
        planet.color.r = 4;
        planet.color.g = 5;
        planet.color.b = 6;
        planet.color.a = 7;

        let mut particle = Particle::default();
        particle.rect.x = 8.0;
        particle.rect.y = 9.0;
        particle.rect.w = 10.0;
        particle.rect.h = 11.0;
        particle.circle.x = 12.0;
        particle.circle.y = 13.0;
        particle.circle.r = 14.0;
        particle.is_rect = true;
        particle.velx = 15.0;
        particle.vely = 16.0;
        particle.velr = 17.0;
        particle.r = 18.0;
        particle.lifetime = 19.0;
        particle.life_timer = 20.0;
        planet.particle_system.particles.push(particle);

        let mut particle = Particle::default();
        particle.rect.x = 21.0;
        particle.rect.y = 22.0;
        particle.rect.w = 23.0;
        particle.rect.h = 24.0;
        particle.circle.x = 25.0;
        particle.circle.y = 26.0;
        particle.circle.r = 27.0;
        particle.is_rect = false;
        particle.velx = 28.0;
        particle.vely = 29.0;
        particle.velr = 30.0;
        particle.r = 31.0;
        particle.lifetime = 32.0;
        particle.life_timer = 33.0;
        planet.particle_system.particles.push(particle);

        planet.particle_system.spawn_timer = 34.0;
        planet.particle_system.spawn_time = 35.0;
        planet.particle_system.lifetime = 36.0;
        planet.particle_system.host_rect.x = 37.0;
        planet.particle_system.host_rect.y = 38.0;
        planet.particle_system.host_rect.w = 39.0;
        planet.particle_system.host_rect.h = 40.0;
        planet.particle_system.host_circle.x = 41.0;
        planet.particle_system.host_circle.y = 42.0;
        planet.particle_system.host_circle.r = 43.0;
        planet.particle_system.is_rect = true;
        planet.particle_system.direction.x = 44.0;
        planet.particle_system.direction.y = 45.0;
        planet.particle_system.color.r = 46;
        planet.particle_system.color.g = 47;
        planet.particle_system.color.b = 48;
        planet.particle_system.color.a = 49;
        planet.particle_system.opacity = 50.0;
        planet.particle_system.vel_multiplier = 51.0;

        let mut rps = RotatingParticleSystem::default();
        rps.particle_system = planet.particle_system.clone();
        rps.r = 52.0;
        rps.velr = 53.0;
        rps.offset = 54.0;
        planet.moons.push(rps);

        let mut rps = RotatingParticleSystem::default();
        rps.particle_system = ParticleSystem::default();
        rps.r = 55.0;
        rps.velr = 56.0;
        rps.offset = 57.0;
        planet.moons.push(rps);

        save_data.planets.push(planet.clone());

        planet.color.r = 58;
        planet.color.g = 59;
        planet.color.b = 60;
        planet.color.a = 61;

        save_data.planets.push(planet.clone());

        planet.color.r = 62;
        planet.color.g = 63;
        planet.color.b = 64;
        planet.color.a = 65;

        save_data.planets.push(planet);

        let mut star = Star::default();
        star.color.r = 1;
        star.color.g = 2;
        star.color.b = 3;
        star.color.a = 4;

        let mut particle = Particle::default();
        particle.rect.x = 5.0;
        particle.rect.y = 6.0;
        particle.rect.w = 7.0;
        particle.rect.h = 8.0;
        particle.circle.x = 9.0;
        particle.circle.y = 10.0;
        particle.circle.r = 11.0;
        particle.is_rect = true;
        particle.velx = 12.0;
        particle.vely = 13.0;
        particle.velr = 14.0;
        particle.r = 15.0;
        particle.lifetime = 16.0;
        particle.life_timer = 17.0;
        star.particle_system.particles.push(particle);

        let mut particle = Particle::default();
        particle.rect.x = 18.0;
        particle.rect.y = 19.0;
        particle.rect.w = 20.0;
        particle.rect.h = 21.0;
        particle.circle.x = 22.0;
        particle.circle.y = 23.0;
        particle.circle.r = 24.0;
        particle.is_rect = false;
        particle.velx = 25.0;
        particle.vely = 26.0;
        particle.velr = 27.0;
        particle.r = 28.0;
        particle.lifetime = 29.0;
        particle.life_timer = 30.0;
        star.particle_system.particles.push(particle);

        star.particle_system.spawn_timer = 31.0;
        star.particle_system.spawn_time = 32.0;
        star.particle_system.lifetime = 33.0;
        star.particle_system.host_rect.x = 34.0;
        star.particle_system.host_rect.y = 35.0;
        star.particle_system.host_rect.w = 36.0;
        star.particle_system.host_rect.h = 37.0;
        star.particle_system.host_circle.x = 38.0;
        star.particle_system.host_circle.y = 39.0;
        star.particle_system.host_circle.r = 40.0;
        star.particle_system.is_rect = true;
        star.particle_system.direction.x = 41.0;
        star.particle_system.direction.y = 42.0;
        star.particle_system.color.r = 43;
        star.particle_system.color.g = 44;
        star.particle_system.color.b = 45;
        star.particle_system.color.a = 46;
        star.particle_system.opacity = 47.0;
        star.particle_system.vel_multiplier = 48.0;

        star.velr = 49.0;
        star.r = 50.0;

        save_data.stars.push(star.clone());

        star.color.r = 51;
        star.color.g = 52;
        star.color.b = 53;
        star.color.a = 54;

        save_data.stars.push(star.clone());

        star.color.r = 55;
        star.color.g = 56;
        star.color.b = 57;
        star.color.a = 58;

        save_data.stars.push(star);

        let mut fish = Fish::default();
        fish.pos.x = 1.0;
        fish.pos.y = 2.0;
        fish.r = 3.0;
        fish.swim_time = 4.0;
        fish.swim_timer = 5.0;
        fish.swim_v = 6.0;
        fish.anim_timer = 7.0;
        fish.anim_time = 8.0;
        fish.color.r = 9;
        fish.color.g = 10;
        fish.color.b = 11;
        fish.color.a = 12;
        fish.body_rect.x = 13.0;
        fish.body_rect.y = 14.0;
        fish.body_rect.w = 15.0;
        fish.body_rect.h = 16.0;
        fish.tail_rect.x = 17.0;
        fish.tail_rect.y = 18.0;
        fish.tail_rect.w = 19.0;
        fish.tail_rect.h = 20.0;

        save_data.fishes.push(fish.clone());

        fish.pos.x = 21.0;
        fish.pos.y = 22.0;
        fish.r = 23.0;

        save_data.fishes.push(fish.clone());

        fish.pos.x = 24.0;
        fish.pos.y = 25.0;
        fish.r = 26.0;

        save_data.fishes.push(fish);

        save_data.player.x = 100.0;
        save_data.player.y = 101.0;
        save_data.player.w = 102.0;
        save_data.player.h = 103.0;

        let mut ps = ParticleSystem::default();
        let mut particle = Particle::default();
        particle.rect.x = 1.0;
        particle.rect.y = 2.0;
        particle.rect.w = 3.0;
        particle.rect.h = 4.0;
        particle.circle.x = 5.0;
        particle.circle.y = 6.0;
        particle.circle.r = 7.0;
        particle.is_rect = true;
        particle.velx = 8.0;
        particle.vely = 9.0;
        particle.velr = 10.0;
        particle.r = 11.0;
        particle.lifetime = 12.0;
        particle.life_timer = 13.0;
        ps.particles.push(particle);

        let mut particle = Particle::default();
        particle.rect.x = 14.0;
        particle.rect.y = 15.0;
        particle.rect.w = 16.0;
        particle.rect.h = 17.0;
        particle.circle.x = 18.0;
        particle.circle.y = 19.0;
        particle.circle.r = 20.0;
        particle.is_rect = false;
        particle.velx = 21.0;
        particle.vely = 22.0;
        particle.velr = 23.0;
        particle.r = 24.0;
        particle.lifetime = 25.0;
        particle.life_timer = 26.0;
        ps.particles.push(particle);

        ps.spawn_timer = 27.0;
        ps.spawn_time = 28.0;
        ps.lifetime = 29.0;
        ps.host_rect.x = 30.0;
        ps.host_rect.y = 31.0;
        ps.host_rect.w = 32.0;
        ps.host_rect.h = 33.0;
        ps.host_circle.x = 34.0;
        ps.host_circle.y = 35.0;
        ps.host_circle.r = 36.0;
        ps.is_rect = true;
        ps.direction.x = 37.0;
        ps.direction.y = 38.0;
        ps.color.r = 39;
        ps.color.g = 40;
        ps.color.b = 41;
        ps.color.a = 42;
        ps.opacity = 43.0;
        ps.vel_multiplier = 44.0;

        save_data.joining_particles.particle_system = ps;
        save_data.joining_particles.r = 45.0;
        save_data.joining_particles.velr = 46.0;
        save_data.joining_particles.offset = 47.0;

        let bytes = save_data.serialize();
        let (des_save_data, size) =
            SaveData::deserialize(&bytes).expect("Should be able to deserialize SaveData!");
        assert_eq!(save_data, des_save_data);
        assert_eq!(bytes.len(), size);
    }
}

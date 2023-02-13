//use quicksilver::{
//    geom::{Circle, Rectangle, Transform, Vector},
//    graphics::{
//        Background::{Blended, Col, Img},
//        Color, Font, FontStyle, Image, View,
//    },
//    input::{ButtonState, Key},
//    lifecycle::{run, Asset, Event, Settings, State, Window},
//    saving::{load, save},
//    sound::Sound,
//    Result,
//};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

mod agnostic_interface;
mod faux_quicksilver;
use faux_quicksilver::{Color, Image, Rectangle, Circle, Vector, Transform, Window, Sound, Font, FontStyle, Event, Key, View};

const WIDTH_F: f32 = 800.0;
const HEIGHT_F: f32 = 600.0;
const MUSIC2_LENGTH: f64 = 2.0 * 60.0 * 1000.0;
const TEXT_RATE: f64 = 100.0;
const PP_GEN_RATE: f64 = 75.0;
const PARTICLE_RAND_VEL_RANGE: f32 = 0.2;
const PARTICLE_RAND_VEL_DIST: f32 = 0.2828427; // dist where x and y = 0.2
const PARTICLE_RAND_ROT_RANGE: f32 = 0.5;
const JOINING_OPACITY_RATE: f32 = 0.00013;
const JOINING_FAR_DIST: f32 = 700.0;
const JOINING_NEAR_DIST: f32 = 150.0;
const DOUBLE_CLICK_TIME: f64 = 350.0;
const SL_NOTIF_TIME: f64 = 5000.0;
const MAX_MOONS: usize = 5;

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
        text_image: Option<Image>,
        text_c: Color,
        h_c: Color,
        c: Color,
    },
    AppearingText {
        text: &'static str,
        text_image: Option<Image>,
        current_text: String,
        text_size: f32,
        text_c: Color,
        timer: f64,
    },
    InstantText {
        text: &'static str,
        text_image: Option<Image>,
        text_size: f32,
        text_color: Color,
    },
    Pause {
        timer: f64,
        length: f64,
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
                text_image: None,
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
                text_image: None,
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
                    55.0,
                    true,
                    "One And All - A Ludum Dare 45 Entry",
                ),
                Menu::instant_text(
                    25.0,
                    HEIGHT_F - 100.0,
                    30.0,
                    true,
                    "Made with quicksilver which is licensed with either",
                ),
                Menu::instant_text(
                    70.0,
                    HEIGHT_F - 80.0,
                    30.0,
                    true,
                    "MIT License or Apache License Version 2.0",
                ),
                Menu::instant_text(
                    25.0,
                    HEIGHT_F - 50.0,
                    30.0,
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
                text_image: None,
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
                text_image: None,
                text_size,
                current_text: String::new(),
                text_c: Color::WHITE,
                timer: 0.0,
            },
            is_hover: false,
            is_focus: false,
            is_loaded: !first,
        }
    }

    fn pause(length: f64, first: bool) -> MenuItem {
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
                Menu::pause(500.0, true),
                Menu::text(50.0, HEIGHT_F - 140.0, 40.0, false, "This is how it is."),
                Menu::pause(500.0, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 100.0,
                    40.0,
                    false,
                    "Nothing is, and everything is nothing.",
                ),
                Menu::pause(500.0, false),
                Menu::text(50.0, HEIGHT_F - 60.0, 40.0, false, "...until you appeared."),
                Menu::pause(100.0, false),
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
                Menu::pause(500.0, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 110.0,
                    40.0,
                    false,
                    "What brings you here? What drives you?",
                ),
                Menu::pause(500.0, false),
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
                Menu::pause(500.0, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 130.0,
                    40.0,
                    false,
                    "Hope that a brighter future will come tomorrow...",
                ),
                Menu::pause(500.0, false),
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
                Menu::pause(500.0, false),
                Menu::text(
                    30.0,
                    HEIGHT_F - 130.0,
                    40.0,
                    false,
                    "With your appearance, things may change for the better..",
                ),
                Menu::pause(500.0, false),
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
                Menu::pause(250.0, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 130.0,
                    40.0,
                    false,
                    "It has been a long time since I have encountered",
                ),
                Menu::text(50.0, HEIGHT_F - 90.0, 40.0, false, "another being..."),
                Menu::pause(500.0, false),
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
                Menu::pause(500.0, false),
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
                Menu::pause(500.0, false),
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
                Menu::pause(200.0, false),
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
                Menu::pause(400.0, true),
                Menu::text(
                    50.0,
                    HEIGHT_F - 140.0,
                    40.0,
                    false,
                    "A new planet... It has most certainly been a while.",
                ),
                Menu::pause(500.0, false),
                Menu::text(
                    50.0,
                    HEIGHT_F - 100.0,
                    40.0,
                    false,
                    "Please, go out and create the new universe, and again..",
                ),
                Menu::pause(300.0, false),
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

#[derive(Serialize, Deserialize, Clone)]
struct Particle {
    rect: Rectangle,
    circle: Circle,
    is_rect: bool,
    velx: f32,
    vely: f32,
    velr: f32,
    r: f32,
    lifetime: f64,
    life_timer: f64,
}

#[derive(Serialize, Deserialize, Clone)]
struct ParticleSystem {
    particles: Vec<Particle>,
    spawn_timer: f64,
    spawn_time: f64,
    lifetime: f64,
    host_rect: Rectangle,
    host_circle: Circle,
    is_rect: bool,
    direction: Vector,
    color: Color,
    opacity: f32,
    vel_multiplier: f32,
}

impl ParticleSystem {
    fn new(
        spawn_time: f64,
        lifetime: f64,
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

    fn update(&mut self, dt: f64) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].life_timer += dt;
            if self.particles[i].life_timer > self.particles[i].lifetime {
                self.particles.swap_remove(i);
            } else {
                if self.is_rect {
                    self.particles[i].rect.x += self.particles[i].velx * dt as f32;
                    self.particles[i].rect.y += self.particles[i].vely * dt as f32;
                    self.particles[i].r += self.particles[i].velr * dt as f32;
                } else {
                    self.particles[i].circle.x += self.particles[i].velx * dt as f32;
                    self.particles[i].circle.y += self.particles[i].vely * dt as f32;
                }
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
                    .gen_range(-PARTICLE_RAND_VEL_RANGE, PARTICLE_RAND_VEL_RANGE)
                    + self.direction.x)
                    * self.vel_multiplier,
                vely: (rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_VEL_RANGE, PARTICLE_RAND_VEL_RANGE)
                    + self.direction.y)
                    * self.vel_multiplier,
                // velx: self.direction.x,
                // vely: self.direction.y,
                velr: rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_ROT_RANGE, PARTICLE_RAND_ROT_RANGE)
                    * self.vel_multiplier,
                r: rand::thread_rng().gen_range(0.0, 90.0),
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
            self.color.a = ((1.0 - (particle.life_timer / particle.lifetime) as f32) * self.opacity * 255.0) as u8;
            if particle.is_rect {
                let pre_transform = Transform::translate(
                    -particle.rect.x / 2.0,
                    -particle.rect.y / 2.0,
                ) * Transform::rotate(particle.r);
                window.draw_ex(
                    &particle.rect,
                    Col(self.color),
                    transform * pre_transform,
                    1,
                );
            } else {
                window.draw_ex(&particle.circle, Col(self.color), transform, 1);
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
                    .gen_range(-PARTICLE_RAND_VEL_RANGE, PARTICLE_RAND_VEL_RANGE)
                    + self.direction.x)
                    * self.vel_multiplier,
                vely: (rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_VEL_RANGE, PARTICLE_RAND_VEL_RANGE)
                    + self.direction.y)
                    * self.vel_multiplier,
                // velx: self.direction.x,
                // vely: self.direction.y,
                velr: rand::thread_rng()
                    .gen_range(-PARTICLE_RAND_ROT_RANGE, PARTICLE_RAND_ROT_RANGE)
                    * self.vel_multiplier,
                r: rand::thread_rng().gen_range(0.0, 90.0),
                lifetime: self.lifetime,
                life_timer: 0.0,
            });
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct RotatingParticleSystem {
    particle_system: ParticleSystem,
    r: f32,
    velr: f32,
    offset: f32,
}

impl RotatingParticleSystem {
    fn new(
        spawn_time: f64,
        lifetime: f64,
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

    fn update(&mut self, dt: f64) {
        if self.particle_system.is_rect {
            let saved_rect = self.particle_system.host_rect;
            self.particle_system.host_rect.pos_add_vec(
                Transform::rotate(self.r) * Vector::new(self.offset, 0.0));
            self.particle_system.update(dt);
            self.particle_system.host_rect = saved_rect;
        } else {
            let saved_cir = self.particle_system.host_circle;
            self.particle_system.host_circle.pos_add_vec(
                Transform::rotate(self.r) * Vector::new(self.offset, 0.0));
            self.particle_system.update(dt);
            self.particle_system.host_circle = saved_cir;
        }
        self.r += self.velr * dt as f32;
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
            window.draw_ex(
                &moved_rect,
                Col(solid_color),
                transform
                    * Transform::translate(-moved_rect.x / 2.0, -moved_rect.y / 2.0)
                    * Transform::rotate(self.r * 1.3),
                1,
            );
        } else {
            let mut moved_cir = self.particle_system.host_circle;
            moved_cir.pos_add_vec(Transform::rotate(self.r) * Vector::new(self.offset, 0.0));
            let mut solid_color = self.particle_system.color;
            solid_color.a = (self.particle_system.opacity * 255.0) as u8;
            window.draw_ex(&moved_cir, Col(solid_color), transform, 1);
        }
    }
}

struct ExplConvCircleParticle {
    circle: Circle,
    offset: f32,
    r: f32,
}

struct ExplConvParticleSystem {
    particles: Vec<ExplConvCircleParticle>,
    lifetime: f64,
    host_circle: Circle,
    color: Color,
    opacity: f32,
    life_timer: f64,
}

impl ExplConvParticleSystem {
    fn new(lifetime: f64, host_circle: Circle, color: Color, opacity: f32) -> Self {
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
                r: rand::thread_rng().gen_range(0.0, 360.0),
            });
        }
    }

    // returns true if finished
    fn update(&mut self, dt: f64, planets: &mut Vec<Planet>) -> bool {
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
            let amount = interp_sq_inv((self.life_timer / self.lifetime) as f32 * 2.0);
            for particle in &mut self.particles {
                let dir =
                    Transform::rotate(particle.r) * Vector::new(particle.offset * amount, 0.0);
                particle.circle.x = dir.x + self.host_circle.x;
                particle.circle.y = dir.y + self.host_circle.y;
            }
        } else {
            let amount = 1.0 - interp_sq(((self.life_timer / self.lifetime) as f32 - 0.5) * 2.0);
            for particle in &mut self.particles {
                let dir =
                    Transform::rotate(particle.r) * Vector::new(particle.offset * amount, 0.0);
                particle.circle.x = dir.x + self.host_circle.x;
                particle.circle.y = dir.y + self.host_circle.y;
            }
        }
        return false;
    }

    fn draw(&mut self, window: &mut Window, transform: Transform) {
        if self.opacity == 0.0 {
            return;
        }
        for particle in &mut self.particles {
            self.color.a = (((self.life_timer / self.lifetime) as f32 / 2.0 + 0.5) * self.opacity * 255.0) as u8;
            window.draw_ex(&particle.circle, Col(self.color), transform, 1);
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Planet {
    circle: Circle,
    color: Color,
    particle_system: ParticleSystem,
    moons: Vec<RotatingParticleSystem>,
}

impl Planet {
    fn new(circle: Circle, color: Color) -> Self {
        let mut smaller_circle = circle;
        smaller_circle.r /= 4.0;
        let mut planet = Planet {
            circle,
            color,
            particle_system: ParticleSystem::new(
                rand::thread_rng().gen_range(2000.0, 3800.0),
                900.0,
                Rectangle::new(0.0, 0.0,1.0, 1.0),
                circle,
                false,
                Vector::new(0.0, 0.0),
                color,
                1.0,
                0.3,
            ),
            moons: Vec::with_capacity(MAX_MOONS),
        };

        let r: f32 = rand::thread_rng().gen_range(0.0, 360.0);
        let clockwise = rand::thread_rng().gen_bool(0.5);
        for i in 0..rand::thread_rng().gen_range(0, MAX_MOONS) {
            planet.moons.push(RotatingParticleSystem::new(
                rand::thread_rng().gen_range(1000.0, 2600.0),
                600.0,
                Rectangle::new(0.0, 0.0, 1.0, 1.0),
                smaller_circle,
                false,
                Vector::new(0.0, 0.0),
                color,
                1.0,
                r,
                if clockwise {
                    rand::thread_rng().gen_range(0.05, 0.15)
                } else {
                    rand::thread_rng().gen_range(-0.15, -0.05)
                },
                rand::thread_rng().gen_range(35.0, 200.0),
                0.2,
            ));
        }

        planet
    }

    fn update(&mut self, dt: f64) {
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
        window.draw_ex(&self.circle, Col(self.color), transform, 1);
        for moon in &mut self.moons {
            moon.draw(window, transform);
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Star {
    color: Color,
    particle_system: ParticleSystem,
    velr: f32,
    r: f32,
}

impl Star {
    fn new(circle: Circle, color: Color, velr: f32, r: f32) -> Self {
        let mut star = Star {
            color,
            particle_system: ParticleSystem::new(
                rand::thread_rng().gen_range(80.0, 200.0),
                850.0,
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
            .force_spawn(rand::thread_rng().gen_range(20, 45));

        star
    }

    fn update(&mut self, dt: f64) {
        self.particle_system.update(dt);
        self.r += self.velr * dt as f32;
    }

    fn draw(&mut self, image: &mut Image, window: &mut Window, transform: Transform) {
        self.particle_system.draw(window, transform);
        let mut image_rect = image.area_rect();
        image_rect.x = self.particle_system.host_circle.x - image_rect.w / 2.0;
        image_rect.y = self.particle_system.host_circle.y - image_rect.h / 2.0;
        window.draw_ex(
            &image_rect,
            Blended(image, self.color),
            transform * Transform::rotate(self.r),
            1,
        );
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Fish {
    pos: Vector,
    r: f32,
    swim_time: f64,
    swim_timer: f64,
    swim_v: f32,
    anim_timer: f64,
    anim_time: f64,
    color: Color,
}

enum FishState {
    Idle,
    Swim,
}

impl Fish {
    fn new(pos: Vector, r: f32, color: Color) -> Self {
        let anim_timer = rand::thread_rng().gen_range(800.0, 1000.0);
        Self {
            pos,
            r,
            swim_time: 800.0,
            swim_timer: 800.0,
            swim_v: 0.2,
            anim_timer,
            anim_time: anim_timer,
            color,
        }
    }

    fn set_next(&mut self, state: FishState) {
        match state {
            FishState::Idle => {
                self.swim_time = rand::thread_rng().gen_range(1100.0, 2400.0);
                self.swim_timer = self.swim_time;
                self.anim_timer = 1600.0;
                self.anim_time = 1600.0;
                self.swim_v = 0.0;
            }
            FishState::Swim => {
                self.swim_time = rand::thread_rng().gen_range(1400.0, 2300.0);
                self.swim_timer = self.swim_time;
                self.r = rand::thread_rng().gen_range(0.0, 360.0);
                self.anim_timer = rand::thread_rng().gen_range(600.0, 1000.0);
                self.anim_time = self.anim_timer;
                self.swim_v = (self.anim_timer / 8000.0) as f32;
            }
        }
    }

    fn update(&mut self, dt: f64) {
        self.swim_time -= dt;
        if self.swim_time < 220.0 {
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

        self.pos += Transform::rotate(self.r) * Vector::new(self.swim_v, 0.0) * dt as f32;
    }

    fn draw(
        &mut self,
        fish_body: &Image,
        fish_tail: &Image,
        window: &mut Window,
        transform: Transform,
    ) {
        let anim_angle = ((self.anim_timer / self.anim_time) * std::f64::consts::PI * 2.0).sin();
        let mut body_rect = fish_body.area_rect();
        body_rect.x = self.pos.x - body_rect.w / 2.0;
        body_rect.y = self.pos.y - body_rect.h / 2.0;
        let body_tr =
            Transform::rotate(anim_angle as f32 * 30.0) * Transform::rotate(self.r + 180.0);
        window.draw_ex(
            &body_rect,
            Blended(fish_body, self.color),
            transform * body_tr,
            1,
        );
        let mut tail_rect = fish_tail.area_rect();
        tail_rect.x = self.pos.x - tail_rect.w / 2.0;
        tail_rect.y = self.pos.y - tail_rect.h / 2.0;
        let anim_angle = ((self.anim_timer / self.anim_time) * std::f64::consts::PI * 2.0
            - std::f64::consts::PI / 3.0)
            .sin();
        let tail_tr = body_tr
            * Transform::translate(body_rect.x / 1.5, 0.0)
            * Transform::translate(-tail_rect.x / 2.0, 0.0)
            * Transform::rotate(-anim_angle as f32 * 45.0)
            * Transform::translate(tail_rect.x / 2.0, 0.0);
        window.draw_ex(
            &tail_rect,
            Blended(fish_tail, self.color),
            transform * tail_tr,
            1,
        );
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct SaveData {
    planets: Vec<Planet>,
    stars: Vec<Star>,
    fishes: Vec<Fish>,
    player: Rectangle,
    joining_particles: RotatingParticleSystem,
}

enum SaveLoadNotification {
    Save { text: Option<Image>, timer: f64 },
    Load { text: Option<Image>, timer: f64 },
}

struct GameState {
    s_boom: Sound,
    s_get: Sound,
    s_power_up: Sound,
    s_tap: Sound,
    s_speak_m: Sound,
    s_speak_f: Sound,
    font: Font,
    music2: Sound,
    i_star: Option<Image>,
    i_star_actual: Option<Image>,
    i_fish: Image,
    i_fish_body: Option<Image>,
    i_fish_tail: Option<Image>,
    music_on: bool,
    music_timer: f64,
    menu: Menu,
    state: u32,
    state_dirty: bool,
    selection_mode: bool,
    current_item: Option<usize>,
    current_finished: bool,
    player: Rectangle,
    player_r: f64,
    player_particles: ParticleSystem,
    joining_particles: RotatingParticleSystem,
    is_create_mode: bool,
    click_release_time: f64,
    dbl_click_timeout: Option<f64>,
    click_time: Option<f64>,
    click_pos: Vector,
    mouse_pos: Vector,
    expl_conv_p_systems: Vec<ExplConvParticleSystem>,
    planets: Vec<Planet>,
    stars: Vec<Star>,
    fishes: Vec<Fish>,
    camera: Rectangle,
    move_to: Vector,
    save_load_notification: Option<SaveLoadNotification>,
}

impl GameState {
    fn new() -> Result<Self, String> {
        Ok(Self {
            s_boom: Sound::load("boom.mp3"),
            s_get: Sound::load("get.mp3"),
            s_power_up: Sound::load("power_up.mp3"),
            s_tap: Sound::load("tap.mp3"),
            s_speak_m: Sound::load("speak_m.mp3"),
            s_speak_f: Sound::load("speak_f.mp3"),
            font: Font::load("ClearSans-Regular.ttf"),
            music2: Sound::load("music2.mp3"),
            i_star: Some(Image::load("star.png")),
            i_star_actual: None,
            i_fish: Image::load("fish.png"),
            i_fish_body: None,
            i_fish_tail: None,
            music_on: false,
            music_timer: 0.0,
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
                1000.0,
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
                1000.0,
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
            camera: Rectangle::new(0.0, 0.0, WIDTH_F, HEIGHT_F),
            move_to: Vector::new(400.0, 300.0),
            save_load_notification: None,
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<(), String> {
        match event {
            Event::MouseMoved(v) => {
                self.mouse_pos = *v;
                let mut hovered = false;
                for i in 0..self.menu.items.len() {
                    if self.menu.items[i].is_inside(v.x, v.y) {
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
            Event::MouseButton(button, state) => {
                if let ButtonState::Released = state {
                    if self.dbl_click_timeout.is_none() {
                        self.click_release_time = 0.0;
                    }
                } else if let ButtonState::Pressed = state {
                    if self.current_finished {
                        if self.is_create_mode {
                            if self.click_release_time < DOUBLE_CLICK_TIME {
                                self.click_release_time = DOUBLE_CLICK_TIME;
                                self.dbl_click_timeout = Some(0.0);
                                self.click_time = None;
                                if self.state == 8 {
                                    let mut expl_conv_system = ExplConvParticleSystem::new(
                                        1500.0,
                                        Circle::new(self.mouse_pos.x, self.mouse_pos.y, 20.0),
                                        Color::from_rgba(0x99, 0xFF, 0x99, 255),
                                        1.0,
                                    );
                                    expl_conv_system.activate(30, 200.0);
                                    self.expl_conv_p_systems.push(expl_conv_system);
                                    self.state = 9;
                                    self.state_dirty = true;
                                    self.s_boom.execute(|s| {
                                        s.set_volume(0.8);
                                        s.play()
                                    })?;
                                } else if self.state == 10 {
                                    let mut rng = rand::thread_rng();
                                    let rand_out = rng.gen_range(0.0, 1.0);
                                    if rand_out < 0.6 {
                                        // spawn planet
                                        let mut expl_conv_system = ExplConvParticleSystem::new(
                                            rng.gen_range(1200.0, 1600.0),
                                            Circle::new(self.mouse_pos.x,self.mouse_pos.y, rng.gen_range(15.0, 25.0)),
                                            Color::from_rgba(
                                                rng.gen_range(0x44, 0xFF),
                                                rng.gen_range(0x44, 0xFF),
                                                rng.gen_range(0x44, 0xFF),
                                                255,
                                            ),
                                            1.0,
                                        );
                                        expl_conv_system.activate(
                                            rng.gen_range(13, 40),
                                            rng.gen_range(150.0, 300.0),
                                        );
                                        self.expl_conv_p_systems.push(expl_conv_system);
                                    } else if rand_out < 0.85 {
                                        // spawn star
                                        let rot_clockwise = rng.gen_bool(0.5);
                                        self.stars.push(Star::new(
                                            Circle::new(self.mouse_pos.x, self.mouse_pos.y, rng.gen_range(3.0, 7.0)),
                                            Color::from_rgba(
                                                rng.gen_range(0x58, 0xFF),
                                                rng.gen_range(0x58, 0xFF),
                                                rng.gen_range(0x58, 0xFF),
                                                255,
                                            ),
                                            if rot_clockwise {
                                                rng.gen_range(0.1, 0.3)
                                            } else {
                                                rng.gen_range(-0.3, -0.1)
                                            },
                                            rng.gen_range(0.0, 90.0),
                                        ));
                                    } else {
                                        // spawn fish
                                        for i in 0..rng.gen_range(1, 4) {
                                            self.fishes.push(Fish::new(
                                                self.mouse_pos,
                                                rng.gen_range(0.0, 360.0),
                                                Color::from_rgba(
                                                    rng.gen_range(0x44, 0xFF),
                                                    rng.gen_range(0x44, 0xFF),
                                                    rng.gen_range(0x44, 0xFF),
                                                    255,
                                                ),
                                            ));
                                        }
                                    }
                                    self.s_boom.execute(|s| {
                                        s.set_volume(0.8);
                                        s.play()
                                    })?;
                                }
                            } else if self.state == 10 {
                                self.click_time = Some(0.0);
                                self.click_pos = self.mouse_pos;
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
                                        self.s_get.execute(|s| {
                                            s.set_volume(0.7);
                                            s.play()
                                        })?;
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
                                    text_image,
                                    current_text,
                                    text_size,
                                    text_c,
                                    timer,
                                } => {
                                    self.font.execute(|f| {
                                        *current_text = text.to_string();
                                        let style = FontStyle::new(*text_size, *text_c);
                                        *text_image = Some(f.render(text, &style)?);
                                        Ok(())
                                    })?;
                                }
                                MenuItemType::Button {
                                    text,
                                    text_image,
                                    text_c,
                                    h_c,
                                    c,
                                } => {
                                    if text_image.is_none() {
                                        self.font.execute(|font| {
                                            let style = FontStyle::new(42.0, *text_c);
                                            *text_image = Some(font.render(text, &style)?);
                                            Ok(())
                                        })?;
                                    }
                                }
                                MenuItemType::Pause { timer, length } => (),
                                MenuItemType::InstantText {
                                    text,
                                    text_image,
                                    text_size,
                                    text_color,
                                } => {
                                    if text_image.is_none() {
                                        self.font.execute(|f| {
                                            let style = FontStyle::new(*text_size, *text_color);
                                            *text_image = Some(f.render(text, &style)?);
                                            Ok(())
                                        })?;
                                    }
                                }
                            }
                            mi.is_loaded = true;
                        }
                        self.current_finished = true;
                    }
                }
            }
            Event::Key(key, state) => {
                if let ButtonState::Pressed = state {
                    match key {
                        Key::S => {
                            if self.state == 10 {
                                let save_data = SaveData {
                                    planets: self.planets.clone(),
                                    stars: self.stars.clone(),
                                    fishes: self.fishes.clone(),
                                    player: self.player.clone(),
                                    joining_particles: self.joining_particles.clone(),
                                };
                                save("OneAndAll_LD45", "slot0", &save_data)?;
                                self.save_load_notification = Some(SaveLoadNotification::Save {
                                    text: None,
                                    timer: SL_NOTIF_TIME,
                                });
                            }
                        }
                        Key::L => {
                            let load_result = load::<SaveData>("OneAndAll_LD45", "slot0");
                            if let Ok(save_data) = load_result {
                                self.planets = save_data.planets;
                                self.stars = save_data.stars;
                                self.fishes = save_data.fishes;
                                self.player = save_data.player;
                                self.joining_particles = save_data.joining_particles;
                                self.expl_conv_p_systems.clear();
                                self.move_to = Vector{ x: self.player.x, y: self.player.y};
                                self.camera.x = self.player.x - WIDTH_F / 2.0;
                                self.camera.y = self.player.y - HEIGHT_F / 2.0;
                                self.dbl_click_timeout = None;
                                self.click_time = None;
                                self.click_release_time = DOUBLE_CLICK_TIME;

                                self.state = 10;
                                self.state_dirty = true;
                                self.save_load_notification = Some(SaveLoadNotification::Load {
                                    text: None,
                                    timer: SL_NOTIF_TIME,
                                });
                            }
                        }
                        Key::R => {
                            if self.state == 10 {
                                self.state = 0;
                                self.state_dirty = true;
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<(), String> {
        let dt = window.update_rate();

        self.click_release_time += dt;
        if let Some(t) = &mut self.click_time {
            *t += dt;
            if *t > DOUBLE_CLICK_TIME {
                self.move_to = self.click_pos; // - Vector::new(WIDTH_F / 2.0, HEIGHT_F / 2.0);
            }
        }

        if let Some(t) = &mut self.dbl_click_timeout {
            *t += dt;
            if *t > 300.0 {
                self.dbl_click_timeout = None;
            }
        }

        self.player.x += (self.move_to.x - self.player.x) / 20.0;
        self.player.y += (self.move_to.y - self.player.y) / 20.0;
        self.player_particles.host_rect.x = self.player.x;
        self.player_particles.host_rect.y = self.player.y;
        self.joining_particles.particle_system.host_rect.x += (self.player.x - self.joining_particles.particle_system.host_rect.x) / 30.0;
        self.joining_particles.particle_system.host_rect.y += (self.player.y - self.joining_particles.particle_system.host_rect.y) / 30.0;
        self.camera.x += (self.player.x - WIDTH_F / 2.0 - self.camera.x) / 40.0;
        self.camera.y += (self.player.y - HEIGHT_F / 2.0 - self.camera.y) / 40.0;
        window.set_view(View::new(self.camera));

        self.player_r += dt / 10.0;

        if self.state_dirty {
            self.state_dirty = false;
            if self.state > 1 && !self.music_on {
                let mut music_on = false;
                self.music2.execute(|m2| {
                    music_on = true;
                    m2.set_volume(0.6);
                    m2.play()
                })?;
                if music_on {
                    self.music_on = true;
                    self.music_timer = 0.0;
                }
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
                    self.camera.x = 0.0;
                    self.camera.y = 0.0;
                    self.click_time = None;
                }
            }
        }

        if self.joining_particles.particle_system.opacity < 1.0 && self.state > 2 {
            self.joining_particles.particle_system.opacity += JOINING_OPACITY_RATE * dt as f32;
            if self.joining_particles.particle_system.opacity > 1.0 {
                self.joining_particles.particle_system.opacity = 1.0;
            }
            self.joining_particles.offset =
                (1.0 - self.joining_particles.particle_system.opacity / 1.0) * JOINING_FAR_DIST
                    + self.joining_particles.particle_system.opacity / 1.0 * JOINING_NEAR_DIST;
        }

        if self.player_particles.opacity < 1.0 && self.state > 1 {
            self.player_particles.opacity += dt as f32 / 7000.0;
            if self.player_particles.opacity > 1.0 {
                self.player_particles.opacity = 1.0;
            }
        }

        if self.music_on {
            self.music_timer += dt;
            if self.music_timer > MUSIC2_LENGTH {
                self.music_timer = 0.0;
                self.music2.execute(|m2| m2.play())?;
            }
        } else if self.state == 10 {
            let mut music_on = false;
            self.music2.execute(|m2| {
                music_on = true;
                m2.set_volume(0.6);
                m2.play()
            })?;
            if music_on {
                self.music_on = true;
                self.music_timer = 0.0;
            }
        }

        for i in 0..self.menu.items.len() {
            let mi: &mut MenuItem = &mut self.menu.items[i];
            if !mi.is_loaded {
                match &mut mi.item_type {
                    MenuItemType::Button {
                        text,
                        text_image,
                        text_c,
                        h_c,
                        c,
                    } => {
                        self.font.execute(|font| {
                            let style = FontStyle::new(42.0, *text_c);
                            *text_image = Some(font.render(text, &style)?);
                            Ok(())
                        })?;
                        if text_image.is_some() {
                            mi.is_loaded = true;
                            if i + 1 < self.menu.items.len() {
                                self.menu.items[i + 1].is_loaded = false;
                            } else {
                                self.current_finished = true;
                            }
                        }
                    }
                    MenuItemType::AppearingText {
                        text,
                        text_image,
                        current_text,
                        text_size,
                        text_c,
                        timer,
                    } => {
                        *timer += dt;
                        if *timer > TEXT_RATE {
                            *timer -= TEXT_RATE;
                            let next = text.chars().nth(current_text.len());
                            if let Some(next_t) = next {
                                current_text.push(next_t);
                                self.s_tap.execute(|s| {
                                    s.set_volume(0.2);
                                    s.play()
                                })?;
                            } else {
                                mi.is_loaded = true;
                                if i + 1 < self.menu.items.len() {
                                    self.menu.items[i + 1].is_loaded = false;
                                } else {
                                    self.current_finished = true;
                                }
                                continue;
                            }
                            self.font.execute(|font| {
                                let style = FontStyle::new(*text_size, *text_c);
                                *text_image = Some(font.render(current_text, &style)?);
                                Ok(())
                            })?;
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
                        text_image,
                        text_size,
                        text_color,
                    } => {
                        if text_image.is_none() {
                            self.font.execute(|f| {
                                let style = FontStyle::new(*text_size, *text_color);
                                *text_image = Some(f.render(text, &style)?);
                                Ok(())
                            })?;
                        }
                        if text_image.is_some() {
                            mi.is_loaded = true;
                            if i + 1 < self.menu.items.len() {
                                self.menu.items[i + 1].is_loaded = false;
                            } else {
                                self.current_finished = true;
                            }
                        }
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
                    if *timer <= 0.0 {
                        self.save_load_notification = None;
                    } else if text.is_none() {
                        self.font.execute(|f| {
                            *text = Some(
                                f.render("Saved the Game", &FontStyle::new(45.0, Color::WHITE))?,
                            );
                            Ok(())
                        })?;
                    }
                }
                SaveLoadNotification::Load { text, timer } => {
                    *timer -= dt;
                    if *timer <= 0.0 {
                        self.save_load_notification = None;
                    } else if text.is_none() {
                        self.font.execute(|f| {
                            *text = Some(
                                f.render("Loaded the Game", &FontStyle::new(45.0, Color::WHITE))?,
                            );
                            Ok(())
                        })?;
                    }
                }
            }
        }

        if self.i_star_actual.is_none() {
            if let Some(i_s) = &mut self.i_star {
                let mut star: Option<Image> = None;
                i_s.execute(|i| {
                    star = Some(i.clone());
                    Ok(())
                })?;
                self.i_star_actual = star;
            }
            if self.i_star_actual.is_some() {
                self.i_star = None;
            }
        }

        if self.i_fish_body.is_none() {
            let mut body: Option<Image> = None;
            self.i_fish.execute(|i| {
                body = Some(i.subimage(Rectangle::new(0.0, 0.0, 32.0, 16.0)));
                Ok(())
            })?;
            self.i_fish_body = body;
        }
        if self.i_fish_tail.is_none() {
            let mut tail: Option<Image> = None;
            self.i_fish.execute(|i| {
                tail = Some(i.subimage(Rectangle::new(32.0, 0.0, 16.0, 16.0)));
                Ok(())
            })?;
            self.i_fish_tail = tail;
        }

        for fish in &mut self.fishes {
            fish.update(dt);
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<(), String> {
        window.clear(Color::BLACK)?;
        let mut rect = Rectangle::default();
        for mi in &mut self.menu.items {
            rect.x = mi.x;
            rect.y = mi.y;
            rect.w = mi.w;
            rect.h = mi.h;
            match &mut mi.item_type {
                MenuItemType::Button {
                    text,
                    text_image,
                    text_c,
                    h_c,
                    c,
                } => {
                    if mi.is_hover {
                        window.draw(&rect, Col(*h_c));
                    } else {
                        window.draw(&rect, Col(*c));
                    }
                    if let Some(i) = text_image {
                        let mut image_rect = i.area_rect();
                        image_rect.x = mi.x + (mi.w - image_rect.w) / 2.0;
                        image_rect.y = mi.y + (mi.h - image_rect.h) / 2.0;
                        window.draw(&image_rect, Img(i));
                    }
                }
                MenuItemType::AppearingText {
                    text,
                    text_image,
                    current_text,
                    text_size,
                    text_c,
                    timer,
                } => {
                    if let Some(i) = text_image {
                        let mut image_rect = i.area();
                        image_rect.pos.x = mi.x;
                        image_rect.pos.y = mi.y;
                        window.draw(&image_rect, Img(i));
                    }
                }
                MenuItemType::InstantText {
                    text,
                    text_image,
                    text_size,
                    text_color,
                } => {
                    if let Some(i) = text_image {
                        let mut image_rect = i.area();
                        image_rect.pos.x = mi.x;
                        image_rect.pos.y = mi.y;
                        window.draw(&image_rect, Img(i));
                    }
                }
                MenuItemType::Pause { timer, length } => (),
            }
        }
        self.player_particles.draw(window, Transform::IDENTITY);
        window.draw_ex(
            &self.player,
            Col(Color::from_rgba(
                0xFF,
                0xFF,
                0xFF,
                (self.player_particles.opacity * 255.0) as u8,
            )),
            Transform::translate(-self.player.x / 2.0, -self.player.y / 2.0)
                * Transform::rotate(self.player_r as f32),
            1,
        );
        self.joining_particles.draw(window, Transform::IDENTITY);
        for expl_conv_ps in &mut self.expl_conv_p_systems {
            expl_conv_ps.draw(window, Transform::IDENTITY);
        }
        for planet in &mut self.planets {
            planet.draw(window, Transform::IDENTITY);
        }

        if let Some(i) = &mut self.i_star_actual {
            for star in &mut self.stars {
                star.draw(i, window, Transform::IDENTITY);
            }
        }

        if let Some(body) = &self.i_fish_body {
            if let Some(tail) = &self.i_fish_tail {
                for fish in &mut self.fishes {
                    fish.draw(body, tail, window, Transform::IDENTITY);
                }
            }
        }

        if let Some(sl) = &mut self.save_load_notification {
            match sl {
                SaveLoadNotification::Save { text, timer }
                | SaveLoadNotification::Load { text, timer } => {
                    if let Some(i) = text {
                        let mut c = Color::WHITE;
                        c.a = ((*timer / SL_NOTIF_TIME) as f32 * 255.0) as u8;
                        let mut image_rect = i.area_rect();
                        image_rect.x = self.camera.x + 20.0;
                        image_rect.y = self.camera.y + 20.0;
                        window.draw(&image_rect, Blended(i, c));
                    }
                }
            }
        }

        Ok(())
    }
}

fn main() {
    run::<GameState>(
        "One And All - a Ludum Dare 45 compo entry",
        Vector::new(800, 600),
        Settings::default(),
    );
}

use quicksilver::{
    geom::{Circle, Rectangle, Transform, Vector},
    graphics::{
        Background::{Col, Img},
        Color, Font, FontStyle, Image,
    },
    input::{ButtonState, Key},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    sound::Sound,
    Result,
};
use rand::prelude::*;

const WIDTH_F: f32 = 800.0;
const HEIGHT_F: f32 = 600.0;
const MUSIC2_LENGTH: f64 = 2.0 * 60.0 * 1000.0;
const TEXT_RATE: f64 = 100.0;
const PP_GEN_RATE: f64 = 75.0;

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
                h_c: Color::from_rgba(0x66, 0xFF, 0xFF, 1.0),
                c: Color::from_rgba(0x33, 0xFF, 0xFF, 1.0),
            },
            is_hover: false,
            is_focus: false,
            is_loaded: false,
        };

        Menu {
            items: vec![
                item,
                Menu::instant_text(
                    150.0,
                    50.0,
                    45.0,
                    true,
                    "One And All",
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
                    Color::from_rgba(0x33, 0x33, 0x33, 1.0),
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
                    Color::from_rgba(0x33, 0x33, 0x33, 1.0),
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
                    Color::from_rgba(0x33, 0x33, 0x33, 1.0),
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
                    Color::from_rgba(0x33, 0x33, 0x33, 1.0),
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
}

struct Particle {
    rect: Rectangle,
    velx: f32,
    vely: f32,
    velr: f32,
    r: f32,
    lifetime: f64,
    life_timer: f64,
}

struct ParticleSystem {
    particles: Vec<Particle>,
    spawn_timer: f64,
    spawn_time: f64,
    lifetime: f64,
    host_rect: Rectangle,
    direction: Vector,
}

impl ParticleSystem {
    fn new(spawn_time: f64, lifetime: f64, host_rect: Rectangle, direction: Vector) -> Self {
        Self {
            particles: Vec::new(),
            spawn_timer: 0.0,
            spawn_time,
            lifetime,
            host_rect,
            direction,
        }
    }

    fn update(&mut self, dt: f64) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].life_timer += dt;
            if self.particles[i].life_timer > self.particles[i].lifetime {
                self.particles.swap_remove(i);
            } else {
                self.particles[i].rect.pos.x += self.particles[i].velx * dt as f32;
                self.particles[i].rect.pos.y += self.particles[i].vely * dt as f32;
                self.particles[i].r += self.particles[i].velr * dt as f32;
            }
        }

        self.spawn_timer += dt;
        if self.spawn_timer > self.spawn_time {
            self.spawn_timer -= self.spawn_time;
            self.particles.push(Particle {
                rect: self.host_rect,
                velx: rand::thread_rng().gen_range(-0.2, 0.2) + self.direction.x,
                vely: rand::thread_rng().gen_range(-0.2, 0.2) + self.direction.y,
                velr: rand::thread_rng().gen_range(-0.5, 0.5),
                r: rand::thread_rng().gen_range(0.0, 90.0),
                lifetime: self.lifetime,
                life_timer: 0.0,
            });
        }
    }
}

struct GameState {
    s_boom: Asset<Sound>,
    s_get: Asset<Sound>,
    s_power_up: Asset<Sound>,
    s_tap: Asset<Sound>,
    s_speak_m: Asset<Sound>,
    s_speak_f: Asset<Sound>,
    font: Asset<Font>,
    music2: Asset<Sound>,
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
    player_opacity: f32,
}

impl State for GameState {
    fn new() -> Result<Self> {
        Ok(Self {
            s_boom: Asset::new(Sound::load("boom.mp3")),
            s_get: Asset::new(Sound::load("get.mp3")),
            s_power_up: Asset::new(Sound::load("power_up.mp3")),
            s_tap: Asset::new(Sound::load("tap.mp3")),
            s_speak_m: Asset::new(Sound::load("speak_m.mp3")),
            s_speak_f: Asset::new(Sound::load("speak_f.mp3")),
            font: Asset::new(Font::load("ClearSans-Regular.ttf")),
            music2: Asset::new(Sound::load("music2.mp3")),
            music_on: false,
            music_timer: 0.0,
            menu: Menu::start(),
            state: 0,
            state_dirty: false,
            selection_mode: true,
            current_item: None,
            current_finished: true,
            player: Rectangle::new((400.0, 300.0), (32.0, 32.0)),
            player_r: 0.0,
            player_particles: ParticleSystem::new(
                PP_GEN_RATE,
                1000.0,
                Rectangle::new((400.0, 300.0), (32.0, 32.0)),
                Vector::new(0.0, 0.0),
            ),
            player_opacity: 0.0,
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::MouseMoved(v) => {
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
                if let ButtonState::Pressed = state {
                    if self.current_finished {
                        if self.selection_mode {
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
                                        } else if idx == 6 {
                                            // miracles
                                            self.state = 4;
                                            self.state_dirty = true;
                                        } else if idx == 7 {
                                            // kindness
                                            self.state = 5;
                                            self.state_dirty = true;
                                        } else {
                                            // determination
                                            self.state = 6;
                                            self.state_dirty = true;
                                        }
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
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let dt = window.update_rate();

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
                _ => {
                    self.menu = Menu::start();
                    self.current_item = None;
                    self.selection_mode = true;
                    self.state = 0;
                    self.player_opacity = 0.0;
                }
            }
        }

        if self.player_opacity < 1.0 && self.state > 1 {
            self.player_opacity += dt as f32 / 7000.0;
            if self.player_opacity > 1.0 {
                self.player_opacity = 1.0;
            }
        }

        if self.music_on {
            self.music_timer += dt;
            if self.music_timer > MUSIC2_LENGTH {
                self.music_timer = 0.0;
                self.music2.execute(|m2| m2.play())?;
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
                        mi.is_loaded = true;
                        if i + 1 < self.menu.items.len() {
                            self.menu.items[i + 1].is_loaded = false;
                        } else {
                            self.current_finished = true;
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

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let mut rect = Rectangle::default();
        for mi in &mut self.menu.items {
            rect.pos.x = mi.x;
            rect.pos.y = mi.y;
            rect.size.x = mi.w;
            rect.size.y = mi.h;
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
                        let mut image_rect = i.area();
                        image_rect.pos.x = mi.x + (mi.w - image_rect.size.x) / 2.0;
                        image_rect.pos.y = mi.y + (mi.h - image_rect.size.y) / 2.0;
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
        for particle in &self.player_particles.particles {
            window.draw_ex(
                &particle.rect,
                Col(Color::from_rgba(
                    0xFF,
                    0xFF,
                    0xFF,
                    (1.0 - (particle.life_timer / particle.lifetime) as f32) * self.player_opacity,
                )),
                Transform::translate((-particle.rect.size.x / 2.0, -particle.rect.size.y / 2.0))
                    * Transform::rotate(particle.r),
                1,
            );
        }
        window.draw_ex(
            &self.player,
            Col(Color::from_rgba(0xFF, 0xFF, 0xFF, self.player_opacity)),
            Transform::translate((-self.player.size.x / 2.0, -self.player.size.y / 2.0))
                * Transform::rotate(self.player_r as f32),
            1,
        );
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

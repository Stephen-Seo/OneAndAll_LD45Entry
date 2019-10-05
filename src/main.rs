use quicksilver::{
    geom::{Circle, Rectangle, Vector},
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
        if x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h {
            true
        } else {
            false
        }
    }
}

struct Menu {
    items: Vec<MenuItem>,
}

impl Menu {
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

        Menu { items: vec![item] }
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
                                    _ => (),
                                }
                            }
                        } else {
                            self.state += 1;
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

        if self.state_dirty {
            self.state_dirty = false;
            match self.state {
                1 => {
                    self.menu = Menu::s_01();
                    self.current_finished = false;
                    self.selection_mode = false;
                }
                _ => {
                    self.menu = Menu::start();
                    self.current_item = None;
                    self.selection_mode = true;
                    self.state = 0;
                }
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
                                self.s_speak_f.execute(|s| {
                                    s.set_volume(0.3);
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
                }
            }
        }
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
                MenuItemType::Pause { timer, length } => (),
            }
        }
        Ok(())
    }
}

fn main() {
    run::<GameState>(
        "LudumDare45_StartWithNothing",
        Vector::new(800, 600),
        Settings::default(),
    );
}

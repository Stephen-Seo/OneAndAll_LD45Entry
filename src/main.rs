use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Background::{Col, Img}, Color, Font, FontStyle, Image},
    input::{ButtonState, Key},
    lifecycle::{run, Asset, Event, Settings, State, Window},
    sound::Sound,
    Result,
};
use rand::prelude::*;

const WIDTH_F: f32 = 800.0;
const WIDTH_H: f32 = 600.0;

enum MenuItemType {
    Button {
        text: &'static str,
        text_image: Option<Image>,
        text_c: Color,
        h_c: Color,
        c: Color,
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
}

struct GameState {
    s_boom: Asset<Sound>,
    s_get: Asset<Sound>,
    s_power_up: Asset<Sound>,
    s_tap: Asset<Sound>,
    s_speak_m: Asset<Sound>,
    s_speak_f: Asset<Sound>,
    font: Asset<Font>,
    timer: f64,
    menu: Menu,
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
            timer: 0.0,
            menu: Menu::start(),
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::MouseMoved(v) =>
                for mi in &mut self.menu.items {
                    if mi.is_inside(v.x, v.y) {
                        mi.is_hover = true;
                    } else {
                        mi.is_hover = false;
                    }
                },
            _ => (),
        }
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let dt = window.update_rate();
        self.timer += dt;
        for mi in &mut self.menu.items {
            if !mi.is_loaded {
                match &mut mi.item_type {
                    MenuItemType::Button { text, text_image, text_c, h_c, c } => {
                        self.font.execute(|font| {
                            let style = FontStyle::new(42.0, *text_c);
                            *text_image = Some(font.render(text, &style)?);
                            Ok(())
                        })?;
                    }
                }
                mi.is_loaded = true;
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
                MenuItemType::Button{ text, text_image, text_c, h_c, c } => {
                    if mi.is_hover {
                        window.draw(&rect, Col(*h_c));
                    } else {
                        window.draw(&rect, Col(*c));
                    }
                    if let Some(i) = text_image {
                        let mut image_rect = i.area();
                        image_rect.pos.x = mi.x + (mi.w - image_rect.size.x) / 2.0;
                        image_rect.pos.y = mi.y + (mi.h - image_rect.size.y) / 2.0;
                        window.draw(
                            &image_rect,
                            Img(i));
                    }
                },
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

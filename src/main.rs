use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{Asset, run, Event, Settings, State, Window},
    Result,
    sound::Sound,
};
use rand::prelude::*;

struct GameState {
    s_boom: Asset<Sound>,
    s_get: Asset<Sound>,
    s_power_up: Asset<Sound>,
    s_tap: Asset<Sound>,
    s_speak_m: Asset<Sound>,
    s_speak_f: Asset<Sound>,
    timer: f64,
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
            timer: 0.0,
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let dt = window.update_rate();
        self.timer += dt;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
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

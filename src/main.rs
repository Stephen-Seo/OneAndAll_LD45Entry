use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};
use rand::prelude::*;

struct GameState {}

impl State for GameState {
    fn new() -> Result<Self> {
        Ok(Self {})
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
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

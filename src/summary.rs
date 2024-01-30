use crate::parser::Event;

#[derive(Default, Debug)]
pub struct Summary;

impl Summary {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&mut self, event: Event) {
        match dbg!(event) {
            Event::Kill {
                killer: _,
                victim: _,
                weapon_used: _,
            } => {},
            Event::MatchEnded => {},
        }
    }

    pub fn output(&self) {
        dbg!(&self);
    }
}

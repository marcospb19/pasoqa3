use std::collections::HashMap;

use super::Event;

type Score = i64;

pub struct Summary;

#[derive(Default, Debug)]
pub struct SummaryProcessor {
    player_score: HashMap<u16, Score>,
}

impl SummaryProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&mut self, event: Event) -> Option<Summary> {
        match dbg!(event) {
            Event::Kill {
                killer: _,
                victim: _,
                weapon_used: _,
            } => {},
            Event::ResetMatch => {},
            Event::PlayerJoined { .. } => {},
            Event::PlayerLeft { .. } => {},
            Event::PlayerNameUpdate { .. } => {},
        }

        None
    }

    pub fn output(&self) {
        dbg!(&self);
    }
}

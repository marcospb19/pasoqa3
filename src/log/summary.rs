use std::{collections::HashMap, mem};

use counter::Counter;

use super::{Event, PlayerId, WORLD_ID};

type PlayerScore = i64;

#[derive(Default, Debug)]
pub struct SummaryProcessor {
    match_number: u32,
    total_kills: u64,
    scoreboard: Counter<PlayerId, PlayerScore>,
    player_names: HashMap<PlayerId, String>,
}

impl SummaryProcessor {
    pub fn new() -> Self {
        Self {
            match_number: 1,
            ..Self::default()
        }
    }

    pub fn process(&mut self, event: Event) {
        match event {
            Event::Kill {
                killer,
                victim,
                weapon_used,
            } => {
                self.total_kills += 1;
                if killer == WORLD_ID {
                    self.scoreboard[&victim] -= 1;
                } else {
                    self.scoreboard[&killer] += 1;
                }
            },
            Event::ResetMatch => {
                let new_self = Self {
                    match_number: self.match_number + 1,
                    ..Self::default()
                };

                let previous_self = mem::replace(self, new_self);

                previous_self.output();
            },
            Event::PlayerJoined { id, name } => {
                self.player_names.insert(id, name);
            },
            Event::PlayerLeft { id } => {
                self.player_names.remove(&id);
                self.scoreboard.remove(&id);
            },
            Event::PlayerNameUpdate { id, new_name } => {
                self.player_names.insert(id, new_name);
            },
        }
    }

    pub fn output(self) {
        let match_name = format!("game_{}", self.match_number);
        let player_names: Vec<&str> =
            self.player_names.values().map(String::as_str).collect();
        let kills_map: Vec<(&str, PlayerScore)> = self
            .scoreboard
            .into_iter()
            .map(|(id, score)| (self.player_names[&id].as_str(), score))
            .collect();

        let json = serde_json::json!({
            match_name: {
                "total_kills": self.total_kills,
                "players": player_names,
                "kills": kills_map,
            }
        });

        let output = serialize_json(&json);

        println!("{output}");
    }
}

fn serialize_json(json: &serde_json::Value) -> String {
    serde_json::to_string_pretty(json)
        .expect("Serializing a JSON from `serde_json` is infallible")
}

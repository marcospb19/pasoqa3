//! Report/summary builder for `q3` matches.

use std::{collections::BTreeMap, mem};

use atty::Stream;
use counter::Counter;
use highlighting::highlight_json;

use super::{Event, PlayerId, WORLD_ID};

type PlayerScore = i32;

/// Processes events to build and output summaries.
///
/// Built match summaries are outputted as highlighted `JSON`.
#[derive(Default, Debug)]
pub struct SummaryProcessor {
    match_number: u32,
    total_kills: u32,
    death_cause_count: Counter<String, u32>,
    scoreboard: Counter<PlayerId, PlayerScore>,
    player_names: BTreeMap<PlayerId, String>,
    game_to_show: Option<u32>,
}

impl SummaryProcessor {
    pub fn new(game_to_show: Option<u32>) -> Self {
        Self {
            game_to_show,
            match_number: 0,
            ..Self::default()
        }
    }

    pub fn process(&mut self, event: Event) {
        match event {
            Event::Kill {
                killer,
                victim,
                death_cause,
            } => {
                self.total_kills += 1;
                self.death_cause_count[&death_cause] += 1;

                if killer == WORLD_ID {
                    self.scoreboard[&victim] -= 1;
                } else {
                    self.scoreboard[&killer] += 1;
                }
            },
            Event::ResetMatch => {
                let new_self = Self {
                    game_to_show: self.game_to_show,
                    match_number: self.match_number + 1,
                    ..Self::default()
                };

                let previous_self = mem::replace(self, new_self);

                previous_self.output();
            },
            Event::PlayerJoined { id, name } => {
                self.scoreboard[&id] = 0;
                self.player_names.insert(id, name);
            },
            Event::PlayerLeft { id } => {
                self.scoreboard.remove(&id);
                self.player_names.remove(&id);
            },
            Event::PlayerNameUpdate { id, new_name } => {
                self.player_names.insert(id, new_name);
            },
        }
    }

    pub fn output(self) {
        let should_skip = self.match_number == 0
            || self
                .game_to_show
                .is_some_and(|game| self.match_number != game);

        if should_skip {
            return;
        }

        let json = {
            let match_name = format!("game_{}", self.match_number);

            let player_names: Vec<&str> =
                self.player_names.values().map(String::as_str).collect();

            let scoreboard_map: BTreeMap<&str, PlayerScore> = self
                .scoreboard
                .into_iter()
                .map(|(id, score)| (self.player_names[&id].as_str(), score))
                .collect();

            serde_json::json!({
                match_name: {
                    "total_kills": self.total_kills,
                    "players": player_names,
                    "scoreboard": scoreboard_map,
                    "death_causes": *self.death_cause_count,
                }
            })
        };

        let json_text = serialize_json(&json);

        let json_text = if atty::is(Stream::Stdout) {
            highlight_json(&json_text)
        } else {
            json_text
        };

        println!("{json_text:}");
    }
}

fn serialize_json(json: &serde_json::Value) -> String {
    serde_json::to_string_pretty(json)
        .expect("Serializing a JSON from `serde_json` is infallible")
}

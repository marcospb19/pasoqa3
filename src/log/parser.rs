//! `LogMessageParser` parses log messages into higher-level events.

use std::collections::{hash_map::Entry, HashMap};

use crate::{
    err,
    log::{Event, PlayerId, PlayerName},
    Result, WrapErr, WrapNone,
};

mod delimited;
mod id_parsing;

use delimited::extract_text_delimited_by;
use id_parsing::{parse_id, IdSequenceParser};

/// Parses log messages into [`Event`]s.
///
/// This parser is stateful.
///
/// This parser skips unrelated messages.
///
/// Check [`Event`] docs.
pub struct LogMessageParser {
    state: LogState,
}

impl LogMessageParser {
    pub fn new() -> Self {
        Self {
            state: LogState::default(),
        }
    }

    pub fn parse_line(&mut self, line: &str) -> Result<Option<Event>> {
        let line = Self::trim_timestamp(line);

        let (prefix, parser_method) = {
            type ParserMethod =
                fn(&mut LogMessageParser, &str) -> Result<Option<Event>>;
            type DispatchMapItem = (&'static str, ParserMethod);

            // Parser method dispatch map, checks the event variant by its prefix
            let method_dispatch_map: &[DispatchMapItem] = &[
                ("ClientBegin:", Self::parse_client_begin),
                ("ClientConnect:", Self::parse_client_connect),
                ("ClientDisconnect:", Self::parse_client_disconnect),
                ("ClientUserinfoChanged:", Self::parse_client_info_changed),
                ("Kill:", Self::parse_kill),
                ("InitGame:", Self::reset_match),
            ];

            let Some(matched_method) = method_dispatch_map
                .iter()
                .find(|(prefix, _)| line.starts_with(prefix))
            else {
                // If there are no matches, return `None` (no event for this line)
                return Ok(None);
            };

            matched_method
        };

        // Split unwrap safety:
        //   If `line.starts_with(prefix)`, then `line.len()` >= `prefix.len()`
        let (_, remainder) = line.split_at(prefix.len());
        let line_contents = remainder.trim();

        // Call dispatched parser method to parse message
        parser_method(self, line_contents)
            .wrap_err_with(|| format!("Log message: '{line_contents}'"))
            .wrap_err("Log file is corrupted or malformed")
            .wrap_err_with(|| err!("Couldn't parse message of type '{prefix}'"))
    }

    fn parse_kill(&mut self, kill_details: &str) -> Result<Option<Event>> {
        let mut id_iter = IdSequenceParser::new(kill_details);

        let mut next_id = || {
            id_iter
                .next()
                .wrap_none("Kill messages expected three integer IDs")
        };

        let death_cause = {
            let death_cause_position = kill_details
                .rfind("MOD_")
                .wrap_none("Death cause missing from Kill message")?;

            let (_left, right) = kill_details.split_at(death_cause_position);
            right.to_owned()
        };

        Ok(Some(Event::Kill {
            killer: next_id()?,
            victim: next_id()?,
            death_cause,
        }))
    }

    fn reset_match(&mut self, _text: &str) -> Result<Option<Event>> {
        self.state.clear();
        Ok(Some(Event::ResetMatch))
    }

    fn parse_client_connect(&mut self, id: &str) -> Result<Option<Event>> {
        let id: PlayerId = parse_id(id).wrap_none(
            "Expected a client integer ID from 'ClientConnect' message",
        )?;

        let entry = self.state.connecting_players.entry(id);

        match entry {
            Entry::Occupied(_) => {
                // Reasons why it might be occupied:
                //
                // 1. Client connect message was duplicated.
                // 2. The last client connection didn't conclude as expected.
                //
                // In both cases, ignoring this is the best option to maintain
                // the semantics consistent, even if the files are "corrupted".
            },
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(None);
            },
        }

        Ok(None)
    }
    fn parse_client_disconnect(&mut self, id: &str) -> Result<Option<Event>> {
        let id: PlayerId = parse_id(id).wrap_none(
            "Expected a client integer ID from 'ClientDisconnect' message",
        )?;

        self.state.connecting_players.remove(&id);
        self.state.connected_players.remove(&id);

        Ok(Some(Event::PlayerLeft { id }))
    }

    fn parse_client_begin(&mut self, id: &str) -> Result<Option<Event>> {
        let id = parse_id(id).wrap_none(
            "Expected a client integer ID from ClientBegin message",
        )?;

        let connecting_player =
            self.state.connecting_players.remove(&id).flatten();

        let Some(name) = connecting_player else {
            // If this connecting player is not present, we got a bad
            // `ClientBegin` message, two cases:
            //
            // 1. This message was accidentally duplicated.
            // 2. This just came out of nowhere.
            //
            // If it's the first, ignore it, otherwise, throw an error.
            if self.state.connected_players.contains_key(&id) {
                // Case 1: Ignored.
                return Ok(None);
            } else {
                // Case 2: Throw the error.
                return Err(err!("Player with ID {id} joined without a name"));
            }
        };

        self.state.connected_players.insert(id, name.clone());

        Ok(Some(Event::PlayerJoined { id, name }))
    }

    fn parse_client_info_changed(
        &mut self,
        update_info: &str,
    ) -> Result<Option<Event>> {
        let id = parse_id(update_info).wrap_none(
            "Expected a client integer ID from ClientUserinfoChanged message",
        )?;
        let name = extract_text_delimited_by(update_info, r"n\", r"\t")
            .wrap_none(r"Expected player name delimited by 'n\' and '\t'")?
            .to_owned();

        // There are two cases where this message should appear:
        //
        // 1. A client is connecting, and this sets up their data.
        // 2. A connected client updated their data.
        //
        // We decide which one it is based on the stored state.
        if let Some(connecting_client) =
            self.state.connecting_players.get_mut(&id)
        {
            connecting_client.replace(name);
            Ok(None)
        } else {
            Ok(Some(Event::PlayerNameUpdate { id, new_name: name }))
        }
    }

    /// Trim off the timestamp of a log line, return the remaining contents.
    fn trim_timestamp(line: &str) -> &str {
        let is_part_of_timestamp =
            |ch: char| ch.is_whitespace() || ch.is_numeric() || ch == ':';

        let is_not_timestamp = |ch| !is_part_of_timestamp(ch);

        let contents_start = line.find(is_not_timestamp).unwrap_or(0);

        &line[contents_start..]
    }
}

#[derive(Default)]
struct LogState {
    connected_players: HashMap<PlayerId, PlayerName>,
    connecting_players: HashMap<PlayerId, Option<PlayerName>>,
}

impl LogState {
    fn clear(&mut self) {
        self.connected_players.clear();
        self.connecting_players.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_line_timestamp() {
        let text = " 25:04 ClientConnect: 2";
        let result = LogMessageParser::trim_timestamp(text);
        let expected = "ClientConnect: 2";

        assert_eq!(expected, result);

        let text = "  0:00 --------";
        let result = LogMessageParser::trim_timestamp(text);
        let expected = "--------";

        assert_eq!(expected, result);

        let text = "981:36 ClientUserinfoChanged: 6 blablabla 0";
        let result = LogMessageParser::trim_timestamp(text);
        let expected = "ClientUserinfoChanged: 6 blablabla 0";

        assert_eq!(expected, result);
    }
}

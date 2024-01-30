use core::array;
use std::collections::{hash_map::Entry, HashMap};

type WeaponId = u16;
type PlayerId = u16;
type PlayerInfo = String;

/// An event emmited by the log parser.
///
/// Consider each line in a log file to be a "message", then, an event is a
/// "higher-level" representation of what messages tell you.
///
/// Ways in which messages differ from events:
///
/// 1. Some events (e.g. [`Event::PlayerConnected`]) are represented by
/// multiple messages.
/// 2. Messages can have different meanings in different contexts, each event
/// variant has an unique (and straight-forward) meaning.
#[derive(Debug)]
pub enum Event {
    /// A player was killed by something or someone.
    ///
    /// The killer might be another player or "the world".
    Kill {
        killer: PlayerId,
        victim: PlayerId,
        weapon_used: WeaponId,
    },
    /// The current match ended.
    ///
    /// At match end, all joined players should be considered disconnected [TODO].
    ///
    /// Note that there is no `Event::MatchStarted`, all events succeeding the
    /// match end should be considered to be in from a new match.
    MatchEnded,
}

/// Parses log messages into [`Event`]s.
///
/// This parser is stateful.
///
/// This parser skips unrelated messages.
pub struct LogMessageParser {
    state: LogState,
}

impl LogMessageParser {
    pub fn new() -> Self {
        Self {
            state: LogState::default(),
        }
    }

    pub fn parse_line(&mut self, line: &str) -> Option<Event> {
        let line = Self::trim_timestamp(line);

        let (prefix, parser_method) = {
            type ParserMethod =
                fn(&mut LogMessageParser, &str) -> Option<Event>;
            type DispatchMapItem = (&'static str, ParserMethod);

            // Parser method dispatch map, checks the event variant by its prefix
            let method_dispatch_map: &[DispatchMapItem] = &[
                ("Kill:", Self::parse_kill),
                ("ShutdownGame:", Self::parse_shutdown_game),
                // ("ClientBegin:", Self::parse_client_begin),
                ("ClientConnect:", Self::parse_client_connect),
                // ("ClientDisconnect:", Self::parse_client_disconnect),
                ("ClientUserinfoChanged:", Self::parse_client_info_changed),
            ];

            method_dispatch_map
                .iter()
                .find(|(prefix, _)| line.starts_with(prefix))?
        };

        // Index unwrap safety:
        //   If `line.starts_with(prefix)`, then `line.len()` >= `prefix.len()`
        let line_without_prefix = line[prefix.len()..].trim();

        parser_method(self, line_without_prefix)
    }

    fn parse_kill(&mut self, kill_details: &str) -> Option<Event> {
        let mut id_iter = kill_details.split(|ch| ch == ' ' || ch == ':');

        let [killer, victim, weapon_used] =
            array::from_fn(|_| id_iter.next().unwrap().parse().unwrap());

        Some(Event::Kill {
            killer,
            victim,
            weapon_used,
        })
    }

    fn parse_shutdown_game(&mut self, _details: &str) -> Option<Event> {
        self.state.clear();
        Some(Event::MatchEnded)
    }

    fn parse_client_connect(
        &mut self,
        connection_details: &str,
    ) -> Option<Event> {
        let mut id_iter = connection_details.split(|ch| ch == ' ' || ch == ':');

        let client_id = id_iter.next().unwrap().parse().unwrap();

        let entry = self.state.connecting_players.entry(client_id);

        match entry {
            Entry::Occupied(_) => {
                // Ignore it, this should be unlikely
            },
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(None);
            },
        }

        None
    }

    fn parse_client_info_changed(
        &mut self,
        _new_info_details: &str,
    ) -> Option<Event> {
        todo!()
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
    connecting_players: HashMap<PlayerId, Option<PlayerInfo>>,
}

impl LogState {
    fn clear(&mut self) {
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

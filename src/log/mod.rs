mod parser;
mod summary;

pub use parser::LogMessageParser;
pub use summary::SummaryProcessor;

type Id = u16;
type PlayerId = Id;
type WeaponId = Id;
type PlayerName = String;

const WORLD_ID: Id = 1022;

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
    ResetMatch,
    /// A player just joined the current match.
    PlayerJoined { id: PlayerId, name: String },
    /// A player just left the current match.
    PlayerLeft { id: PlayerId },
    /// A connected player changed its name.
    PlayerNameUpdate { id: PlayerId, new_name: String },
}

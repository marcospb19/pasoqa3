# `pasoqa3`

`pasoqa3` is an acronym for "***P***arser ***A***nd ***S***ummarizer ***O***f ***Q***uake ***A***rena ***3*** (log files)".

Check [`DECISIONS.md`] for an explanation of why I took some of the decisions I did.

## Installation

In Unix, with the `Rust` toolchain installed, clone the repository and install `pasoqa3` using `cargo`:

```sh
git clone https://github.com/marcospb19/pasoqa3
cargo install --path pasoqa3
```

You can see the binary at `~/.cargo/bin/pasoqa3`, make sure `~/.cargo/bin` is listed in your shell's `$PATH` variable to access it from anywhere.

## Usage examples

```sh
# Summarize all matches in `file.log`
pasoqa3 file.log

# Show summary of the fourth match
pasoqa3 q3.log --game 4
```

```json
{
  "game_4": {
    "total_kills": 4,
    "players": [
      "Isgalamido",
      "Zeh",
      "Dono da Bola"
    ],
    "scoreboard": {
      "Zeh": -2,
      "Dono da Bola": -1,
      "Isgalamido": 1
    },
    "death_causes": {
      "MOD_FALLING": 1,
      "MOD_ROCKET": 1,
      "MOD_TRIGGER_HURT": 2
    }
  }
}
```

## How it works

All matches of **Quake 3 Arena** have their _kill feed_ written into a log file.

A single log file might contain zero or multiple matches written to it.

`pasoqa3` reads a log file, tries parsing it, and outputs a summary of the _kill feed_ for each match.

## Trivia

The name _"pasoqa"_ is a playful reference to _"paçoca"_, [a Brazilian candy] I used to love as a child.

<img src="https://github.com/marcospb19/pasoqa3/assets/38900226/49f30f7e-2830-41e3-b87e-4dcb170888c2" width="300"/>


[a Brazilian candy]: https://en.wikipedia.org/wiki/Pa%C3%A7oca
[`DECISIONS.md`]: ./DECISIONS.md

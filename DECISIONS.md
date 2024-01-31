Here I document decisions I took in this project, and the rationale behind them.

Alias: `q3` = Quake 3 Arena.

# Summary

### What happens to disconnected players?

Disconnected players aren't shown in the summary.

There is no unique ID for each player, so, if we show disconnected players and a player reconnects, they might get duplicated in the scoreboard.

# Performance and optimization

### Why didn't you optimized the code?

`pasoqa3` runs instantly in `--release` for big logs.

I focused on code simplicity and clarity over premature optimizing.

# Parser

### Why not a parsing library?

I didn't use a parser library because it would be counter-productive and add unnecessary complexity.

`q3` logs are simple to parse, and we don't need to fully parse them.

### Why not detect weapons by their ID?

There are conditional compilation directives in `q3`'s source code which cause IDs to change between compiled versions.

With that in mind, I chose to grab the weapon names instead.

### Why not `serde` for the parser?

Within the semantics of `serde::Deserialize`, you can either succeed or fail at parsing a document.

`pasoqa3` ignores most messages and performs "partial parsing", which doesn't match `serde::Deserialize` semantics.

### Decoupling

Summary building is separated from the parser.

The `Event` enum can be thought off as the "Intermediate Representation" or "AST" of traditional language parsing projects.

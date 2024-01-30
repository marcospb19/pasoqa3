Here I document decisions I took in this project, and the rationale behind them.

## Why not `serde` for the parser?

Within the semantics of `serde::Deserialize`, you can either succeed or fail at parsing a document.

It makes sense for formats with formal specification (e.g. `JSON` and `TOML`), but q3 logs don't follow a public spec and might be malformed.

With that in mind, the parser struct shouldn't stop at the first failure, but instead, allow the user to ignore it.

Doing partial deserialization with `serde::Deserialize` leads to a weird API.

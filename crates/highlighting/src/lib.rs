//! A JSON highlighter for ANSI-capable terminals.

use std::fmt::Write;

use owo_colors::{OwoColorize, Rgb};
use syntect::{
    easy::HighlightLines,
    highlighting::{Color, Style, ThemeSet},
    parsing::SyntaxSet,
};

/// Highlights JSON using ANSI escape sequences colors.
///
/// # Panics
///
/// This function might panic if the given `json` is malformed.
pub fn highlight_json(json: &str) -> String {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    let syntax = syntax_set
        .syntaxes()
        .iter()
        .find(|syntax| syntax.name == "JSON")
        .expect("JSON is included with the `default-syntaxes` feature");

    let theme = theme_set.themes.get("base16-eighties.dark").expect(
        "base16-eighties.dark is included with the `default-themes` feature",
    );

    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut result = String::new();

    for line in json.lines() {
        let line_pieces = highlighter
            .highlight_line(line, &syntax_set)
            .expect("Malformed JSON");

        // Write unwrap safety:
        //   Writing to an `String` always returns `Ok(())`
        for (style, text) in line_pieces {
            let Style { foreground, .. } = style;
            let Color { r, g, b, .. } = foreground;
            let color = Rgb(r, g, b);
            write!(result, "{}", text.color(color)).unwrap();
        }
        writeln!(result).unwrap();
    }

    result
}

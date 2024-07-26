use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;
use syntect::easy::HighlightFile;
use std::io::BufRead;
use std::fs::File;

// pub fn highlight(path: PathBuf) -> String {
//     let ss = SyntaxSet::load_defaults_newlines();
//     let ts = ThemeSet::load_defaults();

//     let mut highlighter = HighlightFile::new(path, &ss, &ts.themes["base16-ocean.dark"]).unwrap();
// }

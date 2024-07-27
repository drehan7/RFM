use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::easy::HighlightLines;
use std::path::PathBuf;

pub struct Syntax {
    pub s: SyntaxSet,
    pub t: ThemeSet,
}

pub fn init_syntax() -> Syntax {
    Syntax {
        s: SyntaxSet::load_defaults_newlines(),
        t: ThemeSet::load_defaults(),
    }
}
   
pub type HighlightedFile<'a> = Vec<Vec<(Style, &'a str)>>;

pub fn highlight_file<'a>
    (syn: &'a Syntax, file_path: &'a PathBuf, lines: &'a String) -> HighlightedFile<'a>
{
    // for t in syn.t.themes.keys() {
    //     println!("Theme: {}", t);
    // }

    let ext = match file_path.extension() {
        Some(osstr_ext) => {
            match osstr_ext.to_str() {
                Some(str_ext) => str_ext,
                None => ""
            }
        },
        None => ""
    };

    let syntax = match syn.s.find_syntax_by_extension(ext) {
        Some(syn) => syn,
        None => {
            syn.s.find_syntax_plain_text()
        }
    };

    // Todo: make configurable
    let theme = &syn.t.themes["base16-ocean.dark"];

    let mut h = HighlightLines::new(&syntax, theme);
    let mut ret = vec![];

    for l in lines.split('\n') {
        let r = h.highlight_line(l, &syn.s);
        ret.push(r.unwrap());
    }


    return ret;
}


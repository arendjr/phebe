use lazy_static::lazy_static;
use std::fmt;
use std::fs;
use typed_html::dom::DOMTree;

use super::color_scheme::{generate_css, PreferredColorScheme};

const CODE_MARKER: &str = "<code class=\"language-";

lazy_static! {
    static ref MAIN_CSS: String = fs::read_to_string("main.css").expect("Could not read main CSS");
    static ref FONT_CSS: String = fs::read_to_string("fonts.css").expect("Could not read font CSS");
    static ref PRISM_CSS: String =
        fs::read_to_string("prism.css").expect("Could not read prism CSS");
}

pub struct Document {
    body: DOMTree<String>,
    color_scheme: PreferredColorScheme,
}

impl Document {
    pub fn new(body: DOMTree<String>, color_scheme: PreferredColorScheme) -> Document {
        Document { body, color_scheme }
    }
}

impl<'a> fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let body = self.body.to_string();

        let has_code = body.contains(CODE_MARKER);

        let head = format!(
            "<head><title>Arend van Beelen jr.</title><meta name=\"author\" content=\"Arend van Beelen jr.\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><style>{} {} {} {}</style><script defer src=\"/main.js\" type=\"module\"></script></head>",
            *FONT_CSS,
            *MAIN_CSS,
            generate_css(self.color_scheme),
            if has_code { PRISM_CSS.clone() } else { "".to_owned() }
        );

        let code = if has_code {
            "<script src=\"/prism.js\"></script>"
        } else {
            ""
        };

        write!(f, "<!DOCTYPE><html>{}{}{}</html>", head, body, code)
    }
}

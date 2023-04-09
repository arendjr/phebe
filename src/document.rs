use std::fmt;

use super::color_scheme::{generate_css, PreferredColorScheme};

const CODE_MARKER: &str = "<code class=\"language-";

const MAIN_CSS: &str = include_str!("../statics/main.css");
const FONTS_CSS: &str = include_str!("../statics/fonts.css");

pub struct Document {
    body: String,
    color_scheme: PreferredColorScheme,
}

impl Document {
    pub fn new(body: String, color_scheme: PreferredColorScheme) -> Document {
        Document { body, color_scheme }
    }
}

impl<'a> fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let body = self.body.to_string();

        let has_code = body.contains(CODE_MARKER);

        let head = format!(
            "<head>\
            <title>Arend van Beelen jr.</title>\
            <meta name=\"author\" content=\"Arend van Beelen jr.\">\
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\
            <style>{FONTS_CSS} {MAIN_CSS} {}</style>{}\
            <script defer src=\"/main.js\" type=\"module\"></script>\
            </head>",
            generate_css(self.color_scheme),
            if has_code {
                "<link href=\"/prism.css\" rel=\"stylesheet\" type=\"text/css\">"
            } else {
                ""
            }
        );

        let code = if has_code {
            "<script src=\"/prism.js\"></script>"
        } else {
            ""
        };

        write!(f, "<!DOCTYPE html><html>{head}{body}{code}</html>")
    }
}

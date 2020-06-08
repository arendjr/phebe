use lazy_static::lazy_static;
use std::fmt;
use std::fs;
use typed_html::dom::VNode;
use typed_html::OutputType;
use v_htmlescape::escape;

use super::color_scheme::{generate_css, PreferredColorScheme};

lazy_static! {
    static ref MAIN_CSS: String = fs::read_to_string("main.css").expect("Could not read main CSS");
    static ref NOTO_SANS_JP_CSS: String =
        fs::read_to_string("noto-sans-jp.css").expect("Could not read font CSS");
}

pub struct Document<'a> {
    body: VNode<'a, String>,
    color_scheme: PreferredColorScheme,
}

impl<'a> Document<'a> {
    pub fn new(body: VNode<'a, String>, color_scheme: PreferredColorScheme) -> Document<'a> {
        Document {
            body: body,
            color_scheme: color_scheme,
        }
    }
}

impl<'a> fmt::Display for Document<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let head = format!(
            "<head><title>Arend van Beelen jr.</title><meta name=\"author\" content=\"Arend van Beelen jr.\"><style>{} {} {}</style><script defer src=\"/main.js\" type=\"module\"></script></head>",
            *NOTO_SANS_JP_CSS,
            *MAIN_CSS,
            generate_css(self.color_scheme)
        );

        write!(
            f,
            "<!DOCTYPE><html>{}{}</html>",
            head,
            vnode_to_string(&self.body)
        )
    }
}

fn vnode_to_string<T: OutputType>(node: &VNode<T>) -> String {
    match node {
        VNode::Element(element) => format!(
            "<{}{}>{}</{}>",
            element.name,
            element
                .attributes
                .iter()
                .fold(String::new(), |mut result, (key, value)| -> String {
                    result.push_str(&format!(" {}=\"{}\"", key, escape(value)));
                    result
                }),
            element
                .children
                .iter()
                .fold(String::new(), |mut result, node| -> String {
                    result.push_str(&vnode_to_string(node));
                    result
                }),
            element.name
        ),
        VNode::Text(text) => escape(text).to_string(),
        VNode::UnsafeText(text) => text.to_string(),
    }
}

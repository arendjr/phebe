mod color_scheme;
mod document;

use bytes::Bytes;
use color_scheme::PreferredColorScheme;
use const_format::formatcp;
use document::Document;
use hyper::header::{HeaderMap, HeaderValue};
use hyper::http::response::Builder as ResponseBuilder;
use hyper::http::{Error, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

const PORT: u16 = 3000;

#[derive(Serialize)]
struct JsonPage {
    content: String,
}

struct ArticleDef {
    title: &'static str,
    href: &'static str,
    content: Option<&'static str>,
}

struct PageDef {
    page: &'static str,
    href: &'static str,
    content: String,
}

static APPLICATION_JAVASCRIPT: Lazy<HeaderValue> =
    Lazy::new(|| HeaderValue::from_static("application/javascript"));
static APPLICATION_JSON: Lazy<HeaderValue> =
    Lazy::new(|| HeaderValue::from_static("application/json"));
static TEXT_CSS: Lazy<HeaderValue> = Lazy::new(|| HeaderValue::from_static("text/css"));
static TEXT_HTML: Lazy<HeaderValue> =
    Lazy::new(|| HeaderValue::from_static("text/html; charset=UTF-8"));

const MAIN_JS: &[u8] = include_bytes!("../statics/main.js");
const PRISM_JS: &[u8] = include_bytes!("../statics/prism.js");
const PRISM_CSS: &[u8] = include_bytes!("../statics/prism.css");

static PAGES: Lazy<HashMap<&'static str, Bytes>> =
    Lazy::new(|| generate_static_pages(PreferredColorScheme::Unspecified));
static DARK_PAGES: Lazy<HashMap<&'static str, Bytes>> =
    Lazy::new(|| generate_static_pages(PreferredColorScheme::Dark));
static LIGHT_PAGES: Lazy<HashMap<&'static str, Bytes>> =
    Lazy::new(|| generate_static_pages(PreferredColorScheme::Light));
static JSON_PAGES: Lazy<HashMap<&'static str, Bytes>> = Lazy::new(generate_json_pages);

const THEME_SELECTOR: &str = "<div class=\"theme-selector\">
    <a class=\"dark\" href=\"?preferred_color_scheme=dark\">Dark theme</a>
    <a class=\"light\" href=\"?preferred_color_scheme=light\">Light theme</a>
</div>";

const INDEX_CONTENT: &str = include_str!("../pages/index.html");
const PEOPLE_CONTENT: &str = include_str!("../pages/people.html");
const PROJECTS_CONTENT: &str = include_str!("../pages/projects.html");

macro_rules! article_def {
    ( $title: literal, $href: literal, $path: literal ) => {
        ArticleDef {
            title: $title,
            href: $href,
            content: Some(wrap_content!(formatcp!(
                "<h1>{}</h1>{}",
                $title,
                include_str!($path)
            ))),
        }
    };
}

macro_rules! external_article_def {
    ( $title: literal, $href: literal ) => {
        ArticleDef {
            title: $title,
            href: $href,
            content: None,
        }
    };
}

macro_rules! wrap_content {
    ( $html: expr ) => {
        formatcp!("<div class=\"content\">{}</div>", $html)
    };
}

const ARTICLE_DEFS: &[ArticleDef] = &[
    article_def!(
        "Why I'm excited for Biome's type inference",
        "/2024/01/why-im-excited-for-biomes-type-inference",
        "../articles/why-im-excited-for-biomes-type-inference.html"
    ),
    article_def!(
        "MVP: The Most Valuable Programmer",
        "/2023/04/mvp-the-most-valuable-programmer",
        "../articles/mvp-the-most-valuable-programmer.html"
    ),
    external_article_def!(
        "Why we at Fiberplane use Operational Transformation instead of CRDT (Fiberplane)",
        "https://fiberplane.com/blog/why-we-at-fiberplane-use-operational-transformation-instead-of-crdt"
    ),
    external_article_def!(
        "Creating a Rich Text Editor using Rust and React (Fiberplane)",
        "https://fiberplane.com/blog/creating-a-rich-text-editor-using-rust-and-react"
    ),
    external_article_def!(
        "Writing Redux reducers in Rust (Fiberplane)",
        "https://fiberplane.com/blog/writing-redux-reducers-in-rust"
    ),
    external_article_def!(
        "Announcing fp-bindgen (Fiberplane)",
        "https://fiberplane.com/blog/announcing-fp-bindgen/"
    ),
    article_def!(
        "JavaScript Pipe Operator Proposal: A Battle of Perspectives",
        "/2021/09/js-pipe-proposal-battle-of-perspectives.html",
        "../articles/js-pipe-proposal-battle-of-perspectives.html"
    ),
    article_def!(
        "Advanced Redux: How to create multiple instances of a state slice",
        "/2021/01/how-to-multiple-redux-slice-instances.html",
        "../articles/how-to-multiple-redux-slice-instances.html"
    ),
    article_def!(
        "How I made text-clipper the fastest HTML clipping library",
        "/2016/09/how-i-made-text-clipper-fastest-html.html",
        "../articles/how-i-made-text-clipper-fastest-html.html"
    ),
    article_def!(
        "Selectivity v3 adds React support",
        "/2016/08/selectivity-v3-adds-react-support.html",
        "../articles/selectivity-v3-adds-react-support.html"
    ),
    article_def!(
        "Selectivity v1.1.0 improves keyboard support, adds Ruby Gem",
        "/2015/05/selectivityjs-v110-improves-keyboard.html",
        "../articles/selectivityjs-v110-improves-keyboard.html"
    ),
    article_def!(
        "Selectivity v1.0 released",
        "/2015/03/select3-v10-released.html",
        "../articles/select3-v10-released.html"
    ),
    article_def!(
        "Creating submenus with Selectivity.js",
        "/2015/02/creating-submenus-with-select3.html",
        "../articles/creating-submenus-with-select3.html"
    ),
];

fn get_page_defs() -> Vec<PageDef> {
    let mut pages = vec![
        PageDef {
            page: "me",
            href: "/",
            content: wrap_content!(INDEX_CONTENT).to_owned(),
        },
        PageDef {
            page: "people",
            href: "/people",
            content: wrap_content!(PEOPLE_CONTENT).to_owned(),
        },
        PageDef {
            page: "projects",
            href: "/projects",
            content: wrap_content!(PROJECTS_CONTENT).to_owned(),
        },
        PageDef {
            page: "articles",
            href: "/articles",
            content: generate_articles_content(),
        },
    ];

    for article_def in ARTICLE_DEFS {
        if let Some(content) = article_def.content {
            pages.push(PageDef {
                page: "articles",
                href: article_def.href,
                content: content.to_owned(),
            });
        }
    }

    pages
}

fn generate_json_pages() -> HashMap<&'static str, Bytes> {
    get_page_defs()
        .into_iter()
        .map(|page_def| {
            let json = serde_json::to_string(&JsonPage {
                content: page_def.content,
            })
            .unwrap();

            (page_def.href, json.into())
        })
        .collect()
}

fn generate_static_pages(color_scheme: PreferredColorScheme) -> HashMap<&'static str, Bytes> {
    get_page_defs()
        .into_iter()
        .map(|page_def| {
            let html = build_document(page_def.page, page_def.content, color_scheme);
            (page_def.href, html)
        })
        .collect()
}

fn build_document(page: &str, content: String, color_scheme: PreferredColorScheme) -> Bytes {
    let body = format!(
        "<body class=\"{page}\">{THEME_SELECTOR}{}{content}</body>",
        build_menu(page),
    );
    Document::new(body, color_scheme).to_string().into()
}

fn build_menu(active_page: &str) -> String {
    let menu_items = vec![
        build_menu_item("me", "/", "Me", active_page),
        build_menu_item("people", "/people", "People", active_page),
        build_menu_item("projects", "/projects", "Projects", active_page),
        build_menu_item("articles", "/articles", "Articles", active_page),
        //generate_menu_item("contact", "/contact", "Contact", active_page)
    ];

    format!("<ul class=\"menu\">{}</ul>", menu_items.join(""))
}

fn build_menu_item(page: &str, href: &str, title: &str, active_page: &str) -> String {
    let mut classes = vec![page];
    if page == active_page {
        classes.push("active");
    };

    format!(
        "<li><a class=\"{}\" href=\"{href}\">{title}</a></li>",
        classes.join(" "),
    )
}

fn generate_articles_content() -> String {
    let links = ARTICLE_DEFS
        .iter()
        .map(|&ArticleDef { title, href, .. }| format!("<li><a href=\"{href}\">{title}</a></li>"))
        .collect::<Vec<_>>()
        .join("");

    format!(
        "<div class=\"content\">\
            <h1>Articles</h1>\
            <p>Not an avid blogger myself, I do write the occassional post. I've listed them here for your enjoyment:</p>\
            <ul>{links}</ul>\
        </div>"
    )
}

fn get_cookie<'a>(
    headers: &'a HeaderMap<HeaderValue>,
    cookie_name: &'static str,
) -> Option<&'a str> {
    let Some(cookie) = headers.get("Cookie") else {
        return None;
    };

    let Ok(cookie_str) = cookie.to_str() else {
        return None;
    };

    cookie_str
        .split(';')
        .filter_map(|cookie| cookie.split_once('='))
        .find(|(name, _)| *name == cookie_name)
        .map(|(_, value)| value)
}

fn serve(req: Request<Body>) -> Result<Response<Body>, Error> {
    if req.method() != "GET" {
        return Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body("Method Not Allowed".into());
    }

    let path = req.uri().path();
    if path == "/main.js" {
        return Response::builder()
            .header("Content-Type", APPLICATION_JAVASCRIPT.clone())
            .body(MAIN_JS.into());
    } else if path == "/prism.js" {
        return Response::builder()
            .header("Content-Type", APPLICATION_JAVASCRIPT.clone())
            .body(PRISM_JS.into());
    } else if path == "/prism.css" {
        return Response::builder()
            .header("Content-Type", TEXT_CSS.clone())
            .body(PRISM_CSS.into());
    }

    let color_scheme_override = match req.uri().query() {
        Some("preferred_color_scheme=light") => PreferredColorScheme::Light,
        Some("preferred_color_scheme=dark") => PreferredColorScheme::Dark,
        _ => PreferredColorScheme::Unspecified,
    };

    let (content_type, pages) = if req.headers().get("Accept") == Some(&*APPLICATION_JSON) {
        (APPLICATION_JSON.clone(), &*JSON_PAGES)
    } else {
        let preferred_color_scheme = if color_scheme_override == PreferredColorScheme::Unspecified {
            match get_cookie(req.headers(), "color_scheme") {
                Some("light") => PreferredColorScheme::Light,
                Some("dark") => PreferredColorScheme::Dark,
                _ => PreferredColorScheme::Unspecified,
            }
        } else {
            color_scheme_override
        };

        (
            TEXT_HTML.clone(),
            match preferred_color_scheme {
                PreferredColorScheme::Dark => &*DARK_PAGES,
                PreferredColorScheme::Light => &*LIGHT_PAGES,
                PreferredColorScheme::Unspecified => &*PAGES,
            },
        )
    };

    match pages.get(req.uri().path()) {
        Some(page) => Response::builder()
            .header("Content-Type", content_type)
            .header("Vary", "Accept")
            .color_scheme_cookie(color_scheme_override)
            .body(page.clone().into()),
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not Found".into()),
    }
}

trait ColorSchemeExt {
    fn color_scheme_cookie(self, color_scheme_override: PreferredColorScheme) -> Self;
}

impl ColorSchemeExt for ResponseBuilder {
    fn color_scheme_cookie(self, color_scheme_override: PreferredColorScheme) -> Self {
        if color_scheme_override == PreferredColorScheme::Unspecified {
            self
        } else {
            self.header(
                "Set-Cookie",
                HeaderValue::from_static(if color_scheme_override == PreferredColorScheme::Dark {
                    "color_scheme=dark; Path=/"
                } else {
                    "color_scheme=light; Path=/"
                }),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([0, 0, 0, 0], PORT));

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async { serve(req) }))
    });

    let server = Server::bind(&address).serve(make_service);
    println!("Listening on port {PORT}...");

    if let Err(error) = server.await {
        eprintln!("server error: {error}");
    }
}

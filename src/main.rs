#![recursion_limit = "256"]

use bytes::Bytes;
use hyper::header::{HeaderMap, HeaderValue};
use hyper::http::StatusCode;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;
use typed_html::dom::DOMTree;
use typed_html::elements::FlowContent;
use typed_html::types::{Class, SpacedSet};
use typed_html::{html, text};

mod components;
use components::{Document, PreferredColorScheme};

#[derive(Serialize, Debug)]
struct JsonPage {
    content: String,
}

lazy_static! {
    static ref APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
    static ref MAIN_JS: Bytes = fs::read_to_string("main.js")
        .expect("Could not read main JS")
        .into();
    static ref DARK_PAGES: HashMap<&'static str, Bytes> =
        generate_static_pages(PreferredColorScheme::Dark);
    static ref JSON_PAGES: HashMap<&'static str, Bytes> = generate_json_pages();
    static ref LIGHT_PAGES: HashMap<&'static str, Bytes> =
        generate_static_pages(PreferredColorScheme::Light);
    static ref PAGES: HashMap<&'static str, Bytes> =
        generate_static_pages(PreferredColorScheme::Unspecified);
}

fn generate_json_pages() -> HashMap<&'static str, Bytes> {
    let mut pages = HashMap::new();

    pages.insert("/", generate_json_page(generate_index_content));
    pages.insert("/people", generate_json_page(generate_people_content));
    pages.insert("/projects", generate_json_page(generate_projects_content));

    pages
}

fn generate_static_pages(color_scheme: PreferredColorScheme) -> HashMap<&'static str, Bytes> {
    let mut pages = HashMap::new();

    pages.insert(
        "/",
        generate_document("me", generate_index_content, color_scheme),
    );
    pages.insert(
        "/people",
        generate_document("people", generate_people_content, color_scheme),
    );
    pages.insert(
        "/projects",
        generate_document("projects", generate_projects_content, color_scheme),
    );

    pages
}

fn generate_document(
    page: &str,
    generate_content: impl Fn() -> Box<dyn FlowContent<String>>,
    color_scheme: PreferredColorScheme,
) -> Bytes {
    let mut body: DOMTree<String> = html!(<body class={page}>
        {generate_theme_selector()}
        {generate_menu(page)}
        {generate_content()}
    </body>);
    let doc = Document::new(body.vnode(), color_scheme);
    doc.to_string().into()
}

fn generate_json_page(generate_content: impl Fn() -> Box<dyn FlowContent<String>>) -> Bytes {
    let content = generate_content();
    serde_json::to_string(&JsonPage {
        content: content.to_string(),
    })
    .unwrap()
    .into()
}

fn generate_menu(active_page: &str) -> Box<dyn FlowContent<String>> {
    html!(<ul class="menu">
        <li>{generate_menu_item("me", "/", "Me", active_page)}</li>
        <li>{generate_menu_item("people", "/people", "People", active_page)}</li>
        <li>{generate_menu_item("projects", "/projects", "Projects", active_page)}</li>
        <li>{generate_menu_item("articles", "/articles", "Articles", active_page)}</li>
        <li>{generate_menu_item("contact", "/contact", "Contact", active_page)}</li>
    </ul>)
}

fn generate_menu_item(
    page: &str,
    href: &str,
    title: &str,
    active_page: &str,
) -> Box<dyn FlowContent<String>> {
    let mut classes: SpacedSet<Class> = SpacedSet::new();
    classes.insert(Class::new(page.to_string()));
    if page == active_page {
        classes.insert(Class::new("active"));
    };
    html!(<a class={classes} href={href}>{text!(title)}</a>)
}

fn generate_theme_selector() -> Box<dyn FlowContent<String>> {
    html!(<div class="theme-selector">
        <a class="dark" href="?preferred_color_scheme=dark">"Dark theme"</a>
        <a class="light" href="?preferred_color_scheme=light">"Light theme"</a>
    </div>)
}

fn generate_index_content() -> Box<dyn FlowContent<String>> {
    html!(<div class="content">
        <h1>"Arend van Beelen jr."</h1>
        <p>"I am a software engineer by trade and an author for leasure."</p>
        <p>"Have a look around and may we get acquinted."</p>
    </div>)
}

fn generate_people_content() -> Box<dyn FlowContent<String>> {
    html!(<div class="content">
        <h1>"People"</h1>
        <p><a href="https://www.ciyuxu.nl">"Ciyu Xu"</a></p>
        <p><a href="https://www.aronsilver.com">"Aron Silver"</a></p>
    </div>)
}

fn generate_projects_content() -> Box<dyn FlowContent<String>> {
    html!(<div class="content">
        <h1>"Projects"</h1>
        <p><a href="https://github.com/arendjr/text-clipper">"text-clipper"</a>" - A JavaScript text truncation library."</p>
        <p><a href="https://github.com/arendjr/PlainText">"PlainText"</a>" - A MUD engine written in C++ and JavaScript."</p>
    </div>)
}

fn get_cookie<'a>(
    headers: &'a HeaderMap<HeaderValue>,
    cookie_name: &'static str,
) -> Option<&'a str> {
    if let Some(cookie) = headers.get("Cookie") {
        if let Ok(cookie_str) = cookie.to_str() {
            let cookies = cookie_str.split(';');
            for cookie in cookies {
                let parts: Vec<&str> = cookie.trim().split('=').collect();
                if parts.len() == 2 && parts[0] == cookie_name {
                    return Some(parts[1]);
                }
            }
        }
    }
    None
}

fn serve(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if req.method() != "GET" {
        let mut response = Response::new(Body::from("Method Not Allowed"));
        *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        return Ok(response);
    }

    if req.uri().path() == "/main.js" {
        let mut response = Response::new(Body::from(MAIN_JS.clone()));
        response.headers_mut().insert(
            "Content-Type",
            HeaderValue::from_static("application/javascript"),
        );
        return Ok(response);
    }

    let color_scheme_override = match req.uri().query() {
        Some("preferred_color_scheme=light") => PreferredColorScheme::Light,
        Some("preferred_color_scheme=dark") => PreferredColorScheme::Dark,
        _ => PreferredColorScheme::Unspecified,
    };

    let (content_type, pages) = if req.headers().get("Accept") == Some(&*APPLICATION_JSON) {
        (APPLICATION_JSON.clone(), &*JSON_PAGES)
    } else {
        let mut preferred_color_scheme = match get_cookie(req.headers(), "color_scheme") {
            Some("light") => PreferredColorScheme::Light,
            Some("dark") => PreferredColorScheme::Dark,
            _ => PreferredColorScheme::Unspecified,
        };

        if color_scheme_override != PreferredColorScheme::Unspecified {
            preferred_color_scheme = color_scheme_override;
        }

        (
            HeaderValue::from_static("text/html; charset=UTF-8"),
            match preferred_color_scheme {
                PreferredColorScheme::Dark => &*DARK_PAGES,
                PreferredColorScheme::Light => &*LIGHT_PAGES,
                PreferredColorScheme::Unspecified => &*PAGES,
            },
        )
    };

    match pages.get(req.uri().path()) {
        Some(page) => {
            let mut response = Response::new(Body::from(page.clone()));
            response.headers_mut().insert("Content-Type", content_type);
            if color_scheme_override != PreferredColorScheme::Unspecified {
                response.headers_mut().insert(
                    "Set-Cookie",
                    HeaderValue::from_static(
                        if color_scheme_override == PreferredColorScheme::Dark {
                            "color_scheme=dark"
                        } else {
                            "color_scheme=light"
                        },
                    ),
                );
            }
            Ok(response)
        }
        None => {
            let mut response = Response::new(Body::from("Not Found"));
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async { serve(req) }))
    });

    let server = Server::bind(&address).serve(make_service);

    if let Err(error) = server.await {
        eprintln!("server error: {}", error);
    }
}

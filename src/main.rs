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
use typed_html::{html, text, unsafe_text};

mod components;
use components::{Document, PreferredColorScheme};

#[derive(Serialize, Debug)]
struct JsonPage {
    content: String,
}

struct ArticleDef {
    href: &'static str,
    filename: &'static str,
    title: &'static str,
}

struct PageDef {
    page: &'static str,
    href: &'static str,
    content: Box<dyn FlowContent<String>>,
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

fn get_article_defs() -> Vec<ArticleDef> {
    vec![
        ArticleDef {
            href: "/2016/09/how-i-made-text-clipper-fastest-html.html",
            filename: "how-i-made-text-clipper-fastest-html",
            title: "How I made text-clipper the fastest HTML clipping library",
        },
        ArticleDef {
            href: "/2016/08/selectivity-v3-adds-react-support.html",
            filename: "selectivity-v3-adds-react-support",
            title: "Selectivity v3 adds React support",
        },
        ArticleDef {
            href: "/2015/05/selectivityjs-v110-improves-keyboard.html",
            filename: "selectivityjs-v110-improves-keyboard",
            title: "Selectivity v1.1.0 improves keyboard support, adds Ruby Gem",
        },
        ArticleDef {
            href: "/2015/03/select3-v10-released.html",
            filename: "select3-v10-released",
            title: "Selectivity v1.0 released",
        },
        ArticleDef {
            href: "/2015/02/creating-submenus-with-select3.html",
            filename: "creating-submenus-with-select3",
            title: "Creating submenus with Selectivity.js",
        },
    ]
}

fn get_page_defs() -> Vec<PageDef> {
    let mut pages = vec![
        PageDef {
            page: "me",
            href: "/",
            content: generate_index_content(),
        },
        PageDef {
            page: "people",
            href: "/people",
            content: generate_people_content(),
        },
        PageDef {
            page: "portfolio",
            href: "/portfolio",
            content: generate_portfolio_content(),
        },
        PageDef {
            page: "articles",
            href: "/articles",
            content: generate_articles_content(),
        },
    ];

    for article_def in get_article_defs() {
        pages.push(PageDef {
            page: "articles",
            href: article_def.href,
            content: generate_article_content(&article_def),
        });
    }

    pages
}

fn generate_json_pages() -> HashMap<&'static str, Bytes> {
    let mut pages = HashMap::new();

    for page_def in get_page_defs() {
        pages.insert(
            page_def.href,
            serde_json::to_string(&JsonPage {
                content: page_def.content.to_string(),
            })
            .unwrap()
            .into(),
        );
    }

    pages
}

fn generate_static_pages(color_scheme: PreferredColorScheme) -> HashMap<&'static str, Bytes> {
    let mut pages = HashMap::new();

    for page_def in get_page_defs() {
        pages.insert(
            page_def.href,
            generate_document(page_def.page, page_def.content, color_scheme),
        );
    }

    pages
}

fn generate_document(
    page: &str,
    content: Box<dyn FlowContent<String>>,
    color_scheme: PreferredColorScheme,
) -> Bytes {
    let body: DOMTree<String> = html!(<body class={page}>
        {generate_theme_selector()}
        {generate_menu(page)}
        {content}
    </body>);
    let doc = Document::new(body, color_scheme);
    doc.to_string().into()
}

fn generate_menu(active_page: &str) -> Box<dyn FlowContent<String>> {
    html!(<ul class="menu">
        <li>{generate_menu_item("me", "/", "Me", active_page)}</li>
        <li>{generate_menu_item("people", "/people", "People", active_page)}</li>
        <li>{generate_menu_item("portfolio", "/portfolio", "Portfolio", active_page)}</li>
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
        <p>"My two passions are technology and writing, but neither would exist without the people that create them. A good story without a convincing protagonist inevitably falls flat. And good technology that doesn't cater to the people will never be more than a curio. These are some of the people that inspire my life..."</p>
        <p class="alternate-a">
            <a href="https://www.ciyuxu.nl"><b>"Ciyu Xu"</b></a><br />
            "My loving wife with whom I live in Haarlem. She's a designer and frontend developer."
        </p>
        <p class="alternate-b">
            <b>"Boris van Beelen"</b><br />
            "Our beautiful son. He's a little too young to have his own online presence yet :)"
        </p>
        <p class="alternate-a">
            <a href="https://www.aronsilver.com"><b>"Aron Silver"</b></a><br />
            "A Dutch author of little renown, so far. I'm quite into his works..."
        </p>
    </div>)
}

fn generate_portfolio_content() -> Box<dyn FlowContent<String>> {
    html!(<div class="content">
        <h1>"Portfolio"</h1>
        <p>"Ever since my dad taught me my first few lines of BASIC, programming has been in my passion. I'll present the highlights from recent to old, to spare you the history if you're not interested:"</p>
        <p class="alternate-a">
            <b>"This website"</b><br />
            "This website was custom-built in Rust as an experience to get familiar with the language. Feel free to look at "
            <a href="https://github.com/arendjr/phebe">"the source"</a>"."
        </p>
        <p class="alternate-b">
            <a href="https://www.speakap.com/"><b>"Speakap"</b></a><br />
            "I am employed by Speakap by over 8 years now, where I work as a Principal Software Engineer. Some of the public projects that I created for them are "
            <a href="https://github.com/arendjr/text-clipper">"text-clipper"</a>" and "
            <a href="http://arendjr.github.io/selectivity/">"Selectivity.js"</a>"."
        </p>
        <p class="alternate-a">
            <a href="https://github.com/arendjr/PlainText"><b>"PlainText"</b></a><br />
            "PlainText is a MUD engine I built in C++ and JavaScript as a hobby."
        </p>
        <p class="alternate-b">
            <b>"Hyves"</b><br />
            "Hyves was the largest social network of the Netherlands. I worked there for five-and-a-half years, leading the team that created the Hyves Desktop suite using C++/Qt and web technologies. I also developed several server-side features in PHP and worked on their chat server created using Stackless Python. Finally, I also worked on their mobile stack creating hybrid PhoneGap applications for Symbian, Android, iOS and BlackBerry. Symbian in particular was a fun one, as I had to built the PhoneGap container itself first in C++ :)"
        </p>
        <p class="alternate-a">
            <a href="https://kde.org/"><b>"KDE project"</b></a><br />
            "During my time at university I made several contributions to the KDE project on the side. From the search bar for the Konqueror web browser and the type-ahead-find feature in its KHTML rendering engine to support for the Windows RDP protcol in the KDE Remote Desktop Client application. For the latter, rather than reimplementing the RDP protocol, I created a patch for the "<a href="https://www.rdesktop.org/">"rdesktop application"</a>" so that its X11 window could be embedded into other windows, which was then used by KRDC."
        </p>
    </div>)
}

fn generate_articles_content() -> Box<dyn FlowContent<String>> {
    let articles = get_article_defs();

    html!(<div class="content">
        <h1>"Articles"</h1>
        <p>"Not a great blogger myself, I do write the occassional post. I've listed them here for your enjoyment:"</p>
        <ul>
            {articles.iter().map(|article| html!(<li><a href={article.href}>{text!(article.title)}</a></li>))}
        </ul>
    </div>)
}

fn generate_article_content(article: &ArticleDef) -> Box<dyn FlowContent<String>> {
    let content = fs::read_to_string(format!("articles/{}.html", article.filename))
        .expect("Could not read article");
    html!(<div class="content">
        <h1>{text!(article.title)}</h1>
        {unsafe_text!(content)}
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

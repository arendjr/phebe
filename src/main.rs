#![recursion_limit = "512"]

use bytes::Bytes;
use hyper::header::{HeaderMap, HeaderValue};
use hyper::http::StatusCode;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use std::net::SocketAddr;

mod components;
use components::PreferredColorScheme;

use crate::components::Document;

const PORT: u16 = 3000;

#[derive(Serialize)]
struct JsonPage {
    content: String,
}

struct ArticleDef {
    href: &'static str,
    filename: Option<&'static str>,
    title: &'static str,
}

struct PageDef {
    page: &'static str,
    href: &'static str,
    content: String,
}

static APPLICATION_JSON: Lazy<HeaderValue> =
    Lazy::new(|| HeaderValue::from_static("application/json"));
static MAIN_JS: Lazy<Bytes> = Lazy::new(|| {
    fs::read_to_string("main.js")
        .expect("Could not read main JS")
        .into()
});
static PRISM_JS: Lazy<Bytes> = Lazy::new(|| {
    fs::read_to_string("prism.js")
        .expect("Could not read prism JS")
        .into()
});
static PRISM_CSS: Lazy<Bytes> = Lazy::new(|| {
    fs::read_to_string("prism.css")
        .expect("Could not read prism CSS")
        .into()
});
static DARK_PAGES: Lazy<HashMap<&'static str, Bytes>> =
    Lazy::new(|| generate_static_pages(PreferredColorScheme::Dark));
static JSON_PAGES: Lazy<HashMap<&'static str, Bytes>> = Lazy::new(generate_json_pages);
static LIGHT_PAGES: Lazy<HashMap<&'static str, Bytes>> =
    Lazy::new(|| generate_static_pages(PreferredColorScheme::Light));
static PAGES: Lazy<HashMap<&'static str, Bytes>> =
    Lazy::new(|| generate_static_pages(PreferredColorScheme::Unspecified));

const THEME_SELECTOR: &str = "<div class=\"theme-selector\">
    <a class=\"dark\" href=\"?preferred_color_scheme=dark\">Dark theme</a>
    <a class=\"light\" href=\"?preferred_color_scheme=light\">Light theme</a>
</div>";

fn get_article_defs() -> Vec<ArticleDef> {
    vec![
        ArticleDef {
            href: "https://fiberplane.dev/blog/creating-a-rich-text-editor-using-rust-and-react/",
            filename: None,
            title: "Creating a Rich Text Editor using Rust and React (Fiberplane)",
        },
        ArticleDef {
            href: "https://fiberplane.dev/blog/writing-redux-reducers-in-rust/",
            filename: None,
            title: "Writing Redux reducers in Rust (Fiberplane)",
        },
        ArticleDef {
            href: "https://fiberplane.dev/blog/announcing-fp-bindgen/",
            filename: None,
            title: "Announcing fp-bindgen (Fiberplane)",
        },
        ArticleDef {
            href: "/2021/09/js-pipe-proposal-battle-of-perspectives.html",
            filename: Some("js-pipe-proposal-battle-of-perspectives"),
            title: "JavaScript Pipe Operator Proposal: A Battle of Perspectives",
        },
        ArticleDef {
            href: "/2021/01/how-to-multiple-redux-slice-instances.html",
            filename: Some("how-to-multiple-redux-slice-instances"),
            title: "Advanced Redux: How to create multiple instances of a state slice",
        },
        ArticleDef {
            href: "/2016/09/how-i-made-text-clipper-fastest-html.html",
            filename: Some("how-i-made-text-clipper-fastest-html"),
            title: "How I made text-clipper the fastest HTML clipping library",
        },
        ArticleDef {
            href: "/2016/08/selectivity-v3-adds-react-support.html",
            filename: Some("selectivity-v3-adds-react-support"),
            title: "Selectivity v3 adds React support",
        },
        ArticleDef {
            href: "/2015/05/selectivityjs-v110-improves-keyboard.html",
            filename: Some("selectivityjs-v110-improves-keyboard"),
            title: "Selectivity v1.1.0 improves keyboard support, adds Ruby Gem",
        },
        ArticleDef {
            href: "/2015/03/select3-v10-released.html",
            filename: Some("select3-v10-released"),
            title: "Selectivity v1.0 released",
        },
        ArticleDef {
            href: "/2015/02/creating-submenus-with-select3.html",
            filename: Some("creating-submenus-with-select3"),
            title: "Creating submenus with Selectivity.js",
        },
    ]
}

fn get_page_defs() -> Vec<PageDef> {
    let mut pages = vec![
        PageDef {
            page: "me",
            href: "/",
            content: INDEX_CONTENT.to_owned(),
        },
        PageDef {
            page: "people",
            href: "/people",
            content: PEOPLE_CONTENT.to_owned(),
        },
        PageDef {
            page: "projects",
            href: "/projects",
            content: PROJECTS_CONTENT.to_owned(),
        },
        PageDef {
            page: "articles",
            href: "/articles",
            content: generate_articles_content(),
        },
    ];

    for article_def in get_article_defs() {
        if let Some(filename) = article_def.filename {
            pages.push(PageDef {
                page: "articles",
                href: article_def.href,
                content: generate_article_content(filename, article_def.title),
            });
        }
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

fn generate_document(page: &str, content: String, color_scheme: PreferredColorScheme) -> Bytes {
    let body: String = format!(
        "<body class=\"{}\">{}{}{}</body>",
        page,
        THEME_SELECTOR,
        generate_menu(page),
        content
    );
    Document::new(body, color_scheme).to_string().into()
}

fn generate_menu(active_page: &str) -> String {
    let menu_items = vec![
        generate_menu_item("me", "/", "Me", active_page),
        generate_menu_item("people", "/people", "People", active_page),
        generate_menu_item("projects", "/projects", "Projects", active_page),
        generate_menu_item("articles", "/articles", "Articles", active_page),
        //generate_menu_item("contact", "/contact", "Contact", active_page)
    ];

    format!("<ul class=\"menu\">{}</ul>", menu_items.join(""))
}

fn generate_menu_item(page: &str, href: &str, title: &str, active_page: &str) -> String {
    let mut classes = vec![page];
    if page == active_page {
        classes.push("active");
    };

    format!(
        "<li><a class=\"{}\" href=\"{}\">{}</a></li>",
        classes.join(" "),
        href,
        title
    )
}

const INDEX_CONTENT: &str = "<div class=\"content\">
    <h1>Arend van Beelen jr.</h1>
    <p>Welcome,</p>
    <p>I am a software engineer by trade and an author for leasure.</p>
    <p>Have a look around and may we get acquainted.</p>
    <p>Yours,<br>Arend jr.</p>
</div>";

const PEOPLE_CONTENT: &str = "<div class=\"content\">
    <h1>People</h1>
    <p>My two passions are technology and writing, but neither would exist
        without the people that create them. A good story without a convincing
        protagonist inevitably falls flat. And good technology that doesn't
        cater to the people will never be more than a curio. These are some of
        the people that inspire my life...</p>
    <p class=\"alternate-a\">
        <a href=\"https://www.ciyuxu.nl\"><b>Ciyu Xu</b></a><br>
        My loving wife with whom I live in Haarlem. She's a designer and frontend developer.
    </p>
    <p class=\"alternate-b\">
        <b>Boris van Beelen</b><br>
        Our beautiful son. He's a little too young to have his own online presence yet :)
    </p>
    <p class=\"alternate-a\">
        <a href=\"https://www.aronsilver.com\"><b>Aron Silver</b></a><br>
        A Dutch author of little renown, so far. I'm quite into his works...
    </p>
</div>";

const PROJECTS_CONTENT: &str = "<div class=\"content\">
    <h1>Projects</h1>
    <p>Ever since my dad taught me my first few lines of BASIC, programming has
        been my passion. I'll present the highlights from recent to old, to
        spare you the history if you're not interested:</p>
    <p class=\"alternate\">
        <a href=\"https://fiberplane.dev/\"><b>Fiberplane</b></a><br>
        I am currently working at Fiberplane, where we are building exciting DevOps tooling.
        As part of their efforts, I'm involved with
        <a href=\"https://github.com/fiberplane/fp-bindgen\">fp-bindgen</a>,
        a bindings generator for full-stack WASM plugins.
    </p>
    <p class=\"alternate\">
        <b>This website</b><br>
        This website was custom-built in Rust as an exercise to get familiar
        with the language. Don't expect any marvels, but feel free to look at
        <a href=\"https://github.com/arendjr/phebe\">the source</a>.
    </p>
    <p class=\"alternate\">
        <a href=\"https://www.speakap.com/\"><b>Speakap</b></a><br>
        I was a Principal Software Engineer at Speakap, where I worked for over
        7 years. Some open-source projects that I created for them are
        <a href=\"https://github.com/arendjr/text-clipper\">text-clipper</a> and
        <a href=\"http://arendjr.github.io/selectivity/\">Selectivity.js</a>.
    </p>
    <p class=\"alternate\">
        <a href=\"https://play.google.com/store/apps/details?id=com.xiao_games.SudokuPi\"><b>Sudoku Pi</b></a><br>
        Sudoku Pi is a finger-friendly Sudoku game for Android. Kudos to
        <a href=\"https://www.ciyuxu.nl\">my wife</a> for the design.
    </p>
    <p class=\"alternate\">
        <a href=\"https://github.com/arendjr/PlainText\"><b>PlainText</b></a><br>
        PlainText is a MUD engine I originally built in C++ and JavaScript as
        a hobby. A port to Rust (again as an exercise to gain experience) is
        WIP.
    </p>
    <p class=\"alternate\">
        <b>Hyves</b><br>
        Hyves used to be the largest social network of the Netherlands. I worked
        there for five-and-a-half years, leading the team that created the Hyves
        Desktop suite using C++/Qt and web technologies. I also developed
        several server-side features in PHP and worked on their chat server
        created using Stackless Python. Finally, I also worked on their mobile
        stack creating hybrid PhoneGap applications for Symbian, Android, iOS
        and BlackBerry. Symbian in particular was a fun one, as I first had to
        built the PhoneGap container itself in C++ :)<br>
        An open-source project (now hopelessly outdated) I developed as part of
        my employment there was
        <a href=\"https://github.com/arendjr/woodpecker\">Woodpecker</a>, an
        SCSS compiler written in Python.
    </p>
    <p class=\"alternate\">
        <b>Distributed friend graph database</b><br>
        For my university's master's project, which I did at Hyves, I created a
        PoC for a distributed friend graph database in C++. It was able to
        efficiently retrieve things like friend recommendations as well as
        determine whether any two people shared a connection in the third
        degree.
    </p>
    <p class=\"alternate\">
        <a href=\"https://kde.org/\"><b>KDE project</b></a><br>
        During my time at university I made several contributions to the KDE
        project on the side. From the search bar for the Konqueror web browser
        and the type-ahead-find feature in its KHTML rendering engine to
        support for the Windows RDP protcol in the KDE Remote Desktop Client
        application. For the latter, rather than reimplementing the RDP
        protocol, I created a patch for the
        <a href=\"https://www.rdesktop.org/\">rdesktop application</a> so that
        its X11 window could be embedded into other windows, which was then
        used by KRDC.
    </p>
    <p class=\"alternate\">
        <b>Real-time raytracing engine</b><br>
        For my university's bachelor's project me and three fellow students
        developed a PoC real-time raytracing engine in C++ with CUDA support. Of
        course, given the hardware at the time, this was only feasible on low
        resolutions using various shortcuts, but it was an interesting project
        nonetheless.
    </p>
    <p class=\"alternate\">
        <b>Aukyla</b><br>
        During my first part-time job, I worked as a solo developer in a
        <a href=\"https://www.auton.nl/\">small automation firm</a> that lead to
        the formation of Aukyla Software, where I did what every programmer did
        at the time: Create their own PHP CMS. I created the
        <a href=\"https://sourceforge.net/projects/aukyla/\">Aukyla PHP Framework</a>,
        upon which I built <a href=\"https://sourceforge.net/projects/adms/\">ADMS</a>,
        an intranet-focused Document Management System, and Aukyla Site Manager,
        which powered several public websites at the time.
    </p>
    <p class=\"alternate\">
        <a href=\"https://github.com/arendjr/Qivrit\"><b>Qivrit</b></a><br>
        In order to help myself and fellow classmates in high-school improve our
        natural language skills, I developed a little Windows application using
        Borland C++ Builder. The concept was simple: You enter the lists of all
        the words with their translations, and the program will question you on
        your knowledge of them afterwards. I don't think I have the source to
        this application anymore, but years later, as I made an attempt to learn
        Hebrew, I wrote a similar program called Qivrit. This one was also
        written in C++, but using Qt and tests both knowledge of the Hebrew
        alphabet as well as basic vocabulary.
    </p>
    <p class=\"alternate\">
        <b>Adventure</b><br>
        One of my oldest hobby projects is a game: Adventure. If my memory
        serves me right, the first version was created in QBASIC. When I started
        using KDE, I made a port called
        <a href=\"https://store.kde.org/p/1109408/\">KAdventure</a>.
    </p>
</div>";

fn generate_articles_content() -> String {
    let articles = get_article_defs();
    let links = articles
        .iter()
        .map(|article| {
            format!(
                "<li><a href=\"{}\">{}</a></li>",
                article.href, article.title
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!("<div class=\"content\">
    <h1>Articles</h1>
    <p>Not an avid blogger myself, I do write the occassional post. I've listed them here for your enjoyment:</p>
    <ul>{}</ul>
</div>", links)
}

fn generate_article_content(filename: &str, title: &str) -> String {
    let content =
        fs::read_to_string(format!("articles/{}.html", filename)).expect("Could not read article");
    format!(
        "<div class=\"content\">
<h1>{}</h1>
{}
</div>",
        title, content
    )
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

    let path = req.uri().path();
    if path == "/main.js" || path == "/prism.js" {
        let mut response = Response::new(Body::from(if path == "/prism.js" {
            PRISM_JS.clone()
        } else {
            MAIN_JS.clone()
        }));
        response.headers_mut().insert(
            "Content-Type",
            HeaderValue::from_static("application/javascript"),
        );
        return Ok(response);
    } else if path == "/prism.css" {
        let mut response = Response::new(Body::from(PRISM_CSS.clone()));
        response
            .headers_mut()
            .insert("Content-Type", HeaderValue::from_static("text/css"));
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
                            "color_scheme=dark; Path=/"
                        } else {
                            "color_scheme=light; Path=/"
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
    let address = SocketAddr::from(([0, 0, 0, 0], PORT));

    let make_service = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async { serve(req) }))
    });

    let server = Server::bind(&address).serve(make_service);
    println!("Listening on port {}...", PORT);

    if let Err(error) = server.await {
        eprintln!("server error: {}", error);
    }
}

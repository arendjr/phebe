const menuLinks = document.querySelectorAll(".menu a");

const CODE_MARKER = '<code class="language-';

const pages = new Map();
pages.set(location.pathname, { pageClass: document.body.className });

function highlightCode() {
    if (window.Prism) {
        Prism.highlightAllUnder(document.querySelector(".content"));
    } else {
        const link = document.createElement("link");
        link.setAttribute("href", "/prism.css");
        link.setAttribute("rel", "stylesheet");
        link.setAttribute("type", "text/css");
        document.head.appendChild(link);

        const script = document.createElement("script");
        script.setAttribute("src", "/prism.js");
        document.head.appendChild(script);
    }
}

function loadPage(href, pageClass) {
    if (!pages.has(href) || !pages.get(href).promise) {
        pages.set(href, {
            pageClass,
            promise: fetch(href, {
                headers: {
                    Accept: "application/json",
                    "Cache-Control": "no-cache",
                },
            }).then(response => response.json()),
        });
    }
    return pages.get(href).promise;
}

function navigateTo(href, pageClass) {
    menuLinks.forEach(a => a.classList.remove("active"));

    loadPage(href, pageClass)
        .then(({ content }) => {
            document.body.className = pageClass;
            document
                .querySelector(".menu a." + pageClass)
                .classList.add("active");
            document.querySelector(".content").outerHTML = content;

            if (content.includes(CODE_MARKER)) {
                highlightCode();
            }
        })
        .catch(error => {
            console.error(
                "Failed to load page; fall back to full-page reload",
                error
            );
            window.location = href;
        });
}

menuLinks.forEach(a => {
    a.addEventListener("mouseenter", () => {
        const href = a.getAttribute("href");
        if (href !== location.pathname) {
            loadPage(href, a.className);
        }
    });
});

document.body.addEventListener("click", event => {
    if (event.target.tagName === "A") {
        const a = event.target;
        const href = a.getAttribute("href");
        if (href.startsWith("/")) {
            event.preventDefault();

            history.pushState(null, document.head.title, href);
            navigateTo(
                href,
                a.className.replace(/\s*active\s*/, "") ||
                    document.body.className
            );
        }
    }
});

window.onpopstate = event => {
    event.preventDefault();
    navigateTo(location.pathname, pages.get(location.pathname).pageClass);
};

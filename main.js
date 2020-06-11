const menuLinks = document.querySelectorAll(".menu a");

const pages = new Map();
pages.set(location.pathname, { pageClass: document.body.className });

function loadPage(href, pageClass) {
    if (!pages.has(href) || !pages.get(href).promise) {
        pages.set(href, {
            pageClass,
            promise: fetch(href, {
                headers: { Accept: "application/json" },
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
        if (!a.classList.contains("active")) {
            loadPage(a.getAttribute("href"), a.className);
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
                a.getAttribute("href"),
                a.className || document.body.className
            );
        }
    }
});

window.onpopstate = event => {
    event.preventDefault();
    navigateTo(location.pathname, pages.get(location.pathname).pageClass);
};

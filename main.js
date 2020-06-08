const pages = new Map();

function loadPage(href) {
    if (!pages.has(href)) {
        pages.set(
            href,
            fetch(href, {
                headers: { Accept: "application/json" },
            }).then(response => response.json())
        );
    }
    return pages.get(href);
}

const menuLinks = document.querySelectorAll(".menu a");
menuLinks.forEach(a => {
    a.addEventListener("click", event => {
        event.preventDefault();

        menuLinks.forEach(a => a.classList.remove("active"));
        document.body.className = a.className;
        a.classList.add("active");

        loadPage(a.getAttribute("href"))
            .then(({ content }) => {
                document.querySelector(".content").outerHTML = content;
            })
            .catch(error => {
                console.error(
                    "Failed to load page; fall back to full-page reload",
                    error
                );
                window.location = a.getAttribute("href");
            });
    });

    a.addEventListener("mouseenter", () => {
        if (!a.classList.contains("active")) {
            loadPage(a.getAttribute("href"));
        }
    });
});

--- html-head html ---
#html.head(
  html.title("Title")
    + html.meta(charset: "utf-8")
    + html.base(href: "https://example.com/")
    + html.style("body { color: red; }")
    + html.link(rel: "stylesheet", href: "style.css")
    + html.script("console.log('hi')")
    + html.noscript("JS disabled")
)
Hello World

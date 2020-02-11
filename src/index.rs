use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "blogpost.html")]
pub struct BlogPostTemplate<'a> {
    page_title: &'a str,
    post_title: &'a str,
    author: &'a str,
    body: &'a str,
}

impl<'a> BlogPostTemplate<'a> {
    pub fn new(page_title: &'a str, post_title: &'a str, author: &'a str, body: &'a str) -> Self {
        Self {
            page_title,
            post_title,
            author,
            body,
        }
    }
}

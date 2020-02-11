use askama::Template;
use warp::Filter;

mod index;

#[tokio::main]
async fn main() {
    let index = warp::path!("index" / String).map(|name: String| {
        let template = index::IndexTemplate { name: &name };
        warp::reply::html(template.render().unwrap())
    });

    warp::serve(index).run(([127, 0, 0, 1], 3030)).await;
}

use askama::Template;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use axum::{
    response::{Html, IntoResponse, Response},
    http::StatusCode,
    routing::get, 
    extract,
    Router};

#[tokio::main]
async fn main() {
    // Build the application with a route
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/about", get(about_handler))
        .route("/hello/:name", get(hello_handler))
        .route("/dvd", get(dvd_handler))
        .nest_service("/assets", ServeDir::new("assets"));

    // Run the application
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_handler() -> impl IntoResponse {
    println!("/ called");
    let template = IndexTemplate {}; // Instantiate Struct
    HtmlTemplate(template) // Render Struct
}

async fn about_handler() -> impl IntoResponse {
    println!("/about called");
    let template = AboutTemplate {}; // Instantiate Struct
    HtmlTemplate(template) // Render Struct
}

async fn hello_handler(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    println!("/hello/{} called", name);
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

async fn dvd_handler() -> impl IntoResponse {
    println!("/dvd called");
    let template = DvdTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)] // Askama generated the code..
#[template(path = "index.html")] // using the template in the below path relative to templates
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "dvd.html")]
struct DvdTemplate {}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
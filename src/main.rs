#![feature(proc_macro_hygiene, decl_macro)]

// std

// crates
use rocket::{get, http::ContentType, response::content::Content, routes, *};
use rocket_contrib::{json::Json, serve::StaticFiles, templates::Template};

// local
use shelf::{Book, Context, NoData};

static NOT_FOUND: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/static/not_found.html"
));

#[catch(404)]
fn not_found() -> Content<&'static str> {
    Content(ContentType::HTML, NOT_FOUND)
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        &Context {
            title: "Index",
            parent: "layout",
            data: Some(NoData()),
        },
    )
}

#[post("/search")]
fn return_search_results() //-> Json<>
{
}

#[get("/edit")]
fn edit() -> Template {
    Template::render(
        "edit",
        &Context {
            title: "Template",
            parent: "layout",
            data: Some(NoData()),
        },
    )
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index, return_search_results, edit])
        .launch();
}

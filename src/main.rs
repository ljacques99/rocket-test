#[macro_use] extern crate rocket;
use rocket::http::uri::Segments;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, Rocket from /hello!"
}

#[get("/item/<id>/<name>")]
fn get_item(id: &str, name: &str) -> String {
    format!("Item ID: {}, Name: {}", id, name)
}

#[get("/items?<page>&<per_page>&<sort_by>")]
fn list_items(page: Option<u32>, per_page: Option<u32>, sort_by: Option<&str>) -> String {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let sort_by = sort_by.unwrap_or("id");
    
    format!("Page: {}, Items per page: {}, Sort by: {}", page, per_page, sort_by)
}

#[get("/items/<ids..>")]
fn get_items_by_ids<'r>(ids: Segments<'r, rocket::http::uri::fmt::Path>) -> String {
    let ids_str = ids.collect::<Vec<_>>().join(", ");
    format!("Requested items with IDs: {}", ids_str)
}

#[get("/itemsmultiple?<id>")]
fn get_items_multiple(id: Vec<String>) -> String {
    format!("Requested items with IDs: {:?}", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8090)))
        .mount("/", routes![index, hello, get_item, list_items, get_items_by_ids, get_items_multiple])
}

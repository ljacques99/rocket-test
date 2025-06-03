#[macro_use] extern crate rocket;
use rocket::http::uri::Segments;
use rocket::serde::json::Json;
use rocket::form::Form;
use rocket::data::ToByteUnit;
use rocket::fs::TempFile;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Serialize)]
struct Person {
    name: String,
    age: u32,
    wage: f64,
}

#[derive(FromForm, Debug)]
struct Product {
    name: String,
    price: f64,
    category: String,
    in_stock: bool,
    quantity: Option<u32>,
}

#[derive(FromForm, Debug)]
struct DocumentUpload<'r> {
    title: String,
    description: String,
    file: TempFile<'r>,
}

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

#[post("/person", format = "json", data = "<person>")]  //je pourrais aussi ajouter des paramètres avec /<param1>/<param2> ... et aussi avec querystring ?<query1>&<query2>
fn create_person(person: Json<Person>) -> Json<Person> {
    // Here you would typically save the person to a database
    // For now, we'll just echo back the received person data
    Json(person.into_inner())
}

#[post("/dynamic", format = "json", data = "<data>")]
fn handle_dynamic_json(data: Json<Value>) -> Json<Value> {
    // Here you can handle any JSON structure
    // For example, let's add a timestamp to the response
    let mut response = data.into_inner();
    if let Some(obj) = response.as_object_mut() {
        obj.insert("timestamp".to_string(), Value::String(chrono::Utc::now().to_rfc3339()));
    }
    Json(response)
}

#[post("/product", data = "<product>")]
fn create_product(product: Form<Product>) -> String {
    format!(
        "Created product: {} (${:.2}) in category '{}'. Stock: {} (Quantity: {})",
        product.name,
        product.price,
        product.category,
        if product.in_stock { "Available" } else { "Out of stock" },
        product.quantity.map_or("Not specified".to_string(), |q| q.to_string())
    )
}

#[post("/upload", data = "<upload>")]
async fn upload_document(mut upload: Form<DocumentUpload<'_>>) -> String { //en fonction de la suite, le mut peut être optionnel
    // Read the file content
    let mut content = String::new();
    if let Ok(mut file) = upload.file.open().await {
        if let Err(e) = file.read_to_string(&mut content).await {
            return format!("Error reading file: {}", e);
        }
    }

    format!(
        "Document uploaded:\nTitle: {}\nDescription: {}\nFile content:\n{}",
        upload.title,
        upload.description,
        content
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 8090)))
        .mount("/", routes![index, hello, get_item, list_items, get_items_by_ids, get_items_multiple, create_person, handle_dynamic_json, create_product, upload_document])
}

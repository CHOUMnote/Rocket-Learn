#[macro_use]
extern crate rocket;
pub use serde::{Deserialize, Serialize};

#[get("/")] //root
fn index() -> &'static str {
    "Hello Rocket!!!"
}

#[get("/<name>/<age>")]
fn wave(name: String, age: Option<i32>) -> String {
    let mut n: String = String::new();
    n.push_str(&name);
    match age {
        Some(a) => n.push_str(&a.to_string()),
        None => n.push_str("👋"),
    }
    n
}

mod requests;
use requests::methods::*;
#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![index, wave]).mount(
        "/methods",
        routes![static_paths, delay, get_path, foo_bar, everything],
    )
}

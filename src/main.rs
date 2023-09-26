#[macro_use] extern crate rocket;

use id23::id_v1;
use chrono::Utc;

#[get("/")]
fn id() -> String {
    id_v1(Utc::now()).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![id])
}
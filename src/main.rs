#[macro_use]
extern crate rocket;
use rocket::serde::{Deserialize, json::Json};
use rocket::tokio::time::{Duration, sleep};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    email: String,
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[post("/user", format = "json", data = "<user>")]
fn new_user(user: Json<User>) -> String {
    format!("User created: {} with email {}", user.name, user.email)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![delay])
        .mount("/", routes![new_user])
    // .mount("/test", routes![test])
}

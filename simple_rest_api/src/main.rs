#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &str {
    "hello world"
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
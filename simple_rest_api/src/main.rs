#[macro_use] extern crate rocket;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;

let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
let config = aws_config::from_env().region(region_provider).load().await;
let client = Client::new(&config);

#[get("/")]
fn index() -> &str {
    "hello world"
}


#[get("/users/<id>")]
fn get_user(id: &str)->i8{

}

#[get("/skills/<id>")]
fn get_skill(id: &str)->i8{

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
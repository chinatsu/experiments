#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::request::{Request, FromRequest, Outcome};
use rocket::http::Status;

struct IdToken<'r>(&'r str);

#[derive(Debug)]
enum IdTokenError {
    BadCount,
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IdToken<'r> {
    type Error = IdTokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        let cookie = req.cookies().get("selvbetjening-idtoken");

        match cookie {
            Some(c) if is_valid(c.value()) => Outcome::Success(IdToken(c.value())),
            None => Outcome::Failure((Status::Unauthorized, IdTokenError::Missing)),
            _ => Outcome::Failure((Status::Unauthorized, IdTokenError::Invalid))
        }
    }
}

#[get("/")]
fn hello(key: IdToken<'_>) -> String {
    String::from("Hello, world!")
}

#[rocket::main]
async fn main()  {
    rocket::build().mount("/", routes![hello]).launch().await;
}

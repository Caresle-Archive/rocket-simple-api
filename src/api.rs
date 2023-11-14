use rocket::Route;

mod posts;
mod users;
mod countries;

#[get("/")]
pub fn api_root() -> &'static str {
    "ROCKER SIMPLE API"
}

pub fn api_router() -> Vec<Route> {
    routes![
        api_root,
        // Posts
        posts::get,
        posts::create,
        posts::update,
        posts::destroy,

        // Users
        users::get,
        users::create,
        users::update,
        users::destroy,

        // Countries
        countries::get,
        countries::create,
        countries::update,
        countries::destroy,
    ]
}
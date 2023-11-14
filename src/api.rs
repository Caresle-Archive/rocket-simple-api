use rocket::Route;

#[get("/")]
pub fn api_root() -> &'static str {
    "ROCKER SIMPLE API MOD API"
}

pub fn api_router() -> Vec<Route> {
    routes![api_root]
}
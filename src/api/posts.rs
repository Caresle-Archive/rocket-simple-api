#[get("/posts")]
pub fn get() -> &'static str {
    "List of post"
}

#[post("/posts")]
pub fn create() -> &'static str {
    "Post created"
}

#[put("/posts")]
pub fn update() -> &'static str {
    "Post updated"
}

#[delete("/posts")]
pub fn destroy() -> &'static str {
    "Post deleted"
}

#[get("/users")]
pub fn get() -> &'static str {
    "List of users"
}

#[post("/users")]
pub fn create() -> &'static str {
    "User created"
}

#[put("/users")]
pub fn update() -> &'static str {
    "User updated"
}

#[delete("/users")]
pub fn destroy() -> &'static str {
    "User destroy"
}
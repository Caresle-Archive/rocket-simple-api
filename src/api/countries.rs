#[get("/countries")]
pub fn get() -> &'static str {
    "List of countries"
}

#[post("/countries")]
pub fn create() -> &'static str {
    "Created countries"
}

#[put("/countries")]
pub fn update() -> &'static str {
    "Updated countries"
}

#[delete("/countries")]
pub fn destroy() -> &'static str {
    "Destroy countries"
}
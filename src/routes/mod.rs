pub mod residents;

#[get("/")]
pub fn index() -> String {
    info!("Get index...");
    "Home page".to_string()
}

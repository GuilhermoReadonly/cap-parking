use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, response::status::NotFound};

pub mod residents;

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let file = if file.to_str() == Some("") {
        PathBuf::from("index.html")
    } else {
        file
    };

    let path = Path::new("resources/web-app/").join(file);
    NamedFile::open(&path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

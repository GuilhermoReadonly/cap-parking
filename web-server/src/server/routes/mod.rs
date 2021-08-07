use rocket::{fs::NamedFile, response::status::NotFound};
use std::env;
use std::path::{Path, PathBuf};

pub mod residents;

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let file = if file.to_str() == Some("") {
        PathBuf::from("index.html")
    } else {
        file
    };

    let path = env::current_dir().expect("yes it is");
    info!("The current directory is {}", path.display());

    let path = Path::new("resources/web-app/").join(file);
    NamedFile::open(&path)
        .await
        .map_err(|e| NotFound(e.to_string()))
}

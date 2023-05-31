use std::path::PathBuf;
use app::App;
use term::start_term;
use dirs::home_dir;

pub mod app;
pub mod term;
pub mod process;

fn main() {
    let path: PathBuf = resolve_home("~/.steam/steam/steamapps/workshop/content/431960/"); // TODO: change this to an arg
    let app = App::new().load(path);
    start_term(app).expect("Exiting");
}

pub fn resolve_home(path: &str) -> PathBuf {
    if path.starts_with("~") {
        if let Some(home_dir) = home_dir() {
            let mut full_path = home_dir;
            full_path.push(&path[2..]);
            return full_path;
        }
    }
    PathBuf::from(path)
}
use std::path::PathBuf;
use list::{App};
use util::{resolve_home};
use term::start_term;

pub mod list;
pub mod util;
pub mod term;

fn main() {
    let path: PathBuf = resolve_home("~/.steam/steam/steamapps/workshop/content/431960/"); // TODO: change this to an arg
    let app = App::new().load(path);
    start_term(app).expect("Exiting");
}
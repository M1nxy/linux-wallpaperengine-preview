use std::{sync::{MutexGuard, Mutex}, ffi::OsString, path::PathBuf, process::{Command, Stdio, Child}};
use dirs::home_dir;

lazy_static::lazy_static! {
    static ref WALLPAPER_PROCESS: Mutex<Option<std::process::Child>> = Mutex::new(None);
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

pub fn preview(sel: &OsString){
    let mut shell = Command::new("sh");
    let install_check = shell.arg("-c").arg("which linux-wallpaperengine").status().expect("failed").success();
    let mut unlocked = unlock_process();
    
    if install_check {
        let process = Command::new("linux-wallpaperengine")
            .arg(sel)
            .stdout(Stdio::null())
            .spawn()
            .expect("sh command failed to start");
        *unlocked = Some(process);
    }
}


pub fn unlock_process() -> MutexGuard<'static, Option<std::process::Child>> {
    let mut unlocked = WALLPAPER_PROCESS.lock().unwrap();
    match &mut *unlocked {
        Some(x) => x.kill().expect("Failed to kill previous process!"),
        None => {}
    };
    return unlocked
}
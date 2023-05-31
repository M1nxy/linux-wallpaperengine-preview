use std::{process::{Child, Command, Stdio}, ffi::OsString, sync::Mutex};

pub struct ProcessManager {
    process: Mutex<Option<std::process::Child>> 
}

impl ProcessManager {
    pub fn new()-> ProcessManager {
        return ProcessManager { process: Mutex::from(None)};
    }
    pub fn spawn(&mut self, sel: &OsString){
        self.try_kill();
        let process: Child = Command::new("linux-wallpaperengine")
            .arg(sel)
            .stdout(Stdio::null())
            .spawn()
            .expect("sh command failed to start");
        self.process = Mutex::from(Some(process));
    }

    pub fn try_kill(&mut self){
        let mut unlocked = self.process.lock().unwrap();
        match &mut *unlocked {
            Some(child) => child.kill().expect("Failed to kill previous process!"),
            None => {}
        }
    }
}

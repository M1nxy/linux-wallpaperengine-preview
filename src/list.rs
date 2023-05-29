use std::{ffi::OsString, path::PathBuf, fs::read_dir};
use tui::widgets::ListState;

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<(OsString, usize)>,
}

impl StatefulList {
    pub fn with_items(items: Vec<(OsString, usize)>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct App {
    pub items: StatefulList,
}

impl<'a> App {
    pub fn load(mut self, path: PathBuf) -> App {
        let dir_entries = read_dir(path).expect("Unable to find any wallpapers!");
        let mut entries: Vec<_> = dir_entries
            .enumerate()
            .map(|(index, item)| {
                (item.expect("Failed to read directory entry").file_name(), index)
            })
            .collect();
    
        entries.sort_by_key(|entry| {
            let filename = entry.0.to_owned();
            let filename_str = filename.to_string_lossy();
            filename_str.parse::<u32>().unwrap_or_default()
        });

        self.items = StatefulList::with_items(entries);
        return self
    }
    pub fn new() -> App {
        let mut app = App { items: StatefulList::with_items(vec![]) };
        app.items.state.select(Some(0));
        return app;
    }
    pub fn on_tick(&mut self) {
    }
}
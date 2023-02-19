use self::icon_lookup::guess_icon;
use self::icon_lookup::DEFAULT_ICON;
use hyprland::{
    data::Client,
    event_listener::WindowEventData,
    shared::{HResult, HyprDataActiveOptional},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod icon_lookup;

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    title: String,
    icon_path: PathBuf,
}

impl Title {
    pub fn new() -> HResult<Self> {
        let class: String;
        let title: String;

        match Client::get_active()? {
            Some(c) => {
                class = c.class;
                title = c.title;
            }
            None => {
                class = String::new();
                title = String::new();
            }
        };

        Ok(Self {
            title,
            icon_path: guess_icon(&class),
        })
    }

    pub fn print_json(&self) {
        let msg = serde_json::to_string(&self).unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));

        println!("{msg}");
    }

    pub fn on_active_window_change(&mut self, data: Option<WindowEventData>) {
        match data {
            Some(c) => {
                self.title = c.1;
                self.icon_path = guess_icon(&c.0);
            }
            None => {
                self.title = String::new();
                self.icon_path = DEFAULT_ICON.into();
            }
        };

        self.print_json();
    }
}

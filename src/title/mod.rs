use crate::title::icon_lookup::guess_icon;
use hyprland::event_listener::WindowEventData;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod icon_lookup;

#[derive(Debug, Serialize, Deserialize)]
pub struct Title;

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    title: String,
    icon_path: PathBuf,
}

impl Title {
    pub fn on_active_window_change(&self, data: Option<WindowEventData>) {
        if let Some(data) = data {
            let obj = Message {
                title: data.1,
                icon_path: guess_icon(&data.0),
            };

            let msg =
                serde_json::to_string(&obj).unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));

            println!("{msg}");
        }
    }
}

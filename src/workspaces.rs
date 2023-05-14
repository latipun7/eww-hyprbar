use arrayvec::ArrayVec;
use hyprland::{
    data::{Workspace, Workspaces},
    shared::{HyprData, HyprDataActive, WorkspaceType},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Space {
    id: String,
    windows: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WS {
    current: String,
    spaces: Result<[Space; 5], ArrayVec<Space, 5>>,
}

impl WS {
    fn get_spaces() -> hyprland::Result<Result<[Space; 5], ArrayVec<Space, 5>>> {
        let mut space = ArrayVec::<Space, 5>::new();
        let workspaces: Vec<Workspace> = Workspaces::get()?.filter(|item| item.id > 0).collect();

        for i in 1..6 {
            match workspaces.iter().find(|ws| ws.id == i) {
                Some(w) => space.push(Space {
                    id: w.id.to_string(),
                    windows: w.windows,
                }),
                None => space.push(Space {
                    id: i.to_string(),
                    windows: 0,
                }),
            }
        }

        Ok(space.into_inner())
    }

    fn update_spaces(&mut self) {
        self.spaces = WS::get_spaces().unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));
    }

    pub fn new() -> hyprland::Result<Self> {
        Ok(Self {
            current: match Workspace::get_active() {
                Ok(ws) => ws.id.to_string(),
                Err(_) => String::from("1"),
            },
            spaces: WS::get_spaces()?,
        })
    }

    pub fn print_json(&self) {
        let msg = serde_json::to_string(&self).unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));

        println!("{msg}");
    }

    pub fn on_workspace_change(&mut self, ws: WorkspaceType) {
        let name = match ws {
            WorkspaceType::Regular(ws) => ws,
            WorkspaceType::Special(ws) => match ws {
                Some(expr) => expr,
                None => String::new(),
            },
        };

        self.current = name;
        self.print_json();
    }

    pub fn on_window_events(&mut self) {
        self.update_spaces();
        self.print_json();
    }
}

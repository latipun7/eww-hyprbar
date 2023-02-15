use clap::Parser;
use eww_hyprbar::{server::HyprManager, title::Title, workspaces::WS};
use hyprland::shared::HResult;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Parser)]
enum Cmd {
    Title,
    Workspaces,
}

fn main() -> HResult<()> {
    let cmd = Cmd::parse();
    let mut server = HyprManager::new();

    match cmd {
        Cmd::Title => {
            server
                .listener
                .add_active_window_change_handler(|data, _| Title.on_active_window_change(data));
        }
        Cmd::Workspaces => {
            let workspaces = Rc::new(RefCell::new(WS::new()?));

            let watcher1 = workspaces.clone();
            server.listener.add_workspace_change_handler(move |ws, _| {
                watcher1.borrow_mut().on_workspace_change(ws)
            });

            let watcher2 = workspaces.clone();
            server
                .listener
                .add_window_open_handler(move |_, _| watcher2.borrow_mut().on_window_events());

            let watcher3 = workspaces.clone();
            server
                .listener
                .add_window_moved_handler(move |_, _| watcher3.borrow_mut().on_window_events());

            server
                .listener
                .add_window_close_handler(move |_, _| workspaces.borrow_mut().on_window_events());
        }
    }

    server.start_listener_blocking()
}

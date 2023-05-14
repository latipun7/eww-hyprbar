pub mod title;

pub mod workspaces;

pub mod server {
    use hyprland::{
        data::Monitor, event_listener::EventListenerMutable as EventListener,
        shared::HyprDataActive,
    };

    pub struct HyprManager {
        pub listener: EventListener,
        pub monitor: hyprland::Result<Monitor>,
    }

    impl HyprManager {
        pub fn new() -> Self {
            eprintln!("initializing event listener...");
            Self {
                listener: EventListener::new(),
                monitor: Monitor::get_active(),
            }
        }

        pub fn start_listener_blocking(self) -> hyprland::Result<()> {
            eprintln!("started blocking listener");
            self.listener.start_listener()
        }
    }

    impl Default for HyprManager {
        fn default() -> Self {
            Self::new()
        }
    }
}

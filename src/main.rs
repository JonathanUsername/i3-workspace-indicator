extern crate i3ipc;

use i3ipc::I3Connection;
use i3ipc::I3EventListener;

use i3ipc::event::Event;
use i3ipc::reply::Workspace;
use i3ipc::Subscription;

use i3ipc::event::inner::WorkspaceChange;

const UNFOCUSED: &'static str = "%{F#6F6F6F}%{F-}";
const FOCUSED: &'static str = "";

// const UNFOCUSED: &'static str = "NO";
// const FOCUSED: &'static str = "YES";

fn get_ordered_workspaces() -> Vec<Workspace> {
    let mut connection = I3Connection::connect().unwrap();
    let mut workspaces = connection.get_workspaces().unwrap().workspaces;
    workspaces.sort_by_key(|i| i.num);
    workspaces
}

fn get_symbolised_workspace_focus() -> String {
    let workspaces = get_ordered_workspaces();
    let max_workspace_num = workspaces.iter().map(|i| i.num).max().unwrap();

    (1..=max_workspace_num).map(|idx| {
        match workspaces.iter().find(|w| w.num == idx) {
            Some(w) => if w.focused { FOCUSED } else { UNFOCUSED},
            None => UNFOCUSED
        }
    }).collect::<Vec<&str>>().join(" ")
}

fn main() {
    // establish connection.
    let mut listener = I3EventListener::connect().unwrap();

    let subs = [Subscription::Workspace];
    listener.subscribe(&subs).unwrap();

    for event in listener.listen() {
        match event.unwrap() {
            Event::WorkspaceEvent(e) => match e.change {
                WorkspaceChange::Focus => println!("{}", get_symbolised_workspace_focus()),
                _ => (),
            },
            _ => (),
        }
    }
}

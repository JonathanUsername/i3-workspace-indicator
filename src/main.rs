extern crate i3ipc;

use i3ipc::I3Connection;
use i3ipc::I3EventListener;

use i3ipc::event::Event;
use i3ipc::reply::Workspace;
use i3ipc::Subscription;
use i3ipc::event::inner::WorkspaceChange;

fn get_ordered_workspaces() -> Vec<Workspace> {
    let mut connection = I3Connection::connect().unwrap();
    let mut workspaces = connection.get_workspaces().unwrap().workspaces;
    workspaces.sort_by_key(|i| i.num);
    workspaces
}

fn get_symbolised_workspace_focus() -> String {
    let workspaces = get_ordered_workspaces();
    let max_workspace_num = workspaces.iter().map(|i| i.num).max().unwrap();

    (1..=max_workspace_num)
        .map(|idx| match workspaces.iter().find(|w| w.num == idx) {
            Some(w) => {
                if w.focused {
                    format_indicator(true, idx.to_string())
                } else {
                    format_indicator(false, idx.to_string())
                }
            }
            None => format_indicator(false, idx.to_string()),
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn format_indicator(focussed: bool, s: String) -> String {
    // https://github.com/polybar/polybar/wiki/Formatting#format-tags
    // brackets are escaped by more brackets:
    // https://stackoverflow.com/questions/25569865/how-to-escape-curly-braces-in-a-format-string-in-rust
    if focussed {
        format!("%{{F#fff}}{}%{{F-}}", s)
    } else {
        format!("%{{F#888}}{}%{{F-}}", s)
    }
}

fn print_status() {
    println!("{}", get_symbolised_workspace_focus());
}

fn main() {
    // Print once to start with
    print_status();

    // establish subscriber.
    let mut listener = I3EventListener::connect().unwrap();

    let subs = [Subscription::Workspace];
    listener.subscribe(&subs).unwrap();

    for event in listener.listen() {
        match event.unwrap() {
            Event::WorkspaceEvent(e) => match e.change {
                WorkspaceChange::Focus => print_status(),
                _ => (),
            },
            _ => (),
        }
    }
}

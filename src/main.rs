extern crate i3ipc;
use i3ipc::I3Connection;

const UNFOCUSED: &'static str = "%{F#8A2BE2}%{F-}";
const FOCUSED: &'static str = "";

fn main() {
    // establish a connection to i3 over a unix socket
    let mut connection = I3Connection::connect().unwrap();
    
    // request and print the i3 version
    let workspaces = connection.get_workspaces().unwrap().workspaces;
    let bools: Vec<bool> = workspaces.iter().map(|i| i.focused).collect();

    let out_str: Vec<String> = bools.iter().map(|&i| {
        let val = if i { FOCUSED } else { UNFOCUSED };
        return val.to_owned();
    }).collect();
    println!("{}", out_str.join(" "));
}

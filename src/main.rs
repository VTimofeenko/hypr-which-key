mod get_bindings;
mod utils;
use crate::get_bindings::{run_and_parse_command, ParsedBinding};
use crate::utils::mod_mask_to_string;

use hyprland::event_listener::EventListener;
// use hyprland::prelude::*;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct ShownBinding {
    key_combo: String,
    description: String,
}

impl From<ShownBinding> for String {
    fn from(value: ShownBinding) -> Self {
        value.key_combo + " : " + &value.description
    }
}

impl From<ParsedBinding> for ShownBinding {
    fn from(value: ParsedBinding) -> Self {
        Self {
            key_combo: {
                let mut result = mod_mask_to_string(value.modmask);
                result.push(value.key);
                result.join("+")
            },
            description: {
                if value.description.is_empty() {
                    [value.dispatcher.clone(), value.arg.clone()].join(" ")
                } else {
                    value.description.clone()
                }
            },
        }
    }
}

#[derive(Debug)]
enum Message {
    Start(String),
    Stop,
}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Here be the event listener that connects to hyprland socket and sends the submap data
        let mut listener = EventListener::new();

        listener.add_sub_map_change_handler(move |data| {
            println!("Sending {data:#?}");
            match data.is_empty() {
                true => tx.send(Message::Stop).unwrap(),
                false => {
                    // let binds = Binds::get().unwrap();
                    // using hyprctl to get binds since at the time of writing hyprland rust binds
                    // don't have binds description
                    //
                    // Time to cheat using hyprctl binds that has it
                    let binds = run_and_parse_command().unwrap();

                    let message = binds
                        .into_iter()
                        .filter(|b| b.submap == data)
                        // This looks ugly, but From
                        // trait is not transitive?
                        .map(|b| Into::<ShownBinding>::into(b).into())
                        .collect::<Vec<String>>();

                    tx.send(Message::Start(message.join("\n"))).unwrap()
                }
            };
        });

        listener.start_listener()
    });

    let mut child: Option<Child> = None;

    loop {
        thread::sleep(Duration::from_secs(1));
        match rx.recv() {
            Ok(msg) => {
                println!("{:?}", msg);
                match msg {
                    Message::Start(data) => {
                        if child.is_none() {
                            println!("Starting...");
                            // FIXME: This is bad. "Echo" approach is used because bemenu needs
                            // full stdin to render choices. I am not even using bemenu to select
                            // anything at this point. Moreover using the bemenu as
                            // selection/search would conflict with the actual shortcuts

                            let first_command = Command::new("echo")
                                .arg(data.clone())
                                .stdout(Stdio::piped())
                                .spawn()
                                .expect("Failed to start first command");

                            child = Some(
                                Command::new("bemenu")
                                    .arg("-l")
                                    .arg("10")
                                    .arg("--prompt")
                                    .arg("")
                                    .stdin(first_command.stdout.unwrap())
                                    .spawn()
                                    .expect("Failed to start subprocess"),
                            );
                        };
                    }
                    Message::Stop => {
                        if let Some(mut c) = child.take() {
                            println!("Stopping subprocess...");
                            c.kill().expect("Failed to kill subprocess");
                        }
                    }
                };
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

use hyprland::data::*;
use hyprland::event_listener::EventListener;
use hyprland::prelude::*;
use std::collections::HashMap;
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

// Taken from https://github.com/hyprland-community/Hyprkeys
fn mod_mask_to_string(mod_mask: u16) -> Vec<String> {
    // TODO: Make this const
    let mod_masks = HashMap::from([
        (1, "SHIFT"),
        (2, "CAPS"),
        (4, "CTRL"),
        (8, "ALT"),
        (16, "MOD2"),
        (32, "MOD3"),
        (64, "SUPER"),
        (128, "MOD5"),
    ]);
    let mut cur_val = 7;
    let mut result: Vec<String> = Vec::new();
    let mut mod_mask = mod_mask;

    while mod_mask > 0 {
        let mod_val = 1 << cur_val;
        if mod_mask >= mod_val {
            mod_mask -= mod_val;
            result.push(mod_masks[&(1 << cur_val)].to_string());
        }
        cur_val -= 1;
    }
    result
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
                    let binds = Binds::get().unwrap();

                    let mode_binds = binds
                        .into_iter()
                        .filter(|b| b.submap == data)
                        .collect::<Vec<Bind>>();

                    let message: Vec<String> = mode_binds
                        .iter()
                        .map(|bind| {
                            // Logic that creates an individual mapping message
                            let mut key_combo = mod_mask_to_string(bind.modmask);
                            key_combo.push(bind.key.clone());

                            let binding = ShownBinding {
                                key_combo: key_combo.join("+"),
                                description: [bind.dispatcher.clone(), bind.arg.clone()].join(" "),
                            };
                            binding.into()
                        })
                        .collect();

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
                println!("Error: {}", e);
                break;
            }
        }
    }
}

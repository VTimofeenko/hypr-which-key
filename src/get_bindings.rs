use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::process::Command;

/// Contains parsed fields from hyprctl binds
#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedBinding {
    locked: bool,
    mouse: bool,
    release: bool,
    repeat: bool,
    non_consuming: bool,
    has_description: bool,
    pub modmask: u16,
    pub submap: String,
    pub key: String,
    // keycode: u16,
    catch_all: bool,
    pub description: String,
    pub dispatcher: String,
    pub arg: String,
}

/// Retrieves bindings by calling hyprctl binds and parsing the results
/// This is suboptimal, but rust bindings for hyprland don't have binds description
pub fn run_and_parse_command() -> Result<Vec<ParsedBinding>> {
    let output = Command::new("hyprctl")
        .args(vec!["binds", "-j"])
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Command failed with status: {:?}", output.status);
        return Err(serde_json::Error::custom("Command execution failed"));
    }

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in command output");
    let parsed: Vec<ParsedBinding> = serde_json::from_str(&stdout)?;

    Ok(parsed)
}

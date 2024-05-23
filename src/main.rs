use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

/**
 * The main objective of this project is to move i3 to next or previous workspace with bellow rules:
 * 1. when receive from terminal a positive number, move to the next worspace on the same active monitor
 * 2. when receive from terminal a negative number, move to the previous worspace on the same active monitor
 * 3. when is the first or last workspace on the active montior, does not move to the next or previous workspace
*/

// struct to store the workspace data received from i3-msg -t get_workspaces
#[derive(Serialize, Deserialize)]
struct Workspace {
    num: isize,
    name: String,
    visible: bool,
    focused: bool,
    urgent: bool,
    rect: Rect,
    output: String,
}

// struct to store the rect data received from i3-msg -t get_workspaces
#[derive(Serialize, Deserialize)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct WorkspaceFiltered {
    num: isize,
    focused: bool,
}

fn main() -> Result<()> {
    // get the workspace data from i3-msg -t get_workspaces
    let output = Command::new("i3-msg")
        .arg("-t")
        .arg("get_workspaces")
        .output()
        .expect("failed to execute process");

    //  parse the output to Workspace struct
    let output = String::from_utf8(output.stdout)?;
    let workspaces: Vec<Workspace> = serde_json::from_str(&output)?;

    // get the direction from terminal
    let direction = std::env::args()
        .nth(1)
        .unwrap_or("1".to_string())
        .parse::<isize>()?;
    let active_output = workspaces
        .iter()
        .find(|w| w.focused)
        .unwrap()
        .output
        .clone();

    // filter the workspaces by the active output
    let mut workspaces: Vec<WorkspaceFiltered> = workspaces
        .into_iter()
        .filter_map(|w| {
            if w.output.eq(&active_output) {
                Some(WorkspaceFiltered {
                    num: w.num,
                    focused: w.focused,
                })
            } else {
                None
            }
        })
        .collect();
    workspaces.sort_by(|a, b| a.num.cmp(&b.num));

    // get the current workspace index
    let active_index = workspaces.iter().position(|w| w.focused).unwrap();
    if (direction < 0 && active_index > 0) || (direction > 0 && active_index < workspaces.len() - 1)
    {
        let next_index = match direction {
            -1 => active_index - 1,
            1 => active_index + 1,
            _ => active_index,
        };
        let next_workspace = workspaces.get(next_index).unwrap();
        Command::new("i3-msg")
            .arg("workspace")
            .arg(format!("{}", next_workspace.num))
            .spawn()
            .expect("failed to execute process");
    }

    Ok(())
}

// IMPORTS
use regex::Regex;
use serde::Deserialize;
use serde_json::Result;
use std::env;
use std::fs;
use std::process::Command;
use sysinfo::{ProcessesToUpdate, System};

// define a struct with the 'apps' variable that can hold a vector of strings
#[derive(Deserialize, Debug)]
struct Config {
    apps: Vec<String>,
}

fn main() -> Result<()> {
    // get current working directory (of the compiled binary)
    let cwd = env::current_dir().expect("$HOME environment variable not found");
    const APP_DIR: &'static str = "/Applications/";
    // set path to apps.json
    let json_path = &cwd.join("apps.json");
    // read apps.json
    let json_data =
        fs::read_to_string(json_path).expect("couldnt read the apps.json. Is it there?");
    // deserialize the json into an instance of Config(struct)
    let config: Config = serde_json::from_str(&json_data).expect("couldnt deserialize the json");
    // extract the apps array from the json
    let apps: Vec<String> = config.apps;
    // create an instance of the System struct (from sysinfo crate)
    let mut sys = System::new();
    // refresh process statuses
    sys.refresh_processes(ProcessesToUpdate::All);

    // open apps that are not running
    for app in apps {
        // get is_running status as 'true' or 'false'
        let is_running = sys.processes_by_name(app.as_ref()).next().is_some(); // next() => returns next item from iterator or None // is_some() => returns true for an entry and false for None

        // if app is not running, execute open command
        if !is_running {
            println!("opening => {}", app);
            Command::new("open")
                .arg(format!("/Applications/{}.app", app))
                .spawn()
                .expect("failed to open => {}");
        } else {
            println!("already running => {}", app);
        }
    }

    Ok(()) // main function result
}

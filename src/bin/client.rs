use ReverseShell::client::Client;
use std::io::{self, Write};

fn main() {
    let mut my_client = Client::connect().unwrap();

    loop {
        let mut input = String::new();
        print!("🥜> ");
        io::stdin().read_line(&mut input).unwrap();
        my_client.send(&input).unwrap();
        println!("{}", my_client.receive().unwrap());
        if input.trim() == "YouShouldKillYourself" || input.trim() == "KYS" {
            println!("Exit client Because you murderd the server 🔪🔪🔪");
            std::process::exit(0);
        }
    }
}
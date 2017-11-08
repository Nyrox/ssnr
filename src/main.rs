use std::env;
use std::process::Command;

mod server;
mod client;

use client::Client;

fn test(id: &str) {
	let mut client = Client::connect("127.0.0.1:8542");
	client.push(id);
}

fn main() {
	for argument in env::args() {
		println!("{}", argument);
	}
	
	let arguments: Vec<String> = env::args().collect();
	if arguments.len() < 2 {
		println!("Not enough parameters were supplied. Usage: 'ssnr <command> <params>");
		return;
	}
	
	match &*(arguments[1]) {
		"test" => { test(&arguments[2]) }
		"serv" => {
			server::start_server();
		}
		_ => panic!("{} is not a valid command.", arguments[1])
	}
	
}

use std::env;
use std::process::Command;

mod server;
mod client;
mod api;

use client::Client;

fn push(id: &str) {
	let mut client = Client::connect("127.0.0.1:8542");
	client.push(id);
}

fn pull(id: &str) {
	let mut client = Client::connect("127.0.0.1:8542");
	client.pull(id);
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
		"push" => { push(&arguments[2]) },
		"pull" => { pull(&arguments[2]) },
		"serv" => {
			server::start_server();
		}
		_ => panic!("{} is not a valid command.", arguments[1])
	}
	
}

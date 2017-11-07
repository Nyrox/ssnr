use std::env;

mod server;
mod client;

use client::Client;

fn test() {
	let mut client = Client::connect("127.0.0.1:8542");
	client.push("test");
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
	
	// Start a local server for dev purposes
	server::start_server_local();	
	
	match &*(arguments[1]) {
		"test" => { test() }
		_ => panic!("{} is not a valid command.", arguments[1])
	}
	
	
	loop { }
}

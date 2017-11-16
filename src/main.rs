extern crate toml;

use std::env;
use std::process::Command;

mod server;
mod client;
mod api;

use client::Client;

use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Default)]
struct Config {
	default_host: String,
	default_port: i64
}

impl Config {
	pub fn load() -> Config {
		let mut config = Config::default();
		
		let mut config_dir = std::env::home_dir().unwrap();
		config_dir.push(".config/ssnr");
		fs::create_dir_all(config_dir.clone());
		let config_file = { config_dir.push("config.toml"); config_dir };		
		
		if let Ok(mut file) = File::open(config_file) {
			let mut buffer = String::new();
			file.read_to_string(&mut buffer);
			let toml = buffer.parse::<toml::Value>().unwrap();
			println!("{:?}", toml);
			
			match toml.get("default_host") {
				Some(&toml::Value::String(ref host)) => { config.default_host = host.clone(); }
				Some(value) => { println!("Config parsing error: Found default_host property in config file, but it's value could not be parsed. Expected value::String, found: {:?}", value)}
				_ => ()
			}
			
			match toml.get("default_port") {
				Some(&toml::Value::Integer(port)) => { config.default_port = port; }
				Some(value) => { println!("Config parsing error: Found default_port property in config file, but it's value could not be parsed. Expected value::Integer, found: {:?}", value)}
				_ => ()
			}
		}
		
		config
	}
}



fn push(id: &str) {
	let mut client = Client::connect("127.0.0.1:8542");
	client.push(id);
}

fn pull(id: &str) {
	let mut client = Client::connect("127.0.0.1:8542");
	client.pull(id);
}

fn main() {
	let config = Config::load();
	println!("{:?}", config);
	
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

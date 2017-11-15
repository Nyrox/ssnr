extern crate gitignore;
extern crate zip;
extern crate walkdir;
extern crate rand;

use self::rand::{ Rng, OsRng };

use std::io::{Cursor};
use std::io::prelude::*;
use std::net::{TcpStream};
use std::path::{Path};
use std::fs::File;
use std::fs;
use std::mem::{transmute};

use self::zip::write::FileOptions;

use api;
use api::{PullRequest, PushRequest};

pub struct Client<'a> {
	ignore: gitignore::File<'a>,
	pub conn: TcpStream
}

impl<'a> Client<'a> {
	pub fn connect(ip: &str) -> Client {
		Client { ignore: gitignore::File::new(Path::new(".gitignore")).unwrap(), conn: TcpStream::connect(ip).unwrap() }
	}
	
	pub fn push(&mut self, repo_name: &str) {
		println!("Pushing...");
		
		let buffer = Vec::<u8>::new();
		let mut cursor = Cursor::new(buffer);

		println!("{:?}, {:?}", repo_name.len(), repo_name.as_bytes());
		{
			let mut zip = zip::ZipWriter::new(&mut cursor);
			let options = FileOptions::default();
			
			let files = self.ignore.included_files().unwrap();
			
			for file in files.into_iter() {
				if file.is_file() {
					println!("Adding file: {:?}", file);
					zip.start_file(file.to_str().unwrap(), options).unwrap();
					let mut file = File::open(file).unwrap();
					let mut buffer = Vec::new();
					file.read_to_end(&mut buffer).unwrap();
					zip.write_all(&*buffer).unwrap();
				}
			}
			
			zip.finish().unwrap();
		}
		
		
		let request = PushRequest::new(repo_name, cursor.get_mut().to_vec());
		
		self.conn.write_all(&request.encode()).unwrap();
	}
	
	pub fn pull(&mut self, repo_name: &str) {
		println!("Pulling...");
		
		let request = PullRequest::new(repo_name);
		self.conn.write_all(&request.encode()).unwrap();
		
		let mut buffer = Vec::new();
		self.conn.read_to_end(&mut buffer).unwrap();
		
		let mut reader = Cursor::new(buffer);
		let mut zip = zip::ZipArchive::new(reader).unwrap();
		
		for i in 0..zip.len() {
			let mut file = zip.by_index(i).expect("File failure.");
			if let Some(sub) = file.name().rfind('\\') {
				fs::create_dir_all(file.name().split_at(sub).0);
			}
			println!("{}", file.name());
			let mut buffer = Vec::new();
			file.read_to_end(&mut buffer).expect("Reading file failed.");
			let mut diskFile = File::create(file.name()).expect("Creating file failed");
			diskFile.write_all(&buffer).expect("Writing to file failed.");			
		}

	}
}

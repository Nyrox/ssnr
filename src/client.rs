extern crate gitignore;
extern crate zip;
extern crate walkdir;
extern crate crypto;
extern crate rand;

use self::crypto::{ symmetriccipher, buffer, aes, blockmodes, scrypt, bcrypt };
use self::crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };
use self::crypto::scrypt::ScryptParams;

use self::rand::{ Rng, OsRng };

use std::io::{Cursor};
use std::io::prelude::*;
use std::net::{TcpStream};
use std::path::{Path};
use std::fs::File;
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
		
		// Write the header in
		/*
		Header Format:
		8bit command code
			42 => push
		
		32bit length of repo identifier
		
		Variable Length Repository Identifier
		*/
		let mut header = Vec::<u8>::new();
		header.write_all(&[42; 1]).unwrap();
		let id_len: [u8; 4] = unsafe { transmute(repo_name.len() as u32) };
		header.write_all(&id_len).unwrap();
		header.write_all(repo_name.as_bytes()).unwrap();
		
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
		
		
		let payload = api::EncryptedPayload::encrypt(cursor.get_mut(), "test");
		let request = PushRequest::new(repo_name, payload);
		
		self.conn.write_all(&request.encode()).unwrap();
	}
}

extern crate gitignore;
extern crate zip;
extern crate walkdir;
extern crate crypto;
extern crate rand;

use self::crypto::{ symmetriccipher, buffer, aes, blockmodes, scrypt };
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
		
		header.append(cursor.get_mut());
		println!("{}", header.len());
		
		let key: [u8; 32] = [0; 32];
		let iv: [u8; 16] = [0; 16];
		
		let params = ScryptParams::new(10, 8, 16);
		let s_key = scrypt::scrypt_simple("test", &params).unwrap();
		println!("{}, {:?}", s_key.len(), s_key);
		let encrypted = encrypt(&header, &key, &iv).unwrap();
		println!("{}", encrypted.len());
		
		self.conn.write_all(&encrypted).unwrap();
	}
}

#[allow(dead_code)]
fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
	
	let mut encryptor = aes::cbc_encryptor(
	aes::KeySize::KeySize256,
	key,
	iv,
	blockmodes::PkcsPadding);
	
	let mut final_result = Vec::<u8>::new();
	let mut read_buffer = buffer::RefReadBuffer::new(data);
	let mut buffer = [0; 4096];
	let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
	
	loop {
		let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));
		
		final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
		
		match result {
			BufferResult::BufferUnderflow => break,
			BufferResult::BufferOverflow => { }
		}
	}
	
	Ok(final_result)
}
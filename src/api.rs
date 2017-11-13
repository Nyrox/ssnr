extern crate crypto;

use self::crypto::{ bcrypt, symmetriccipher, buffer, aes, blockmodes};
use self::crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};

use std::io::prelude::*;
use std::mem::{transmute};

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Command {
	PULL = 42,
	PUSH = 33
}

pub struct CommandHeader {
	command: Command
}

impl CommandHeader {
	pub fn encode(&self) -> Vec<u8> {
		let mut output = Vec::<u8>::new();
		output.push(self.command as u8);
		return output;
	}
}

pub struct PullRequestHeader {
	
}

pub struct PullRequest {
	pub header: PullRequestHeader
}

pub struct PushRequestHeader {
	pub command_header: CommandHeader,
	id_len: u32,
	id: Vec<u8>
}

impl PushRequestHeader {
	pub fn new(repository: &str, command_header: CommandHeader) -> PushRequestHeader {
		PushRequestHeader {
			command_header,
			id_len: repository.len() as u32,
			id: Vec::from(repository.as_bytes())
		}
	}
	
	pub fn encode(&self) -> Vec<u8> {
		let mut output = Vec::new();
		output.write_all(&self.command_header.encode());
		let len_bytes: [u8; 4] = unsafe { transmute(self.id_len) };
		output.write_all(&len_bytes);
		output.write_all(&self.id);
		return output;
	}
}

pub struct PushRequest {
	pub header: PushRequestHeader,
	pub data: EncryptedPayload
}

impl PushRequest {
	pub fn new(repository: &str, data: EncryptedPayload) -> PushRequest {
		PushRequest {
			data,
			header: PushRequestHeader::new(repository, CommandHeader {
				command: Command::PUSH
			})
		}
	}
	
	pub fn encode(&self) -> Vec<u8> {
		let mut output = Vec::<u8>::new();
		output.write_all(&self.header.encode());
		output.write_all(&self.data.data);
		
		return output;
	}
}

pub struct EncryptedPayload {
	pub data: Vec<u8>
}

impl EncryptedPayload {
	pub fn encrypt(data: &[u8], password: &str) -> EncryptedPayload {
		let mut key = generate_binary_key(password, [0; 16]);
		let iv: [u8; 16] = [0; 16];
		
		EncryptedPayload { data: encrypt_binary_blob(&data, &key, &iv).unwrap() }
	}
}

fn generate_binary_key(password: &str, salt: [u8; 16]) -> [u8; 24] {
	let mut key = [0; 24];
	
	bcrypt::bcrypt(5, &salt, password.as_bytes(), &mut key);
	return key;
}

fn encrypt_binary_blob(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
	let mut encryptor = aes::cbc_encryptor(aes::KeySize::KeySize192, key, iv, blockmodes::PkcsPadding);
	
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

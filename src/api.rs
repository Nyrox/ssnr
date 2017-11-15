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
	pub command_header: CommandHeader,
	id_len: u32,
	id: Vec<u8>
}

impl PullRequestHeader {
	pub fn new(repository: &str, command_header: CommandHeader) -> PullRequestHeader {
		PullRequestHeader {
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

pub struct PullRequest {
	pub header: PullRequestHeader
}

impl PullRequest {
	pub fn new(repository: &str) -> PullRequest {
		PullRequest {
			header: PullRequestHeader::new(repository, CommandHeader {
				command: Command::PULL
			})
		}
	}
	
	pub fn encode(&self) -> Vec<u8> {
		let mut output = Vec::<u8>::new();
		output.write_all(&self.header.encode()).unwrap();
		
		return output;
	}
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
	pub data: Vec<u8>
}

impl PushRequest {
	pub fn new(repository: &str, data: Vec<u8>) -> PushRequest {
		PushRequest {
			data,
			header: PushRequestHeader::new(repository, CommandHeader {
				command: Command::PUSH
			})
		}
	}
	
	pub fn encode(&self) -> Vec<u8> {
		let mut output = Vec::<u8>::new();
		output.write_all(&self.header.encode()).unwrap();
		output.write_all(&self.data).unwrap();
		
		return output;
	}
}


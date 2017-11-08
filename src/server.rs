use std::io::prelude::*;
use std::net::{TcpListener};
use std::thread;
use std::mem::{transmute};
use std::fs::File;

extern crate zip;


// Will start a server in a seperate thread and return control back to the caller
#[allow(dead_code)]
pub fn start_server_local() -> thread::JoinHandle<()> {
	thread::spawn(|| {
		start_server();
	})
}

// Will start a server on the thread it is called, blocking execution
pub fn start_server() {
	let listener = TcpListener::bind("127.0.0.1:8542").unwrap();
	
	println!("Server started.");
	
	for stream in listener.incoming() {
		println!("Request");
		let mut stream = stream.unwrap();
		
		// Read the protocol header
		let mut cmd: [u8; 1] = [0; 1];
		stream.read_exact(&mut cmd).unwrap();
		let cmd = cmd[0];
		
		let mut id_len: [u8; 4] = [0; 4];
		stream.read_exact(&mut id_len).unwrap();
		let id_len: u32 = unsafe { transmute(id_len) };
		
		let mut id: Box<[u8]> = vec![0; id_len as usize].into_boxed_slice();
		stream.read_exact(&mut id).unwrap();
		println!("{:?}", id);
		let id = String::from_utf8_lossy(&*id).into_owned();
		
		println!("{}, {}, {}", cmd, id_len, id);
			
		let mut buffer = Vec::new();
		stream.read_to_end(&mut buffer).unwrap();
		
		println!("{}", buffer.len());
		
		let mut file = File::create(format!("data/{}.zip", id)).unwrap();
		file.write_all(&buffer).unwrap();
		
		// let mut zip = zip::ZipArchive::new(Cursor::new(buffer)).unwrap();
		// 
		// for i in 0..zip.len() {
		// 	let file = zip.by_index(i).unwrap();
		// 	println!("Filename: {}", file.name());
		// }
	}
}
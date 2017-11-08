default: build

build:
	cargo build --release
	./env.sh

server:
	target/debug/ssnr.exe serv
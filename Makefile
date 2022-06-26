.PHONY: default build run clean

default: clean build run

build:
	cargo build --release
	

clean:
	cargo clean

help:
	./target/release/tagparam -h

try:
	./target/release/tagparam - 3 pick-this-one

container:
	docker build -t tagparam:latest .

container-push:
	docker login
	docker tag tagparam:latest hunterkirk/tagparam:v0.3.7
	docker tag tagparam:latest hunterkirk/tagparam:latest
	docker push hunterkirk/tagparam:v0.3.7
	docker push hunterkirk/tagparam:latest

container-try:
	docker run -it tagparam:latest - 1 foo-bar-baz
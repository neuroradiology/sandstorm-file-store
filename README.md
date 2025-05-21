# sandstorm-file-store

## Description

This is a simple (but working) rust server that is intended to be used by developers who are packaging sandstorm apps.

It is used within a grain and accepts PUT requests from the sandstorm parent, and writes them to in-grain storage (which is at /var/data).

## Usage

Make sure **git** and **rustup** packages are installed on your system

> git clone https://github.com/neuroradiology/sandstorm-file-store

then...

> cd sandstorm-file-store
>
> cargo build
>
> cargo run

The program should run until you terminate it with ctl-C


Then test using curl:

> curl -X GET http://127.0.0.1:8080/ -d "Hello, this is a GET request!"

> curl -X POST http://127.0.0.1:8080/ -d "Hello, this is a POST request!"

> curl -X PUT http://127.0.0.1:8080/ -d "Hello, this is a PUT request!"

> curl -X DELETE http://127.0.0.1:8080/ -d "Hello, this is a DELETE request!"

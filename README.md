[![progress-banner](https://backend.codecrafters.io/progress/http-server/49558be1-0a0d-4e82-8787-1acb36e07f30)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

# Roostver: A Rust HTTP Server

This HTTP Server project is an extended version of the CodeCrafters HTTP Server 
challenge written in Rust. The original challenge is a learning exercise to 
implement a HTTP/1.1 server capable of serving multiple clients.

This version builds upon the initial challenge and adds more functionality. 
However, the primary purpose of this project is educational. It is intended 
for learning and understanding HTTP servers and the Rust programming language. 
While contributions for further improvement are welcomed, bear in mind that this
project is not deemed as a production-ready HTTP server.

[HTTP](https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol) is the
protocol that powers the web. In this challenge, you'll build a HTTP/1.1 server
that is capable of serving multiple clients.

## Prerequisites

To build and run the project, you need the following software on your machine:

- [Rust programming language](https://www.rust-lang.org/learn/get-started)
- Make (Comes preinstalled on Unix-like OSes. If not, check [GNU Make](https://www.gnu.org/software/make/) to install Make)

Ensure you have these installed before proceeding.

## Building the Project

The project comes with a Makefile for easy building.

Firstly, you need to modify the Makefile to set your `PROJECT_BUILD_PATH` and `PROJECT_NAME` accordingly.

Then run the following command

```sh
make
```

## Contributions and License

This project is open for contributions. Please feel free to submit Pull Requests to improve the functionality
of this HTTP server.

This project is licensed under GPL 3.0.
use std::error::Error;
use std::net::{TcpListener, TcpStream};
use std::thread;

use structopt::StructOpt;

use http::{HTTPReader, HTTPWriter};

mod http;

#[derive(Debug, Clone, StructOpt)]
struct Args {
    /// Set directory
    #[structopt(short = "d", long = "directory")]
    directory: Option<String>,
}

fn handle_client(stream: TcpStream, args: Args) -> Result<(), Box<dyn Error>> {
    let mut reader: HTTPReader = HTTPReader::new(&stream);
    let mut writer: HTTPWriter = HTTPWriter::new(&stream);

    println!("Accepted new connection");

    loop {
        let mut buffer = [0; 1]; // Create a buffer

        match stream.peek(&mut buffer) {
            Ok(0) => {
                println!("Connection closed.");
                break; // Exit the loop if connection closed
            }
            Ok(_) => {
                reader.read_request()?;
                
                println!("Data : {}", reader);

                let http_response = reader.route_request(args.directory.clone())?;

                writer.set_response(http_response);

                writer.write_response()?;

                // If connection is not kept alive, break loop
                if !reader.is_kept_alive() {
                    break;
                }
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }

    return Ok(());
}

fn main() {
    println!("Logs from your program will appear here!");

    let args = Args::from_args();

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let args = args.clone();
                thread::spawn(move || {
                    handle_client(stream, args.clone()).unwrap_or_else(|error| {
                        eprintln!("{:?}", error);
                    });
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::io::Write;
use std::env;
use std::error::Error;
use tokio;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let client_uri =
  env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;
    let mut contents = "Databases:\r\n".to_string();
    for name in client.list_database_names(None, None).await? {
       contents.push_str(&format!("- {}\r\n", name));
    }
   let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream, contents.clone());
    }
   Ok(())
}

fn handle_connection(mut stream: TcpStream, contents: String) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


use dotenv;
use std::env;
use space_viewer::shared::{DaemonRequest, DaemonResponse, NasaResultShort};
use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const SOCKET_PATH: &str = "/tmp/space_viewer.sock";
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // reads in all environment variables from .env file, block until done
    let api_key = env::var("API_KEY"); // nasa API key  

    match api_key {
        Ok(val) => println!("API successfully read"),
        Err(e) => println!("Error API_KEY: {}", e),
    }

    let _ = tokio::fs::remove_file(SOCKET_PATH).await; // attempts to free socket
    let listener = UnixListener::bind(SOCKET_PATH).unwrap(); // binds to socket once free
    println!("Successfully bound to socket at {SOCKET_PATH}");
    loop {
        match listener.accept().await { 
            Ok((stream, _addr)) => {
                println!("Connected to CLI");
            }
            Err(e) => { 
                panic!("Error in connecting to CLI");
            }
        }






    }
}

fn search(search_term : String) { // should return dynamic sized array of all terms relating to search_term


}
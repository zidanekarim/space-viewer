use dotenv;
use std::env;
use space_viewer::shared::{DaemonRequest, DaemonResponse, NasaResultShort};
use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const SOCKET_PATH: &str = "/tmp/space_viewer.sock";

fn main() {
    dotenv::dotenv().ok(); // reads in all environment variables from .env file
    let api_key = env::var("API_KEY"); // nasa API key  

    match api_key {
        Ok(val) => println!("API successfully read"),
        Err(e) => println!("Error API_KEY: {}", e),
    }

}

fn search(search_term : String) { // should return dynamic sized array of all terms relating to search_term


}
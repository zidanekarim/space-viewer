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
            Ok((mut stream, _addr)) => {
                println!("Connected to CLI");
                let task_api_key = api_key.clone();
                tokio::spawn(async move {

                    let mut buffer = [0; 4096]; // 4kb buffer
                    match stream.read(&mut buffer).await {
                        Ok(0) => {
                            println!("CLI Disconnected/Sent no info");
                        }
                        Ok(n) => {
                            println!("Successfully read {n} bytes");
                            // now need to deserialize with serde
                            let cleaned_buffer = &buffer[..n]; // buffer slices up to empty spots

                            match serde_json::from_slice::<DaemonRequest>(cleaned_buffer) { // parses JSON struct of socket data, and attempts to convert to DaemonRequest 
                                Ok(request) => { // now either Search or Download
                                    match request {
                                        DaemonRequest::Search {query} => {
                                            println!("Searching for {query}");
                                            search(query, task_api_key).await;
                                        }
                                        DaemonRequest::Download {nasa_id} => {
                                            return; // todo
                                        }
                                    }    
                                }
                                Err(e) => {
                                    eprintln!("Error converting socket data to DaemonRequest, with error {}", e);
                                }

                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from stream {}", e);
                        }
                    }
                });
                

            }

            
            Err(e) => { 
                panic!("Error in connecting to CLI");
            }
        }

       





    }
}

async fn search(search_term : String, api_key : &str) { // should return dynamic sized array of all terms relating to search_term


}
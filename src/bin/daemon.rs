use dotenv;
use std::env;
use space_viewer::shared::{DaemonRequest, DaemonResponse, NasaResultShort};
use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct NasaSearchResponse {
    collection: NasaCollection,
}

#[derive(Deserialize, Debug)]
struct NasaCollection {
    items: Vec<NasaItem>,
}

#[derive(Deserialize, Debug)]
struct NasaItem {
    data: Vec<NasaItemData>,
}

#[derive(Deserialize, Debug)]
struct NasaItemData {
    title: String,
    nasa_id: String,
    media_type: String,
}

const SOCKET_PATH: &str = "/tmp/space_viewer.sock";
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); // reads in all environment variables from .env file, block until done
    let api_key = match env::var("API_KEY") {
        Ok(val) => {
            println!("API successfully read");
            val 
        }
        Err(e) => {
            println!("Error API_KEY: {}", e);
            panic!("Fatal: API_KEY is required to start the daemon!"); 
        }
    };

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
                                            match search(&query, &task_api_key).await {
                                                Ok(search_result) => {
                                                    println!("Successfully searched");
                                                }
                                                Err(e) => {
                                                    eprintln!("Error searching for {} with error {}", &query, e);
                                                }
                                            }
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

            Err(_e) => { 
                panic!("Error in connecting to CLI");
            }
        }
    }
}

async fn search(search_term : &str, api_key : &str) ->  Result<Vec<NasaResultShort>, reqwest::Error> { // should return dynamic sized arvector of all terms relating to search_term
    let client = Client::new();
    const URL : &str = "https://images-api.nasa.gov/search";
    let response = client
        .get(URL)
        .query(&[
            ("q", search_term),
            ("media_type", "image,video"),
            ("api_key", api_key)
        ])
        .send().await?;
        
        let parsed_data: NasaSearchResponse = response.json().await?;

        let mut cleaned_results = Vec::new();

        for item in parsed_data.collection.items {
            if let Some(data) = item.data.into_iter().next() {
                cleaned_results.push(NasaResultShort {
                    title: data.title, // uses local struct to fill in shared NasaResultShort struct
                    nasa_id: data.nasa_id,
                    media_type: data.media_type,
                });
            }
        }

        return Ok(cleaned_results);
}

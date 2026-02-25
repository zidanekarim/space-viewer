use inquire::{Text, validator::{StringValidator, Validation}};
use reqwest::Client;
use space_viewer::shared::{DaemonRequest, DaemonResponse}; 
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

const SOCKET_PATH: &str = "/tmp/space_viewer.sock";
#[tokio::main] 
async fn main() {
    let validator = |input: &str| if input.chars().count() > 140 || input.trim().is_empty() {
        Ok(Validation::Invalid("You're only allowed 140 characters, with a minimum of 1 character.".into()))
    } else {
        Ok(Validation::Valid)
    };

    println!("NASA Image & Video Repository");
    let message = Text::new("Enter your search term: ")
        .with_validator(validator)

        .prompt();

    let search_term = match message {
        Ok(search_term) => search_term,
        Err(err) => {
            eprintln!("Error while processing your search term: {}", err);
            return;
        }
    };

    // connect to Daemon through socket
    let mut connection = match UnixStream::connect(SOCKET_PATH).await {
        Ok(connected) => connected,
        Err(err) => {
            eprintln!("Error connecting to socket from main: {}", err);
            return;
        }
    };

    let request = DaemonRequest::Search {query : search_term.clone()};

    let serialized_request = match serde_json::to_vec(&request) { // convert request into serialized JSON bytes
        Ok(ser) => ser,
        Err(err) => {
            eprintln!("Error serializing raw request: {}", err);
            return;
        }

    };
    match connection.write_all(&serialized_request).await {
        Ok(_) => {
            println!("Searching for {search_term}...");
        }
        Err(e) => {
            eprintln!("Failed to send data to daemon: {}", e);
            return;
        }
    }
    let mut response_buffer = Vec::new();
    match connection.read_to_end(&mut response_buffer).await {
        Ok(0) => {
            eprintln!("No search results read");
            return;
        }
        Ok(n)=> {
            println!("Read {n} bytes!");
            match serde_json::from_slice::<DaemonResponse>(&response_buffer) {
                Ok(DaemonResponse::SearchResults {items: results }) => {
                    println!("Successfully parsed {} NASA results!", results.len());
                }
                Ok(_) => {
                    eprintln!("Received unexpected response type from daemon.");
                }
                Err(e) => {
                    eprintln!("Failed to parse daemon response: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error in receving search results, {}", e);
            return;
        }
    }

    
}
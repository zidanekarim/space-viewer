use inquire::{Text, validator::{StringValidator, Validation}};
use reqwest::Client;
use space_viewer::shared::{DaemonRequest, DaemonResponse}; 
use tokio::io::AsyncWriteExt;
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
    
}
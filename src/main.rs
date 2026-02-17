use inquire::{Text, validator::{StringValidator, Validation}};
use reqwest::Client;

fn main() {
    let validator = |input: &str| if input.chars().count() > 140 {
        Ok(Validation::Invalid("You're only allowed 140 characters.".into()))
    } else {
        Ok(Validation::Valid)
    };

    println!("Hello!");
    let message = Text::new("Enter your search term")
        .with_validator(validator)

        .prompt();

    match message {
        Ok(message) => println!("You searched for {}", message),
        Err(err) => println!("Error while publishing your status: {}", err),
    }
}
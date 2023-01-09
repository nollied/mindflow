use std::env;
use std::fs;
use std::path::Path;
use dialoguer::{theme::ColorfulTheme ,Input};

// Set authorization token used to authenticate with the Mindflow server
pub async fn set_token(auth_key: Option<String>) {
    let token = match auth_key {
        Some(token) => {
            token
        } None => {
            match Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Authorization token: ")
                .interact_text() {
                    Ok(token) => token,
                    Err(e) => {
                        println!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
        }
    };

    let home_dir = match env::var("HOME") {
        Ok(home_dir) => home_dir,
        Err(e) => {
            println!("Unable to find home directory in ENV: {}", e);
            std::process::exit(1);
        }
    };

    let path = Path::new(&home_dir).join(".mindflow");

    let result = fs::write(path, token).map_err(|e| e.to_string());

    match result {
        Ok(_) => {
            println!("Successfully authorized with token");
        }
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
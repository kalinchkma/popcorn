use std::process;


pub struct Config {
    pub port: u16,
    pub host: String
}

impl Config {
    // Create new server configuration from environtment variables
    pub fn from_env() -> Self {
        use dotenvy::dotenv;
        use std::env;

        // Intialized dotenv to load environtment variables from .env file
        dotenv().ok();

        let port = env::var("PORT").unwrap_or_else(|_e| {
            println!("Server port must be specify on environtment variables");
            process::exit(0)
        }).parse::<u16>().unwrap_or_else(|_| {
            println!("Server post must be a number");
            process::exit(0)
        });
        let host = env::var("HOST").unwrap_or_else(|_| {
            println!("Server host address must be set");
            process::exit(0)
        });

        Config {
            port,
            host
        }
    }

    // Create server configuration from arguments
    pub fn new(port: u16, host: String) -> Self {
        Config {
            port,
            host
        }
    }
}
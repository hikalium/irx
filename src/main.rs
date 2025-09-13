use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::env;

#[derive(Parser)]
#[command(name = "irx")]
#[command(about = "A CLI for Nature Remo API", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all devices
    List,
}

#[derive(Deserialize, Debug)]
struct Device {
    name: String,
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List => {
            let token = env::var("NATURE_REMO_TOKEN")
                .expect("NATURE_REMO_TOKEN environment variable not set");

            let client = reqwest::Client::new();
            let devices = client
                .get("https://api.nature.global/1/devices")
                .bearer_auth(token)
                .send()
                .await?
                .json::<Vec<Device>>()
                .await?;

            for device in devices {
                println!("- {} (ID: {})", device.name, device.id);
            }
        }
    }

    Ok(())
}

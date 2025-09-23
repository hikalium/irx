use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use serde::Deserialize;
use std::collections::HashMap;
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
    /// List all devices and their appliances
    List,
    /// Show the state of an appliance
    Show {
        #[arg(long)]
        appliance: String,
    },
    /// Set the state of an appliance
    Set {
        #[arg(long)]
        appliance: String,

        #[arg(long)]
        temperature: Option<String>,
    },
}

#[derive(Deserialize, Debug, Clone)]
struct Device {
    name: String,
    id: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Appliance {
    id: String,
    nickname: String,
    device: Device,
    #[serde(rename = "type")]
    appliance_type: String,
    settings: Option<AcSettings>,
}

#[derive(Deserialize, Debug, Clone)]
struct AcSettings {
    temp: String,
    mode: String,
    vol: String,
    dir: String,
    button: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    let token =
        env::var("NATURE_REMO_TOKEN").expect("NATURE_REMO_TOKEN environment variable not set");
    let client = reqwest::Client::new();

    match &cli.command {
        Commands::List => {
            let devices_future = client
                .get("https://api.nature.global/1/devices")
                .bearer_auth(token.clone())
                .send();

            let appliances_future = client
                .get("https://api.nature.global/1/appliances")
                .bearer_auth(token)
                .send();

            let (devices_res, appliances_res) = tokio::join!(devices_future, appliances_future);

            let devices = devices_res?.json::<Vec<Device>>().await?;
            let appliances = appliances_res?.json::<Vec<Appliance>>().await?;

            let mut appliances_by_device: HashMap<String, Vec<Appliance>> = HashMap::new();
            for appliance in appliances {
                appliances_by_device
                    .entry(appliance.device.id.clone())
                    .or_default()
                    .push(appliance);
            }

            for device in devices {
                println!("- {} (ID: {})", device.name, device.id);
                if let Some(device_appliances) = appliances_by_device.get(&device.id) {
                    for appliance in device_appliances {
                        println!("  - {} (ID: {})", appliance.nickname, appliance.id);
                    }
                }
            }
        }
        Commands::Show { appliance } => {
            let appliances = client
                .get("https://api.nature.global/1/appliances")
                .bearer_auth(token)
                .send()
                .await?
                .json::<Vec<Appliance>>()
                .await?;

            let target_appliance = appliances.iter().find(|a| a.id == *appliance);

            if let Some(app) = target_appliance {
                println!("State for {} (ID: {})", app.nickname, app.id);
                println!("Type: {}", app.appliance_type);

                match app.appliance_type.as_str() {
                    "AC" => {
                        if let Some(settings) = &app.settings {
                            println!("  Temperature: {}", settings.temp);
                            println!("  Mode: {}", settings.mode);
                            println!("  Volume: {}", settings.vol);
                            println!("  Direction: {}", settings.dir);
                            println!("  Button: {}", settings.button);
                        } else {
                            println!("  No settings found for this AC.");
                        }
                    }
                    _ => {
                        println!(
                            "  Detailed state for '{}' is not yet supported.",
                            app.appliance_type
                        );
                    }
                }
            } else {
                eprintln!("Appliance with ID '{}' not found.", appliance);
            }
        }
        Commands::Set {
            appliance,
            temperature,
        } => {
            let appliances = client
                .get("https://api.nature.global/1/appliances")
                .bearer_auth(token.clone())
                .send()
                .await?
                .json::<Vec<Appliance>>()
                .await?;

            let target_appliance = appliances.iter().find(|a| a.id == *appliance).cloned();

            if let Some(app) = target_appliance {
                if app.appliance_type != "AC" {
                    eprintln!(
                        "Error: Appliance '{}' is not an Air Conditioner.",
                        app.nickname
                    );
                    return Ok(());
                }

                let mut form_data = HashMap::new();
                if let Some(temp) = temperature {
                    form_data.insert("temperature", temp.clone());
                }

                if form_data.is_empty() {
                    println!("No settings to change.");
                    return Ok(());
                }

                let url = format!(
                    "https://api.nature.global/1/appliances/{}/aircon_settings",
                    appliance
                );
                let res = client
                    .post(&url)
                    .bearer_auth(token)
                    .form(&form_data)
                    .send()
                    .await?;

                if res.status().is_success() {
                    println!("Successfully updated appliance '{}'.", app.nickname);
                } else {
                    eprintln!(
                        "Error updating appliance '{}': {}",
                        app.nickname,
                        res.status()
                    );
                    eprintln!("{}", res.text().await?);
                }
            } else {
                eprintln!("Appliance with ID '{}' not found.", appliance);
            }
        }
    }

    Ok(())
}

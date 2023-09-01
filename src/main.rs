use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{mpsc, Arc, Mutex},
};

use actix_web::{web::Data, App, HttpServer};
use alarm_endpoint::{alarm, disable_alarm};
use clap::Parser;
use config::generate_default_config;
use once_cell::sync::Lazy;

mod alarm_endpoint;
mod alarm_responses;
mod config;

pub static CHANNEL_STORE: Lazy<Arc<Mutex<HashMap<u32, mpsc::Sender<()>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[derive(clap_derive::Parser)]
struct Args {
    ///Path to the config file
    #[arg(short, long, default_value = "./config.yaml")]
    config_path: PathBuf,
    ///If set will generate the default config file at the provided config path
    #[arg(short, long)]
    generate_config: bool,
}

#[actix_web::main()]
async fn main() {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    if !args.config_path.exists() {
        if args.generate_config {
            generate_default_config(&args.config_path).expect("Unable to generate default config");
        } else {
            log::error!("No config file");
            return;
        }
    }
    let config = config::prarse_config(args.config_path.into()).unwrap();
    let c1 = config.clone();

    HttpServer::new(move || {
        App::new()
            .service(alarm)
            .service(disable_alarm)
            .app_data(Data::new(c1.clone()))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((config.ip_address, config.port))
    .unwrap()
    .run()
    .await
    .unwrap();
}

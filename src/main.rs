use std::{
    collections::HashMap,
    sync::{mpsc, Arc, Mutex},
};

use actix_web::{web::Data, App, HttpServer};
use alarm_endpoint::{alarm, disable_alarm};
use once_cell::sync::Lazy;

mod alarm_endpoint;
mod alarm_responses;
mod config;

pub static CHANNEL_STORE: Lazy<Arc<Mutex<HashMap<u32, mpsc::Sender<()>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let ip_address = std::env::var("IP_ADDRESS").expect("No ip address provided");
    let port = std::env::var("PORT")
        .expect("No port provided")
        .parse()
        .expect("Failed to prase port");
    let api_key = std::env::var("API_KEY").expect("No api key porvided");

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    HttpServer::new(move || {
        App::new()
            .service(alarm)
            .service(disable_alarm)
            .app_data(Data::new(api_key.clone()))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind((ip_address, port))?
    .run()
    .await
}

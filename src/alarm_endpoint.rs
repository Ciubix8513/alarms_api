use std::{sync::mpsc, thread};

use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use log::{info, warn};
use rand::RngCore;
use serde_derive::{Deserialize, Serialize};

use crate::{
    alarm_responses::test_alarm::logging_allarm,
    config::{Config, Severity},
    CHANNEL_STORE,
};

#[derive(Debug, Serialize, Deserialize)]
/// Struct for the alarm endpoint
pub struct AlarmRequest {
    pub api_key: String,
    pub host_id: String,
    pub failure_status: Severity,
    pub failure_cause: String,
}

#[post("/alarm")]
pub async fn alarm(payload: Json<AlarmRequest>, config: Data<Config>) -> impl Responder {
    if config.api_key != payload.api_key {
        return HttpResponse::Unauthorized();
    }

    let (tx, rx) = mpsc::channel::<()>();
    let mut rng = rand::thread_rng();

    let mut alarm_id = rng.next_u32();

    let mut map = CHANNEL_STORE.lock().unwrap();

    loop {
        if map.contains_key(&alarm_id) {
            alarm_id = rng.next_u32();
        } else {
            break;
        }
    }

    map.insert(alarm_id, tx);

    if let Some(item) = config.hosts.iter().find(|i| i.name == payload.host_id) {
        // if let Some(response).
    }

    // warn!("{}", map.contains_key(&alarm_id));

    match payload.failure_status {
        FailureStatus::Warn => {
            log::warn!(
                "WARN FROM {}, CAUSE {}\n ALARM ID {alarm_id}",
                payload.host_id,
                payload.failure_cause
            );
            thread::spawn(move || {
                while rx.try_recv().is_err() {
                    logging_allarm("TEST");
                }
            });
        }
        FailureStatus::Error => log::error!(
            "ERROR FROM {}, CAUSE {}",
            payload.host_id,
            payload.failure_cause,
        ),
    }

    HttpResponse::Ok()
}

#[derive(Debug, Deserialize)]
pub struct AlarmId {
    pub api_key: String,
    pub id: u32,
}

#[post("/disable_alarm")]
pub async fn disable_alarm(payload: Json<AlarmId>, api_key: Data<String>) -> impl Responder {
    if api_key.into_inner() != payload.api_key.clone().into() {
        return HttpResponse::Unauthorized();
    }

    let mut map = CHANNEL_STORE.lock().unwrap();

    for i in map.iter() {
        info!("{:?}", i);
    }

    return if let Some(channel) = map.remove(&payload.id) {
        channel.send(()).unwrap();

        HttpResponse::Ok()
    } else {
        warn!("Wrong id ");
        HttpResponse::BadRequest()
    };
}

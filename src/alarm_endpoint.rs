#![allow(clippy::unused_async)]
use crate::{
    alarm_responses,
    config::{AlarmResponseTypes, Config, Severity},
    CHANNEL_STORE,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use rand::RngCore;
use serde_derive::{Deserialize, Serialize};
use std::{sync::mpsc, thread};

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Struct for the alarm endpoint
pub struct AlarmRequest {
    pub api_key: String,
    pub host_id: String,
    pub severity: Severity,
    pub failure_cause: String,
}

#[post("/alarm")]
pub async fn alarm(payload: Json<AlarmRequest>, config: Data<Config>) -> impl Responder {
    if config.api_key != payload.api_key {
        return HttpResponse::Unauthorized();
    }

    let mut rng = rand::thread_rng();

    let mut alarm_id = rng.next_u32();

    let mut map = CHANNEL_STORE.lock().expect("POISON ERROR FUCK!");
    loop {
        if map.contains_key(&alarm_id) {
            alarm_id = rng.next_u32();
        } else {
            break;
        }
    }

    alarm_responses::log::alarm(
        &payload.host_id,
        &payload.failure_cause,
        &payload.severity,
        &format!("alarm id: {alarm_id}"),
    );

    //Find the host
    if let Some(item) = config.hosts.iter().find(|i| i.name == payload.host_id) {
        //Find first config with the same severity
        for i in &item.responses {
            if i.severity != payload.severity {
                continue;
            }

            i.repeating.map_or_else(
                || match &i.response {
                    AlarmResponseTypes::Sound(f) => {
                        let tmp = f.clone();
                        _ = thread::spawn(move || {
                            if let Err(e) = alarm_responses::sounds::alarm(&tmp) {
                                log::error!("Audio alarm error: {e}");
                            }
                        });
                    }
                    AlarmResponseTypes::Log(msg) => alarm_responses::log::alarm(
                        &payload.host_id,
                        &payload.failure_cause,
                        &i.severity,
                        msg,
                    ),
                    AlarmResponseTypes::File(p) => alarm_responses::file::alarm(p),
                },
                |t| {
                    let (tx, rx) = mpsc::channel::<()>();
                    map.insert(alarm_id, tx);
                    let tmp = i.clone();
                    let tmp1 = payload.clone();
                    thread::spawn(move || {
                        while rx.try_recv().is_err() {
                            match &tmp.response {
                                AlarmResponseTypes::Sound(f) => {
                                    if let Err(e) = alarm_responses::sounds::alarm(f) {
                                        log::error!("Audio alarm error: {e}");
                                    }
                                }
                                AlarmResponseTypes::Log(msg) => alarm_responses::log::alarm(
                                    &tmp1.host_id,
                                    &tmp1.failure_cause,
                                    &tmp.severity,
                                    msg,
                                ),
                                AlarmResponseTypes::File(p) => alarm_responses::file::alarm(p),
                            }
                            thread::sleep(t);
                        }
                    });
                },
            );
            break;
        }
    } else {
        log::warn!("Could not find host {}", payload.host_id);
    }
    HttpResponse::Ok()
}

#[derive(Debug, Deserialize)]
pub struct AlarmId {
    pub api_key: String,
    pub id: u32,
}

#[post("/disable_alarm")]
pub async fn disable_alarm(payload: Json<AlarmId>, config: Data<Config>) -> impl Responder {
    if config.api_key != payload.api_key {
        return HttpResponse::Unauthorized();
    }

    let mut map = CHANNEL_STORE.lock().expect("POISON ERROR FUCK!");

    map.remove(&payload.id).map_or_else(
        || {
            log::warn!("Wrong id ");
            HttpResponse::BadRequest()
        },
        |channel| {
            if let Err(e) = channel.send(()) {
                log::error!("Error sending {}", e);
                HttpResponse::InternalServerError()
            } else {
                HttpResponse::Ok()
            }
        },
    )
}

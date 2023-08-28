use std::sync::Mutex;

use actix_web::{get, post, web::{self, Bytes}, App, HttpResponse, HttpServer, Responder};
use prost::Message;

pub mod rstation {
    include!(concat!(env!("OUT_DIR"), "/rstation.rs"));
}

struct MeasurementsState {
    set: Mutex<rstation::MeasurementSet>
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/weather/measurements")]
async fn post_measurements(req_body: Bytes, data: web::Data<MeasurementsState>) -> impl Responder {
    let m = rstation::MeasurementSet::decode(req_body).unwrap();
    let mut state = data.set.lock().unwrap();
    state.measurements.extend_from_slice(&m.measurements[..]);
    HttpResponse::Ok().body(format!("Got {}, Total {}", m.measurements.len(), state.measurements.len()))
}

#[get("/weather/measurements")]
async fn get_measurements(data: web::Data<MeasurementsState>) -> impl Responder {
    let state = data.set.lock().unwrap();
    let body = state.encode_to_vec();
    HttpResponse::Ok().body(body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let measurements_state = web::Data::new(MeasurementsState{
        set: Mutex::default()
    });
    HttpServer::new(move || {
        App::new()
            .app_data(measurements_state.clone())
            .service(hello)
            .service(get_measurements)
            .service(post_measurements)
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

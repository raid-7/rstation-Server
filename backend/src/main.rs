use r_station::db;
use r_station::rstation;

use std::ops::Bound;

use actix_web::{get, post, web::{self, Bytes}, App, HttpResponse, HttpServer};
use actix_files::Files;
use prost::Message;
use itertools::Itertools;
use serde::Deserialize;


fn prune_measurements(measurements: Vec<rstation::Measurement>, limit: usize) -> Vec<rstation::Measurement> {
    let len = measurements.len();
    if len <= limit {
        return measurements;
    }
    let measurements = measurements.into_iter()
        .into_group_map_by(|m| m.sensor.clone());
    let mut res = Vec::new();
    let skip_div = (len + limit - 1usize) / limit;
    let got_from_skip_div = (len - 1) / skip_div + 1;
    let rest = limit - got_from_skip_div;
    for (_, sensor_measurements) in measurements {
        for (i, m) in sensor_measurements.into_iter().enumerate() {
            if i % skip_div == 0 ||
                i % skip_div == (skip_div + 1) / 2 && i >= (len - skip_div * rest) {
                res.push(m);
            }
        }
    }
    res
}


#[derive(Deserialize)]
struct GetMeasurementsQuery {
    sensor: Option<String>,
    from: Option<u64>,
    to: Option<u64>,
    max_num_points_per_sensor: Option<usize>
}

fn fetch_measurements(db: &db::DbState, query: &GetMeasurementsQuery) -> std::io::Result<Vec<rstation::Measurement>> {
    let res = if query.from.is_some() || query.to.is_some() {
        let ms = db::fetch_measurements_by_ts(db, (
            query.from.map_or(Bound::Unbounded, |t| Bound::Included(t)),
            query.to.map_or(Bound::Unbounded, |t| Bound::Included(t))
        ))?;
        match &query.sensor {
            Some(prefix) => ms.into_iter()
                .filter(|m| m.sensor.starts_with(prefix))
                .collect(),
            None => ms
        }
    } else if query.sensor.is_some() {
        db::fetch_measurements_by_sensor(db, query.sensor.as_ref().unwrap())?
    } else {
        db::fetch_all_measurements(db)?
    };
    let res = if let Some(limit) = query.max_num_points_per_sensor {
        prune_measurements(res, limit)
    } else {
        res
    };
    Ok(res)
}

#[post("/api/measurements")]
async fn post_measurements(req_body: Bytes, data: web::Data<db::DbState>) -> Result<HttpResponse, std::io::Error> {
    let ms = rstation::MeasurementSet::decode(req_body)?;
    let sz = ms.measurements.len();
    db::insert_measurements(ms.measurements, &*data)?;
    Ok(HttpResponse::Ok().body(format!("Got {}", sz)))
}

#[get("/api/measurements")]
async fn get_measurements(data: web::Data<db::DbState>, query: web::Query<GetMeasurementsQuery>) -> Result<HttpResponse, std::io::Error> {
    let response = rstation::MeasurementSet {
        measurements: fetch_measurements(&*data, &*query)?
    };
    let body = response.encode_to_vec();
    Ok(
        HttpResponse::Ok()
            .content_type("application/protobuf")
            .body(body)
    )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = std::env::var("APP_DB_PATH").unwrap_or("test-db".to_owned());
    let port = std::env::var("APP_PORT")
        .map_or(8080, |s| s.parse::<u16>().unwrap());
    let num_workers = std::env::var("APP_WORKERS")
        .map_or(2, |s| s.parse::<usize>().unwrap());

    let db = sled::open(db_path).unwrap();
    let db_state = web::Data::new(db::DbState::new(&db).unwrap());
    HttpServer::new(move || {
        App::new()
            .app_data(db_state.clone())
            .service(get_measurements)
            .service(post_measurements)
            .service(Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .workers(num_workers)
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

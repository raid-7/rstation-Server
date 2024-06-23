use r_station::db;
use r_station::rstation;

use std::ops::Bound;

use actix_web::{get, post, web::{self, Bytes, Header}, http, App, HttpResponse, HttpServer};
use actix_files::Files;
use prost::Message;
use itertools::Itertools;
use serde::Deserialize;
use chrono;


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
    max_num_points: Option<usize>
}

#[derive(Clone)]
struct XAuthorizedDevice {
    authorized: bool
}

impl http::header::TryIntoHeaderValue for XAuthorizedDevice {
    type Error = actix_web::http::header::InvalidHeaderValue;

    fn try_into_value(self) -> Result<http::header::HeaderValue, Self::Error> {
        Ok(http::header::HeaderValue::from_static(
            if self.authorized {
                "1"
            } else {
                "0"
            }
        ))
    }
}

impl http::header::Header for XAuthorizedDevice {
    fn name() -> http::header::HeaderName {
        http::header::HeaderName::from_static("x-authorized-device")
    }

    fn parse<M: actix_web::HttpMessage>(msg: &M) -> Result<Self, actix_web::error::ParseError> {
        match msg.headers().get(Self::name()) {
            Some(value) => {
                match value.to_str() {
                    Ok(str_value) =>
                        Some(str_value.eq_ignore_ascii_case("true")
                        || str_value.eq_ignore_ascii_case("1")),
                    Err(_) => None
                }
            },
            None => Some(false)
        }.map_or(Err(actix_web::error::ParseError::Header), 
            |v| Ok(XAuthorizedDevice { authorized: v }))
    }
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
    let res = if let Some(limit) = query.max_num_points {
        prune_measurements(res, limit)
    } else {
        res
    };
    Ok(res)
}

#[post("/api/measurements")]
async fn post_measurements(req_body: Bytes, data: web::Data<db::DbState>,
        authorized_device: Header<XAuthorizedDevice>) -> Result<HttpResponse, std::io::Error> {
    if !authorized_device.authorized {
        return Ok(HttpResponse::Forbidden().finish())
    }
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

fn prune_measurements_in_database(db: &db::DbState) {
    let offset = chrono::Duration::days(30);
    let gap = chrono::Duration::minutes(10);

    let now = chrono::Utc::now();
    let time_to_end = now - offset;
    let ts_to_end = time_to_end.timestamp_micros().max(0) as u64;
    let time_to_end_str = time_to_end.format("%d.%m.%Y %H:%M:%S").to_string();
    let min_time_between_points = gap.num_microseconds().unwrap() as u64;

    println!("Start pruning measurements until {time_to_end_str} UTC...");
    let result = db::prune_measurements(db, ts_to_end, min_time_between_points, 2048);
    let status = if let Err(err) = result {
        format!("failure: {}", err.to_string())
    } else {
        "success".to_owned()
    };
    println!("Finished pruning measurements: {status}");
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = std::env::var("APP_DB_PATH").unwrap_or("test-db".to_owned());
    let port = std::env::var("APP_PORT")
        .map_or(8080, |s| s.parse::<u16>().unwrap());
    let num_workers = std::env::var("APP_WORKERS")
        .map_or(2, |s| s.parse::<usize>().unwrap());

    let db = sled::Config::default()
        .path(db_path)
        .cache_capacity(128*1024*1024)
        .open()
        .unwrap();
    let db_state = web::Data::new(db::DbState::new(&db).unwrap());
    
    let db_state_to_prune = db_state.clone();
    let prune_measurements_thread = std::thread::spawn(move || {
        prune_measurements_in_database(db_state_to_prune.as_ref());
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_state.clone())
            .service(get_measurements)
            .service(post_measurements)
            .service(Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .workers(num_workers)
    .bind(("0.0.0.0", port))?
    .run();

    prune_measurements_thread.join().unwrap();

    server.await
}

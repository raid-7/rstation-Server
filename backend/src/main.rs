use std::{ops::RangeBounds, ops::Bound};

use actix_web::{get, post, web::{self, Bytes}, App, HttpResponse, HttpServer};
use prost::Message;
use serde::Deserialize;

pub mod rstation {
    include!(concat!(env!("OUT_DIR"), "/rstation.rs"));
}

fn get_ts_prefixed_key(m: &rstation::Measurement) -> Vec<u8> {
    let mut res = Vec::with_capacity(8 + m.sensor.len());
    res.extend(m.timestamp_us.to_be_bytes());
    res.extend(m.sensor.as_bytes());
    res
}

fn get_sensor_prefixed_key(m: &rstation::Measurement) -> Vec<u8> {
    let mut res = Vec::with_capacity(9 + m.sensor.len());
    res.extend(m.sensor.as_bytes());
    res.push(0);
    res.extend(m.timestamp_us.to_be_bytes());
    res
}

struct DbState {
    ts_prefixed_tree: sled::Tree,
    sensor_prefixed_tree: sled::Tree
}

fn db_insert_measurements(measurements: &Vec<rstation::Measurement>, db: &DbState) -> std::io::Result<()> {
    let mut ts_batch = sled::Batch::default();
    let mut sensor_batch = sled::Batch::default();
    let mut buf = Vec::new();

    for m in measurements {
        if m.timestamp_us == 0 {
            continue;
        }
        buf.clear();
        m.encode(&mut buf).unwrap();
        ts_batch.insert(get_ts_prefixed_key(m), &buf[..]);
        sensor_batch.insert(get_sensor_prefixed_key(m), &buf[..]);
    }

    db.ts_prefixed_tree.apply_batch(ts_batch)?;
    db.sensor_prefixed_tree.apply_batch(sensor_batch)?;
    Ok(())
}

fn ts_bound_to_key(bound: Bound<&u64>, end: bool) -> Bound<[u8; 8]> {
    match bound {
        Bound::Included(v) => if end {
            Bound::Excluded((v + 1).to_be_bytes())
        } else {
            Bound::Included(v.to_be_bytes())
        },
        Bound::Excluded(v) => Bound::Excluded(v.to_be_bytes()),
        Bound::Unbounded => Bound::Unbounded
    }
}

fn db_fetch_measurements_by_ts(db: &DbState, ts: impl RangeBounds<u64>) -> sled::Iter {
    let begin_buf = ts_bound_to_key(ts.start_bound(), false);
    let end_buf = ts_bound_to_key(ts.end_bound(), true);
    db.ts_prefixed_tree.range((begin_buf, end_buf))
}

fn db_fetch_measurements_by_sensor(db: &DbState, sensor_prefix: &String) -> sled::Iter {
    db.sensor_prefixed_tree.scan_prefix(sensor_prefix.as_bytes())
}

fn db_iter_to_vec(iter: sled::Iter) -> std::io::Result<Vec<rstation::Measurement>> {
    let mut res = Vec::new();
    for m_res in iter.values() {
        let m_enc = m_res?;
        res.push(rstation::Measurement::decode(&m_enc[..])?);
    }
    Ok(res)
}


#[derive(Deserialize)]
struct GetMeasurementsQuery {
    sensor: Option<String>,
    from: Option<u64>,
    to: Option<u64>,
}

fn fetch_measurements(db: &DbState, query: &GetMeasurementsQuery) -> std::io::Result<Vec<rstation::Measurement>> {
    let res = if query.from.is_some() || query.to.is_some() {
        let iter = db_fetch_measurements_by_ts(db, (
            query.from.map_or(Bound::Unbounded, |t| Bound::Included(t)),
            query.to.map_or(Bound::Unbounded, |t| Bound::Included(t))
        ));
        let ms = db_iter_to_vec(iter)?;
        match &query.sensor {
            Some(prefix) => ms.into_iter()
                .filter(|m| m.sensor.starts_with(prefix))
                .collect(),
            None => ms
        }
    } else if query.sensor.is_some() {
        db_iter_to_vec(db_fetch_measurements_by_sensor(db, query.sensor.as_ref().unwrap()))?
    } else {
        db_iter_to_vec(db.ts_prefixed_tree.iter())?
    };
    Ok(res)
}

#[post("/api/measurements")]
async fn post_measurements(req_body: Bytes, data: web::Data<DbState>) -> Result<HttpResponse, std::io::Error> {
    let ms = rstation::MeasurementSet::decode(req_body)?;
    db_insert_measurements(&ms.measurements, &*data)?;
    Ok(HttpResponse::Ok().body(format!("Got {}", ms.measurements.len())))
}

#[get("/api/measurements")]
async fn get_measurements(data: web::Data<DbState>, query: web::Query<GetMeasurementsQuery>) -> Result<HttpResponse, std::io::Error> {
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
    let db_state = web::Data::new(DbState{
        ts_prefixed_tree: db.open_tree("measurements-timestamp-prefixed").unwrap(),
        sensor_prefixed_tree: db.open_tree("measurements-sensor-prefixed").unwrap(),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(db_state.clone())
            .service(get_measurements)
            .service(post_measurements)
    })
    .workers(num_workers)
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

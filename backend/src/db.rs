use crate::rstation;
use std::{ops::RangeBounds, ops::Bound};
use prost::Message;

const MAX_POINTS_TO_PROCESS: usize = 200_000_000;

#[derive(Default, Debug)]
pub struct KeyDecodingError;

impl std::error::Error for KeyDecodingError {}

impl std::fmt::Display for KeyDecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "key decoding error")
    }
}

impl From<std::str::Utf8Error> for KeyDecodingError {
    fn from(_: std::str::Utf8Error) -> Self {
        return Self::default();
    }
}

fn get_ts_prefixed_key(m: &rstation::Measurement) -> Vec<u8> {
    let mut res = Vec::with_capacity(8 + m.sensor.len());
    res.extend(m.timestamp_us.to_be_bytes());
    res.extend(m.sensor.as_bytes());
    res
}

fn decode_ts_prefixed_key(key: &sled::IVec) -> Result<(u64, String), KeyDecodingError> {
    if key.len() < 8 {
        return Err(KeyDecodingError::default());
    }
    let ts = u64::from_be_bytes(key[0..8].try_into().unwrap());
    let sensor = std::str::from_utf8(&key[8..])?.to_owned();
    Ok((ts, sensor))
}

fn get_sensor_prefixed_key(m: &rstation::Measurement) -> Vec<u8> {
    let mut res = Vec::with_capacity(9 + m.sensor.len());
    res.extend(m.sensor.as_bytes());
    res.push(0);
    res.extend(m.timestamp_us.to_be_bytes());
    res
}

fn decode_sensor_prefixed_key(key: &sled::IVec) -> Result<(u64, String), KeyDecodingError> {
    let mut i = 0;
    while i < key.len() && key[i] != 0 {
        i += 1;
    }
    if i + 9 > key.len() {
        return Err(KeyDecodingError::default());
    }

    let ts = u64::from_be_bytes(key[(i + 1)..(i + 9)].try_into().unwrap());
    let sensor = std::str::from_utf8(&key[..i])?.to_owned();
    Ok((ts, sensor))
}

pub struct DbState {
    ts_prefixed_tree: sled::Tree,
    sensor_prefixed_tree: sled::Tree
}

impl DbState {
    pub fn new(db: &sled::Db) -> std::io::Result<Self> {
        Ok(Self{
            ts_prefixed_tree: db.open_tree("measurements-timestamp-prefixed")?,
            sensor_prefixed_tree: db.open_tree("measurements-sensor-prefixed")?,
        })
    }
}

pub fn insert_measurements(measurements: Vec<rstation::Measurement>, db: &DbState) -> std::io::Result<()> {
    let mut ts_batch = sled::Batch::default();
    let mut sensor_batch = sled::Batch::default();
    let mut buf = Vec::new();

    for mut m in measurements {
        if m.timestamp_us == 0 {
            continue;
        }
        let ts_key = get_ts_prefixed_key(&m);
        let sensor_key = get_sensor_prefixed_key(&m);
        buf.clear();
        m.timestamp_us = 0;
        m.sensor = String::default();
        m.encode(&mut buf).unwrap();
        ts_batch.insert(ts_key, &buf[..]);
        sensor_batch.insert(sensor_key, &buf[..]);
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

pub fn fetch_measurements_by_ts(db: &DbState, ts: impl RangeBounds<u64>) -> std::io::Result<Vec<rstation::Measurement>> {
    let begin_buf = ts_bound_to_key(ts.start_bound(), false);
    let end_buf = ts_bound_to_key(ts.end_bound(), true);
    iter_to_vec(db.ts_prefixed_tree.range((begin_buf, end_buf)), true)
}

pub fn fetch_measurements_by_sensor(db: &DbState, sensor_prefix: &String) -> std::io::Result<Vec<rstation::Measurement>> {
    iter_to_vec(db.sensor_prefixed_tree.scan_prefix(sensor_prefix.as_bytes()), false)
}

pub fn fetch_all_measurements(db: &DbState) -> std::io::Result<Vec<rstation::Measurement>> {
    iter_to_vec(db.ts_prefixed_tree.iter(), true)
}

fn iter_to_vec(iter: sled::Iter, ts_prefixed: bool) -> std::io::Result<Vec<rstation::Measurement>> {
    let mut res = Vec::new();
    for m_res in iter {
        let (m_key, m_enc) = m_res?;
        let mut m = rstation::Measurement::decode(&m_enc[..])?;
        let decoded_key = if ts_prefixed {
            decode_ts_prefixed_key(&m_key)
        } else {
            decode_sensor_prefixed_key(&m_key)
        };
        match decoded_key {
            Ok((ts, sensor)) => {
                m.timestamp_us = ts;
                m.sensor = sensor;
                res.push(m);
            },
            Err(err) => {
                println!("Invalid measurement in DB: {}", err);
                continue;
            }
        }
        if res.len() >= MAX_POINTS_TO_PROCESS {
            break;
        }
    }
    Ok(res)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ts_prefixed_key() {
        let m = rstation::Measurement{
            sensor: "sensor".to_owned(),
            timestamp_us: 123456789098u64,
            value: 1.233242
        };
        let key = sled::IVec::from(get_ts_prefixed_key(&m));
        let (ts, sensor) = decode_ts_prefixed_key(&key).unwrap();
        assert_eq!(m.timestamp_us, ts);
        assert_eq!(m.sensor, sensor);
    }

    #[test]
    fn sensor_prefixed_key() {
        let m = rstation::Measurement{
            sensor: "sensor".to_owned(),
            timestamp_us: 123456789098u64,
            value: 1.233242
        };
        let key = sled::IVec::from(get_sensor_prefixed_key(&m));
        let (ts, sensor) = decode_sensor_prefixed_key(&key).unwrap();
        assert_eq!(m.timestamp_us, ts);
        assert_eq!(m.sensor, sensor);
    }
}
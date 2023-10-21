pub mod db;
pub mod device_logs;

pub mod rstation {
    include!(concat!(env!("OUT_DIR"), "/rstation.rs"));
}

pub mod db;

pub mod rstation {
    include!(concat!(env!("OUT_DIR"), "/rstation.rs"));
}

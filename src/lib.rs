mod __priv {
    include!(concat!(env!("OUT_DIR"), "/terraform.rs"));
}

pub use __priv::docker::*;

use serde::{ Serialize, Deserialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataPoint {
    topo: Topology,
    pub spreadProfilePrices: Vec<SpreadProfilePrice>,
    ts: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Topology {
    platform: String,
    server: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpreadProfilePrice {
    spreadProfile: String,
    bidSpread: f64,
    askSpread: f64,
    bid: f64,
    pub ask: f64,
}

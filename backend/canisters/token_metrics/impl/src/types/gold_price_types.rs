use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct YumiApiResponse {
    code: u32,
    msg: String,
    pub data: Vec<GoldData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoldData {
    #[serde(rename = "_id")]
    id: String,
    symbols: String,
    unit: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "tradeAt")]
    trade_at: String,
    pub price: f64,
    #[serde(rename = "__v")]
    v: u32,
}

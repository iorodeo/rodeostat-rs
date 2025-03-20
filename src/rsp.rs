use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Success {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rsp<T> {
    pub success: bool,
    pub response: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamRsp<T> {
    pub success: bool,
    pub response: SetParam<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMsg {
    pub message: String,
}

// ----------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVariant {
    pub command: String,
    pub variant: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTestNames {
    pub command: String,
    pub test_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVolt {
    pub command: String,
    pub v: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetVolt {
    pub command: String,
    pub v: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCurr {
    pub command: String,
    pub i: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRefVolt {
    pub command: String,
    pub r: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetParam<T> {
    pub command: String,
    pub test: String,
    pub param: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetParam<T> {
    pub command: String,
    pub test: String,
    pub param: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetVoltRange {
    pub command: String,
    pub volt_range: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetVoltRange {
    pub command: String,
    pub volt_range: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrRange {
    pub command: String,
    pub curr_range: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetCurrRange {
    pub command: String,
    pub curr_range: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetDeviceId {
    pub command: String,
    pub device_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceId {
    pub command: String,
    pub device_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVersion {
    pub command: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAllElectConn {
    pub command: String,
    pub connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllElectConn {
    pub command: String,
    pub connected: bool,
}


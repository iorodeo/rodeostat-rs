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
    pub response: Param<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMsg {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variant {
    pub command: String,
    pub variant: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TestNames {
    pub command: String,
    pub test_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Volt {
    pub command: String,
    pub v: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Curr {
    pub command: String,
    pub i: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefVolt {
    pub command: String,
    pub r: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Param<T> {
    pub command: String,
    pub test: String,
    pub param: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VoltRange {
    pub command: String,
    pub volt_range: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrRange {
    pub command: String,
    pub curr_range: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceId {
    pub command: String,
    pub device_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SamplePeriod {
    pub command: String,
    pub sample_period: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TestDoneTime {
    pub command: String,
    pub test_done_time: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub command: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllElectConn {
    pub command: String,
    pub connected: bool,
}


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Success {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMsg {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVariantRsp {
    pub command: String,
    pub variant: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVariant {
    pub success: bool,
    pub response: GetVariantRsp,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTestNamesRsp {
    pub command: String,
    pub test_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTestNames {
    pub success: bool,
    pub response: GetTestNamesRsp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVoltRsp {
    pub command: String,
    pub v: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVolt {
    pub success: bool,
    pub response: GetVoltRsp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetVoltRsp {
    pub command: String,
    pub v: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetVolt {
    pub success: bool,
    pub response: SetVoltRsp,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrRsp {
    pub command: String,
    pub i: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCurr {
    pub success: bool,
    pub response: GetCurrRsp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRefVoltRsp {
    pub command: String,
    pub r: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRefVolt {
    pub success: bool,
    pub response: GetRefVoltRsp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetParam<T> {
    pub command: String,
    pub test: String,
    pub param: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetParamRsp<T> {
    pub success: bool,
    pub response: GetParam<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetParam<T> {
    pub command: String,
    pub test: String,
    pub param: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetParamRsp<T> {
    pub success: bool,
    pub response: SetParam<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVersionRsp {
    pub command: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVersion {
    pub success: bool,
    pub response: GetVersionRsp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAllElectConnRsp {
    pub command: String,
    pub connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAllElectConn {
    success: bool,
    pub response: SetAllElectConnRsp,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllElectConnRsp {
    pub command: String,
    pub connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllElectConn {
    success: bool,
    pub response: GetAllElectConnRsp,
}

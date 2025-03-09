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
pub struct GetCyclicParamRsp {
    pub command: String,
    pub test: String,
    pub param: crate::param::CyclicParam,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCyclicParam {
    pub success: bool, 
    pub response: GetCyclicParamRsp,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetSinusoidParamRsp {
    pub command: String,
    pub test: String,
    pub param: crate::param::SinusoidParam,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSinusoidParam {
    pub success: bool, 
    pub response: GetSinusoidParamRsp,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetConstantParamRsp {
    pub command: String,
    pub test: String,
    pub param: crate::param::ConstantParam,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetConstantParam {
    pub success: bool, 
    pub response: GetConstantParamRsp,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetSquareWaveParamRsp {
    pub command: String,
    pub test: String,
    pub param: crate::param::SquareWaveParam,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLinearSweepParam {
    pub success: bool, 
    pub response: GetLinearSweepParamRsp,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetLinearSweepParamRsp {
    pub command: String,
    pub test: String,
    pub param: crate::param::LinearSweepParam,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSquareWaveParam {
    pub success: bool, 
    pub response: GetSquareWaveParamRsp,
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




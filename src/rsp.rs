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
pub struct GetVariantPrm {
    pub command: String,
    pub variant: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVariant {
    pub success: bool,
    pub response: GetVariantPrm,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTestNamesPrm {
    pub command: String,
    pub test_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTestNames {
    pub success: bool,
    pub response: GetTestNamesPrm,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetVoltPrm {
    pub command: String,
    pub v: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVolt {
    pub success: bool,
    pub response: GetVoltPrm,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SetVoltPrm {
    pub command: String, 
    pub v: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetVolt {
    pub success: bool, 
    pub response: SetVoltPrm,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetCurrPrm {
    pub command: String, 
    pub i: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCurr {
    pub success: bool, 
    pub response: GetCurrPrm,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetRefVoltPrm {
    pub command: String,
    pub r: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetRefVolt {
    pub success: bool,
    pub response: GetRefVoltPrm,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetVersionPrm {
    pub command: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVersion {
    pub success: bool,
    pub response: GetVersionPrm,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SetAllElectConnPrm {
    pub command: String, 
    pub connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAllElectConn {
    success: bool, 
    pub response: SetAllElectConnPrm,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllElectConnPrm {
    pub command: String, 
    pub connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllElectConn {
    success: bool, 
    pub response: GetAllElectConnPrm,
}




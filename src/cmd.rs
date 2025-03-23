use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NoArgCmd<'a> {
    pub command: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetVolt<'a> {
    pub command: &'a str,
    pub v: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAllElectConn<'a> {
    pub command: &'a str,
    pub connected: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetParam<'a> {
    pub command: &'a str,
    pub test: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetParam<'a, T>
where
    T: crate::param::TestParam + Serialize,
{
    pub command: &'a str,
    pub test: &'a str,
    pub param: T,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetVoltRange<'a> {
    pub command: &'a str,
    pub volt_range: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetCurrRange<'a> {
    pub command: &'a str,
    pub curr_range: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetDeviceId<'a> {
    pub command: &'a str,
    pub device_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetSamplePeriod<'a> {
    pub command: &'a str,
    pub sample_period: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTestDoneTime<'a> {
    pub command: &'a str,
    pub test: &'a str,
}



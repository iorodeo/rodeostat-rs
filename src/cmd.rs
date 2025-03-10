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
    T: crate::param::TestParam + Serialize
{
    pub command: &'a str,
    pub test: &'a str,
    pub param: T,
}

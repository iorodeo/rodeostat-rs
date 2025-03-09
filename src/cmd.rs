use serde::{Deserialize, Serialize};
//use crate::constant;

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

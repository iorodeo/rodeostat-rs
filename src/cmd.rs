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

//pub const GET_TEST_NAMES: NoArgCmd = NoArgCmd {
//    command: constant::GET_TEST_NAMES_STR,
//};
//
//
//pub const GET_VERSION: NoArgCmd = NoArgCmd {
//    command: constant::GET_VERSION_STR,
//};

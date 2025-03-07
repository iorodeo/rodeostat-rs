use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CyclicParam {
    pub quiet_value: f32, 
    pub quiet_time: f32, 
    pub amplitude: f32, 
    pub offset: f32, 
    pub shift: f32,
    pub period: u32,
    pub num_cycles: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SinusoidParam {
    pub quiet_value: f32, 
    pub quiet_time: f32, 
    pub amplitude: f32, 
    pub offset: f32, 
    pub shift: f32,
    pub period: u32, 
    pub num_cycles: u32, 
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConstantParam {
    pub quiet_value: f32, 
    pub quiet_time: f32, 
    pub value: f32, 
    pub duration: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SquareWaveParam {
    pub quiet_value: f32, 
    pub quiet_time: f32, 
    pub start_value: f32,
    pub final_value: f32,
    pub step_value: f32,
    pub amplitude: f32, 
    pub window: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinearSweepParam {
    pub quiet_value: f32, 
    pub quiet_time: f32, 
    pub start_value: f32,
    pub final_value: f32,
    pub duration: u32,
}

//#[derive(Serialize, Deserialize, Debug)]
//#[serde(rename_all = "camelCase")]
//pub struct Chronoamp {
//    //{'quietValue': 0, 'quietTime': 0, 'step': [[1000, 0.5], [1000, -0.5]]}
//    pub quiet_value: f32, 
//    pub quiet_time: f32, 
//    pub step: Vec<Vec<f32>>,
//}

use serde::{Deserialize, Serialize};

pub trait TestParam {
    fn name() -> &'static str {
        crate::constant::DUMMY_TEST_STR
    }
}

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
impl TestParam for CyclicParam {
    fn name() -> &'static str {
        crate::constant::CYCLIC_TEST_STR
    }
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
impl TestParam for SinusoidParam {
    fn name() -> &'static str {
        crate::constant::SINUSOID_TEST_STR
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConstantParam {
    pub quiet_value: f32,
    pub quiet_time: f32,
    pub value: f32,
    pub duration: f32,
}
impl TestParam for ConstantParam {
    fn name() -> &'static str {
        crate::constant::CONSTANT_TEST_STR
    }
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
impl TestParam for SquareWaveParam {
    fn name() -> &'static str {
        crate::constant::SQUARE_WAVE_TEST_STR
    }
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
impl TestParam for LinearSweepParam {
    fn name() -> &'static str {
        crate::constant::LINEAR_SWEEP_TEST_STR
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChronoampParam {
    pub quiet_value: f32,
    pub quiet_time: f32,
    pub step: [(u32, f32); 2],
}
impl TestParam for ChronoampParam {
    fn name() -> &'static str {
        crate::constant::CHRONOAMP_TEST_STR
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MultistepParam {
    pub quiet_value: f32,
    pub quiet_time: f32,
    pub step: Vec<(u32, f32)>,
}
impl TestParam for MultistepParam {
    fn name() -> &'static str {
        crate::constant::MULTISTEP_TEST_STR
    }
}

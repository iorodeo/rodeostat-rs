use anyhow::anyhow;
use serde_json::Value as JsonValue;
use serialport::SerialPort;
use std::io::{BufRead, BufReader};
use std::time::Duration;

mod cmd;
mod constant;
mod rsp;
mod param;

#[derive(Debug)]
pub struct Rodeostat {
    pub port: Box<dyn SerialPort>,
}

impl Rodeostat {
    pub fn new(name: &str) -> anyhow::Result<Rodeostat> {
        let port = serialport::new(name, constant::SERIAL_BAUDRATE)
            .timeout(Duration::from_millis(constant::SERIAL_TIMEOUT))
            .open()?;
        Ok(Rodeostat { port })
    }

    pub fn get_hardware_variant(&mut self) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VARIANT_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetVariant = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.variant)
    }

    pub fn get_firmware_version(&mut self) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VERSION_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetVersion = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.version)
    }


    pub fn get_test_names(&mut self) -> anyhow::Result<Vec<String>> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_TEST_NAMES_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetTestNames = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.test_names)
    }

    pub fn get_volt(&mut self) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VOLT_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetVolt = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.v)
    }

    pub fn set_volt(&mut self, volt: f32) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::SetVolt {
            command: constant::SET_VOLT_STR,
            v: volt,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::SetVolt = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.v)
    }

    pub fn get_curr(&mut self) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_CURR_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetCurr = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.i)
    }

    pub fn get_ref_volt(&mut self) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_REF_VOLT_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetRefVolt = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.r)
    }

    pub fn get_cyclic_param(&mut self) -> anyhow::Result<param::CyclicParam> {
        let cmd_json = serde_json::to_value(&cmd::GetParam {
            command: constant::GET_PARAM_STR, 
            test: constant::CYCLIC_TEST_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetCyclicParam = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.param)
    }

    pub fn get_sinusoid_param(&mut self) -> anyhow::Result<param::SinusoidParam> {
        let cmd_json = serde_json::to_value(&cmd::GetParam {
            command: constant::GET_PARAM_STR, 
            test: constant::SINUSOID_TEST_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetSinusoidParam = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.param)
    }

    pub fn get_constant_param(&mut self) -> anyhow::Result<param::ConstantParam> {
        let cmd_json = serde_json::to_value(&cmd::GetParam {
            command: constant::GET_PARAM_STR,
            test: constant::CONSTANT_TEST_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetConstantParam = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.param)
    }

    pub fn get_square_wave_param(&mut self) -> anyhow::Result<param::SquareWaveParam> {
        let cmd_json = serde_json::to_value(&cmd::GetParam {
            command: constant::GET_PARAM_STR, 
            test: constant::SQUARE_WAVE_TEST_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetSquareWaveParam = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.param)
    }

    pub fn get_linear_sweep_param(&mut self) -> anyhow::Result<param::LinearSweepParam> {
        let cmd_json = serde_json::to_value(&cmd::GetParam {
            command: constant::GET_PARAM_STR, 
            test: constant::LINEAR_SWEEP_TEST_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetLinearSweepParam = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.param)
    }

    pub fn get_chronoamp_param(&mut self) -> anyhow::Result<param::ChronoampParam> {
        let cmd_json = serde_json::to_value(&cmd::GetParam {
            command: constant::GET_PARAM_STR,
            test: constant::CHRONOAMP_TEST_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetChronoampParam = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.param)
    }

    pub fn set_all_elect_connected(&mut self, value: bool) -> anyhow::Result<bool> {
        let cmd_json = serde_json::to_value(&cmd::SetAllElectConn {
            command: constant::SET_ALL_ELECT_CONNECTED_STR,
            connected: value, 
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::SetAllElectConn = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.connected)
    }

    pub fn get_all_elect_connected(&mut self) -> anyhow::Result<bool> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_ALL_ELECT_CONNECTED_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetAllElectConn = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.connected)
    }


    // ----------------------------------------------------------------------------

    pub fn write_json(&mut self, cmd_json: &JsonValue) -> anyhow::Result<()> {
        let mut cmd_bytes = serde_json::to_vec(&cmd_json)?;
        // Deal with itsy bitsy M4 serial bug which causes issues when message
        // length is an integer multiple of 64.
        if (cmd_bytes.len() + 2) % 64 == 0 {
            cmd_bytes.push(' ' as u8);
        }
        cmd_bytes.push('\r' as u8);
        cmd_bytes.push('\n' as u8);
        self.port.write(&cmd_bytes)?;
        self.port.flush()?;
        Ok(())
    }

    pub fn read_rsp(&mut self) -> anyhow::Result<String> {
        let mut rsp_string = String::new();
        let mut reader = BufReader::new(&mut self.port);
        reader.read_line(&mut rsp_string)?;
        let rsp::Success { success } = serde_json::from_str(&rsp_string)?;
        if success {
            Ok(rsp_string)
        } else {
            let rsp_struct: rsp::ErrorMsg = serde_json::from_str(&rsp_string)?;
            Err(anyhow!(rsp_struct.message))
        }
    }

    pub fn write_json_read_rsp(&mut self, cmd_json: &JsonValue) -> anyhow::Result<String> {
        self.write_json(&cmd_json)?;
        let rsp_string = self.read_rsp()?;
        Ok(rsp_string)
    }
}

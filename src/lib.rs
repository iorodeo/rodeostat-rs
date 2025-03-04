use anyhow::anyhow;
use serde_json::Value as JsonValue;
use serialport::SerialPort;
use std::io::{BufRead, BufReader};
use std::time::Duration;

mod cmd;
mod constant;
mod rsp;

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

    pub fn get_variant(&mut self) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VARIANT_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetVariant = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.variant)
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

    pub fn get_version(&mut self) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VERSION_STR,
        })?;
        let rsp_string = self.write_json_read_rsp(&cmd_json)?;
        let rsp_struct: rsp::GetVersion = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct.response.version)
    }

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

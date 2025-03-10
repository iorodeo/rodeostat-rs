mod cmd;
mod constant;
pub mod param;
mod rsp;

use anyhow::anyhow;
use serde::{de, ser};
use serde_json::Value as JsonValue;
use serialport::SerialPort;
use std::io::{BufRead, BufReader};
use std::time::Duration;

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
        let rsp_struct: rsp::GetVariant = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.variant)
    }

    pub fn get_firmware_version(&mut self) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VERSION_STR,
        })?;
        let rsp_struct: rsp::GetVersion = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.version)
    }

    pub fn get_test_names(&mut self) -> anyhow::Result<Vec<String>> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_TEST_NAMES_STR,
        })?;
        let rsp_struct: rsp::GetTestNames = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.test_names)
    }

    pub fn get_volt(&mut self) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VOLT_STR,
        })?;
        let rsp_struct: rsp::GetVolt = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.v)
    }

    pub fn set_volt(&mut self, volt: f32) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::SetVolt {
            command: constant::SET_VOLT_STR,
            v: volt,
        })?;
        let rsp_struct: rsp::SetVolt = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.v)
    }

    pub fn get_curr(&mut self) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_CURR_STR,
        })?;
        let rsp_struct: rsp::GetCurr = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.i)
    }

    pub fn get_ref_volt(&mut self) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_REF_VOLT_STR,
        })?;
        let rsp_struct: rsp::GetRefVolt = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.r)
    }

    pub fn get_param<T>(&mut self) -> anyhow::Result<T>
    where
        T: param::TestParam + de::DeserializeOwned,
    {
        let cmd_json = serde_json::to_value(&cmd::GetParam {
            command: constant::GET_PARAM_STR,
            test: T::name(),
        })?;
        let rsp_struct: rsp::GetParamRsp<T> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.param)
    }

    pub fn set_param<T>(&mut self, param: T) -> anyhow::Result<T>
    where
        T: param::TestParam + ser::Serialize + de::DeserializeOwned,
    {
        let cmd_json = serde_json::to_value(&cmd::SetParam {
            command: constant::SET_PARAM_STR,
            test: T::name(),
            param: param,
        })?;
        let rsp_struct: rsp::SetParamRsp<T> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.param)
    }

    pub fn set_all_elect_connected(&mut self, value: bool) -> anyhow::Result<bool> {
        let cmd_json = serde_json::to_value(&cmd::SetAllElectConn {
            command: constant::SET_ALL_ELECT_CONNECTED_STR,
            connected: value,
        })?;
        let rsp_struct: rsp::SetAllElectConn = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.connected)
    }

    pub fn get_all_elect_connected(&mut self) -> anyhow::Result<bool> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_ALL_ELECT_CONNECTED_STR,
        })?;
        let rsp_struct: rsp::GetAllElectConn = self.write_json_read_rsp(&cmd_json)?;
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

    pub fn write_json_read_rsp_string(&mut self, cmd_json: &JsonValue) -> anyhow::Result<String> {
        self.write_json(&cmd_json)?;
        let rsp_string = self.read_rsp()?;
        Ok(rsp_string)
    }

    pub fn write_json_read_rsp<T>(&mut self, cmd_json: &JsonValue) -> anyhow::Result<T>
    where
        T: de::DeserializeOwned,
    {
        let rsp_string = self.write_json_read_rsp_string(cmd_json)?;
        let rsp_struct: T = serde_json::from_str(&rsp_string)?;
        Ok(rsp_struct)
    }
}

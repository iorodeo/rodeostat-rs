mod cmd;
mod constant;
pub mod param;
mod rsp;

use rsp::{Rsp, ParamRsp};
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
        let rsp_struct: Rsp<rsp::Variant> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.variant)
    }


    pub fn get_volt(&mut self) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VOLT_STR,
        })?;
        let rsp_struct: Rsp<rsp::Volt> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.v)
    }


    pub fn set_volt(&mut self, volt: f32) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::SetVolt {
            command: constant::SET_VOLT_STR,
            v: volt,
        })?;
        let rsp_struct: Rsp<rsp::Volt> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.v)
    }


    pub fn get_curr(&mut self) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_CURR_STR,
        })?;
        let rsp_struct: Rsp<rsp::Curr> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.i)
    }


    pub fn get_ref_volt(&mut self) -> anyhow::Result<f32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_REF_VOLT_STR,
        })?;
        let rsp_struct: Rsp<rsp::RefVolt> = self.write_json_read_rsp(&cmd_json)?;
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
        let rsp_struct: ParamRsp<T> = self.write_json_read_rsp(&cmd_json)?;
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
        let rsp_struct: ParamRsp<T> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.param)
    }


    pub fn get_volt_range(&mut self) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VOLT_RANGE_STR,
        })?;
        let rsp_struct: Rsp<rsp::VoltRange> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.volt_range)
    }


    pub fn set_volt_range(&mut self, volt_range: &str) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::SetVoltRange {
            command: constant::SET_VOLT_RANGE_STR,
            volt_range: volt_range,
        })?;
        let rsp_struct: Rsp<rsp::VoltRange> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.volt_range)
    }


    pub fn get_all_volt_range(&mut self) -> anyhow::Result<Vec<String>> {
        let hardware_variant = self.get_hardware_variant()?;
        if hardware_variant.contains("AD8251") {
            Ok(to_vec_string(&constant::VOLT_RANGES_8V))
        } else {
            Ok(to_vec_string(&constant::VOLT_RANGES_10V))
        }
    }


    pub fn get_curr_range(&mut self) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_CURR_RANGE_STR,
        })?;
        let rsp_struct: Rsp<rsp::CurrRange> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.curr_range)
    }


    pub fn set_curr_range(&mut self, curr_range: &str) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::SetCurrRange {
            command: constant::SET_CURR_RANGE_STR,
            curr_range: curr_range,
        })?;
        let rsp_struct: Rsp<rsp::CurrRange> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.curr_range)
    }


    pub fn get_all_curr_range(&mut self) -> anyhow::Result<Vec<String>> {
        let hardware_variant = self.get_hardware_variant()?;
        match hardware_variant {
            val if val.contains("nano") => Ok(to_vec_string(&constant::CURR_RANGES_NANO)),
            val if val.contains("micro") => Ok(to_vec_string(&constant::CURR_RANGES_MICRO)),
            val if val.contains("milli") => Ok(to_vec_string(&constant::CURR_RANGES_MILLI_10)),
            val if val.contains("10Milli") => Ok(to_vec_string(&constant::CURR_RANGES_MILLI_10)),
            val if val.contains("24Milli") => Ok(to_vec_string(&constant::CURR_RANGES_MILLI_24)),
            _ => Err(anyhow!("unknown hardware variant")),
        }
    }


    pub fn get_device_id(&mut self) -> anyhow::Result<u32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_DEVICE_ID_STR,
        })?;
        let rsp_struct: Rsp<rsp::DeviceId> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.device_id)
    }


    pub fn set_device_id(&mut self, device_id: u32) -> anyhow::Result<u32> {
        let cmd_json = serde_json::to_value(&cmd::SetDeviceId {
            command: constant::SET_DEVICE_ID_STR,
            device_id: device_id,
        })?;
        let rsp_struct: Rsp<rsp::DeviceId> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.device_id)
    }


    pub fn get_sample_period(&mut self) -> anyhow::Result<u32> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_SAMPLE_PERIOD_STR, 
        })?;
        let rsp_struct: Rsp<rsp::SamplePeriod> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.sample_period)
    }


    pub fn set_sample_period(&mut self, sample_period: u32) -> anyhow::Result<u32> {
        let cmd_json = serde_json::to_value(&cmd::SetSamplePeriod {
            command: constant::SET_SAMPLE_PERIOD_STR, 
            sample_period: sample_period,
        })?;
        let rsp_struct: Rsp<rsp::SamplePeriod> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.sample_period)
    }


    pub fn get_sample_rate(&mut self) -> anyhow::Result<f32> {
        let sample_period = self.get_sample_period()? as f32;
        Ok(1.0f32/ms_to_s(sample_period))
    }


    pub fn set_sample_rate(&mut self, sample_rate: f32) -> anyhow::Result<f32> {
        let sample_period = s_to_ms(1.0f32/sample_rate) as u32;
        let sample_period = self.set_sample_period(sample_period)? as f32;
        Ok(1.0f32/ms_to_s(sample_period))
    }


    pub fn get_test_done_time(&mut self, test: &str) -> anyhow::Result<u32> {
        let cmd_json = serde_json::to_value(&cmd::GetTestDoneTime {
            command: constant::GET_TEST_DONE_TIME_STR, 
            test: test,
        })?;
        let rsp_struct: Rsp<rsp::TestDoneTime> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.test_done_time)
    }


    pub fn get_test_names(&mut self) -> anyhow::Result<Vec<String>> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_TEST_NAMES_STR,
        })?;
        let rsp_struct: Rsp<rsp::TestNames> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.test_names)
    }


    pub fn get_firmware_version(&mut self) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_VERSION_STR,
        })?;
        let rsp_struct: Rsp<rsp::Version> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.version)
    }

    pub fn get_hardware_version(&mut self) -> anyhow::Result<String> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_HARDWARE_VERSION_STR,
        })?;
        let rsp_struct: Rsp<rsp::Version> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.version)
    }

    pub fn set_all_elect_connected(&mut self, value: bool) -> anyhow::Result<bool> {
        let cmd_json = serde_json::to_value(&cmd::SetAllElectConn {
            command: constant::SET_ALL_ELECT_CONNECTED_STR,
            connected: value,
        })?;
        let rsp_struct: Rsp<rsp::AllElectConn> = self.write_json_read_rsp(&cmd_json)?;
        Ok(rsp_struct.response.connected)
    }


    pub fn get_all_elect_connected(&mut self) -> anyhow::Result<bool> {
        let cmd_json = serde_json::to_value(&cmd::NoArgCmd {
            command: constant::GET_ALL_ELECT_CONNECTED_STR,
        })?;
        let rsp_struct: Rsp<rsp::AllElectConn> = self.write_json_read_rsp(&cmd_json)?;
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


fn to_vec_string(str_array: &[&str]) -> Vec<String> {
    let string: Vec<String> = str_array.iter().map(|v| v.to_string()).collect();
    string
}

fn ms_to_s(val: f32) -> f32 {
    val*1.0e-3f32
}

fn s_to_ms(val: f32) -> f32 {
    val*1.0e3f32
}

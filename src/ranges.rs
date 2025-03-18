use anyhow::anyhow;

pub enum VoltRange {
    Range1V,
    Range2V,
    Range4V,
    Range5V,
    Range8V,
    Range10V,
}

impl VoltRange {
    fn to_string(range: VoltRange) -> String {
        match range {
            VoltRange::Range1V => "1V".to_string(),
            VoltRange::Range2V => "2V".to_string(),
            VoltRange::Range4V => "4V".to_string(),
            VoltRange::Range5V => "5V".to_string(),
            VoltRange::Range8V => "8V".to_string(),
            VoltRange::Range10V => "10V".to_string(),
        }
    }

    fn from_str(range: &str) -> anyhow::Result<VoltRange> {
        match range {
            "1V" => Ok(VoltRange::Range1V),
            "2V" => Ok(VoltRange::Range2V),
            "4V" => Ok(VoltRange::Range4V),
            "5V" => Ok(VoltRange::Range5V),
            "8V" => Ok(VoltRange::Range8V),
            "10V" => Ok(VoltRange::Range10V), 
            _ => Err(anyhow!(format!("unknown voltage range {}",range))),
        }
    }
}


pub enum CurrRange {
    Range60nA, 
    Range100nA, 
    Range1uA,
    Range10uA, 
    Range100uA, 
    Range1000uA, 
    Range10000uA, 
    Range12000uA, 
    Range24000uA, 
}

impl CurrRange {
    fn to_string(range: CurrRange) -> String {
        match range {
            CurrRange::Range60nA => "60nA".to_string(), 
            CurrRange::Range100nA => "100nA".to_string(), 
            CurrRange::Range1uA => "1uA".to_string(), 
            CurrRange::Range10uA => "10uA".to_string(), 
            CurrRange::Range100uA => "100uA".to_string(), 
            CurrRange::Range1000uA => "1000uA".to_string(), 
            CurrRange::Range10000uA => "10000uA".to_string(), 
            CurrRange::Range12000uA => "12000uA".to_string(), 
            CurrRange::Range24000uA => "24000uA".to_string(), 
        }
    }

    fn from_str(range: &str) -> anyhow::Result<CurrRange> {
        match range {
            "60nA" => Ok(CurrRange::Range60nA),
            "100nA" => Ok(CurrRange::Range100nA),
            "1uA" => Ok(CurrRange::Range1uA),
            "10uA" => Ok(CurrRange::Range10uA),
            "100uA" => Ok(CurrRange::Range100uA),
            "1000uA" => Ok(CurrRange::Range1000uA),
            "10000uA" => Ok(CurrRange::Range10000uA),
            "12000uA" => Ok(CurrRange::Range12000uA),
            "24000uA" => Ok(CurrRange::Range24000uA),
            _ => Err(anyhow!(format!("unknown current range {}",range))),
        }
    }
}


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
            VoltRange::Range1V  => "1V".to_string(),
            VoltRange::Range2V  => "2V".to_string(),
            VoltRange::Range4V  => "4V".to_string(),
            VoltRange::Range5V  => "5V".to_string(),
            VoltRange::Range8V  => "8V".to_string(),
            VoltRange::Range10V => "10V".to_string(),
        }
    }

    //fn from_str(range: &str) -> VoltRange {
    //}
}


pub enum CurrRange {
    Range60nA, 
    Range100nA, 
    Range1uA,
    Range10uA, 
    Range100uA, 
    Range1mA, 
    Range10mA, 
    Range12mA, 
    Range24mA, 
}

impl CurrRange {
    fn to_string(range: CurrRange) -> String {
        match range {
            CurrRange::Range60nA  => "60nA".to_string(), 
            CurrRange::Range100nA => "100nA".to_string(), 
            CurrRange::Range1uA   => "1uA".to_string(), 
            CurrRange::Range10uA  => "10uA".to_string(), 
            CurrRange::Range100uA => "100uA".to_string(), 
            CurrRange::Range1mA   => "1mA".to_string(), 
            CurrRange::Range10mA  => "10mA".to_string(), 
            CurrRange::Range12mA  => "12mA".to_string(), 
            CurrRange::Range24mA  => "24mA".to_string(), 
        }
    }

    //fn from_str(range: &str) -> CurrRange {
    //}
}

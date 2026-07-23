//! Protocol constants: parameter/command identifiers and frame delimiters.

/// Identifiers of the sensor's configurable parameters.
///
/// Covers detection `RangeGate`, `AbsenseReportDelay is seconds`
/// and the trigger/hold thresholds for each of the 16 distance gates.
/// Each variant's value is the 16-bit word
/// Rage calculates as: 
/// RangeGate=0 -> 0-70cm(~35cm delcared)
/// RangeGate=1 -> 0-1400cm(~70cm delcared)
/// RangeGate * 70cm 
/// for RangeGate 0 TriggerThreshold00, HoldThreshold00 sets detection sensivity limits       
/// for RangeGate 1 TriggerThreshold00, HoldThreshold00 and TriggerThreshold01, HoldThreshold01        
#[repr(u16)]
pub enum ParameterID{
    RangeGate = 0x0100,
    AbsenseReportDelay = 0x0400,

    TriggerThreshold00 = 0x1000,
    TriggerThreshold01 = 0x1100,
    TriggerThreshold02 = 0x1200,
    TriggerThreshold03 = 0x1300,
    TriggerThreshold04 = 0x1400,
    TriggerThreshold05 = 0x1500,
    TriggerThreshold06 = 0x1600,

    TriggerThreshold07 = 0x1700,
    TriggerThreshold08 = 0x1800,
    TriggerThreshold09 = 0x1900,

    TriggerThreshold10 = 0x1A00,
    TriggerThreshold11 = 0x1B00,
    TriggerThreshold12 = 0x1C00,

    TriggerThreshold13 = 0x1D00,
    TriggerThreshold14 = 0x1E00,
    TriggerThreshold15 = 0x1F00,


    HoldThreshold00 = 0x2000,
    HoldThreshold01 = 0x2100,
    HoldThreshold02 = 0x2200,

    HoldThreshold03 = 0x2300,
    HoldThreshold04 = 0x2400,
    HoldThreshold05 = 0x2500,

    HoldThreshold06 = 0x2600,
    HoldThreshold07 = 0x2700,
    HoldThreshold08 = 0x2800,

    HoldThreshold09 = 0x2900,
    HoldThreshold10 = 0x2A00,
    HoldThreshold11 = 0x2B00,

    HoldThreshold12 = 0x2C00,
    HoldThreshold13 = 0x2D00,
    HoldThreshold14 = 0x2E00,
    HoldThreshold15 = 0x2F00,

}

impl ParameterID{
    pub fn get_bytes(self) -> [u8;2]{
        (self as u16).to_be_bytes()
    }

    pub fn default_value(&self) -> f32{
        match self {
            ParameterID::RangeGate => 15.0,
            ParameterID::AbsenseReportDelay => 10.0,
            ParameterID::TriggerThreshold00 => 48.93,
            ParameterID::TriggerThreshold01 => 45.57,
            ParameterID::TriggerThreshold02 => 43.20,
            ParameterID::TriggerThreshold03 => 36.18,
            ParameterID::TriggerThreshold04 => 34.45,
            ParameterID::TriggerThreshold05 => 32.04,
            ParameterID::TriggerThreshold06 => 30.22,
            ParameterID::TriggerThreshold07 => 27.90,
            ParameterID::TriggerThreshold08 => 25.86,
            ParameterID::TriggerThreshold09 => 23.45,
            ParameterID::TriggerThreshold10 => 21.90,
            ParameterID::TriggerThreshold11 => 21.37,
            ParameterID::TriggerThreshold12 => 19.98,
            ParameterID::TriggerThreshold13 => 20.05,
            ParameterID::TriggerThreshold14 => 18.98,
            ParameterID::TriggerThreshold15 => 18.75,
            ParameterID::HoldThreshold00 => 47.38,
            ParameterID::HoldThreshold01 => 44.03,
            ParameterID::HoldThreshold02 => 41.66,
            ParameterID::HoldThreshold03 => 34.63,
            ParameterID::HoldThreshold04 => 32.90,
            ParameterID::HoldThreshold05 => 30.49,
            ParameterID::HoldThreshold06 => 28.67,
            ParameterID::HoldThreshold07 => 26.35,
            ParameterID::HoldThreshold08 => 24.31,
            ParameterID::HoldThreshold09 => 21.90,
            ParameterID::HoldThreshold10 => 20.35,
            ParameterID::HoldThreshold11 => 19.82,
            ParameterID::HoldThreshold12 => 18.44,
            ParameterID::HoldThreshold13 => 18.50,
            ParameterID::HoldThreshold14 => 17.43,
            ParameterID::HoldThreshold15 => 17.20,
        }
    }

}

#[repr(u16)]
pub enum CommandID{
    EnableConfig = 0xFF00,
    EnableConfigAck = 0xFF01,
    EndSaveConfig = 0xFE00,
    EndSaveConfigAck = 0xFE01,
    WriteParam = 0x0700,
    WriteParamAck = 0x0701,
    ReadParam = 0x0800,
    ReadParamAck = 0x0801,
    ReadFirmwareVersion = 0x0000,
    ReadSerialNumber = 0x1100,
    ReportMode = 0x1200,//64 00 00 00 = Basic (ASCII ON RangeGate 1234 or OFF); 04 00 00 00 = RangeGate with energy; 00 00 00 00 = 20Dopple * 16EnergyGates
    None = 0xFFFF,
}

impl CommandID{
    pub fn get_bytes(self) -> [u8;2]{
        (self as u16).to_be_bytes()
    }

    pub const fn as_u16(self) -> u16 {
        self as u16
    }

}

pub const SEND_HEADER:[u8;4] = [0xFD ,0xFC,0xFB,0xFA];
pub const SEND_TAIL:[u8;4] = [0x04 ,0x03,0x02,0x01];











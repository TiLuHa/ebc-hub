use std::fmt;

pub const INBOUND_FRAME_SIZE: usize = 19;
pub const OUTBOUND_FRAME_SIZE: usize = 10;

// Start of Frame (SOF) and End of Frame (EOF) bytes.
pub const START_BYTE: u8 = 0xfa;
pub const END_BYTE: u8 = 0xf8;

// ZKETECH EBC model codes sent from the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    EbcA05 = 0x05,
    EbcA10H = 0x06,
    EbcA20 = 0x09,
}

impl TryFrom<u8> for DeviceType {
    type Error = color_eyre::eyre::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == DeviceType::EbcA05 as u8 => Ok(DeviceType::EbcA05),
            x if x == DeviceType::EbcA10H as u8 => Ok(DeviceType::EbcA10H),
            x if x == DeviceType::EbcA20 as u8 => Ok(DeviceType::EbcA20),
            val => Err(color_eyre::eyre::eyre!("Unknown device code: {}", val)),
        }
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::EbcA20 => write!(f, "EBC-A20"),
            DeviceType::EbcA10H => write!(f, "EBC-A10H"),
            DeviceType::EbcA05 => write!(f, "EBC-A05"),
        }
    }
}

pub mod command;
pub mod ebc_manager_runner;

pub use command::Command;
pub use ebc_manager_runner::EbcRunner;

use crate::ebc::{
    constants::DeviceType, device_capabilities::DeviceCapabilities, frame::InboundFrame,
};

#[derive(Clone, Debug)]
pub struct Event {
    pub frame: InboundFrame,
}

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub device_type: DeviceType,
    pub fw_version: String,
    pub capabilities: DeviceCapabilities,
}

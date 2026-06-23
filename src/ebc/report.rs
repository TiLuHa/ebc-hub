use serde::{Deserialize, Serialize};

use crate::ebc::constants::DeviceType;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceMode {
    DischargeConstantCurrent,
    DischargeConstantPower,
    ChargeConstantVoltage,
}

impl std::fmt::Display for DeviceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DischargeConstantCurrent => write!(f, "Discharge Constant Current"),
            Self::DischargeConstantPower => write!(f, "Discharge Constant Power"),
            Self::ChargeConstantVoltage => write!(f, "Charge Constant Voltage"),
        }
    }
}

pub enum StatusReportType {
    DischargeConstantCurrentOnReport = 0x0A,
    DischargeConstantCurrentOnFirmwareReport = 0x6E,
    DischargeConstantCurrentOffReport = 0x00,
    DischargeConstantCurrentOffFirmwareReport = 0x64,
    DischargeConstantCurrentEnd = 0x14,

    DischargeConstantPowerOnReport = 0x0B,
    DischargeConstantPowerOnFirmwareReport = 0x6F,
    DischargeConstantPowerOffReport = 0x01,
    DischargeConstantPowerOffFirmwareReport = 0x65,
    DischargeConstantPowerEnd = 0x15,

    ChargeConstantCurrentOnReport = 0x0C,
    ChargeConstantCurrentOnFirmwareReport = 0x70,
    ChargeConstantCurrentOffReport = 0x02,
    ChargeConstantCurrentOffFirmwareReport = 0x66,
    ChargeConstantCurrentEnd = 0x16,
}

#[derive(Clone, Debug)]
pub struct FirmwareReport {
    pub device_mode: DeviceMode,
    pub in_progress: bool,
    pub current_ma: u16,
    pub voltage_mv: u16,
    pub milli_ampere_hours: u16,
    pub unknown: u16, // Always 0.
    pub firmware_version: String,
    // Calibration parameters, offset and gain maybe?
    pub unknown1: u16, // Always 2988
    pub unknown2: u16, // Always 2087
    pub device_type: DeviceType,
}

#[derive(Clone, Debug)]
pub struct ChargeReport {
    pub in_progress: bool,
    pub current_ma: u16,
    pub voltage_mv: u16,
    pub milli_ampere_hours: u16,
    pub unknown: u16, // Always 0.
    pub charge_current_ma: u16,
    pub charge_voltage_mv: u16,
    pub cutoff_current_ma: u16,
    pub device_type: DeviceType,
}

#[derive(Clone, Debug)]
pub struct DischargeConstantCurrentReport {
    pub in_progress: bool,
    pub current_ma: u16,
    pub voltage_mv: u16,
    pub milli_ampere_hours: u16,
    pub unknown: u16, // Always 0.
    pub discharge_current_ma: u16,
    pub cutoff_voltage_mv: u16,
    pub cutoff_time_min: u16,
    pub device_type: DeviceType,
}

#[derive(Clone, Debug)]
pub struct DischargeConstantPowerReport {
    pub in_progress: bool,
    pub current_ma: u16,
    pub voltage_mv: u16,
    pub milli_ampere_hours: u16,
    pub unknown: u16, // Always 0.
    pub discharge_power_w: u16,
    pub cutoff_voltage_mv: u16,
    pub cutoff_time_min: u16,
    pub device_type: DeviceType,
}

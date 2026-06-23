use crate::ebc::{
    codec::{decode_base240, xor_checksum},
    command::{
        adjust_constant_current_discharge_command, adjust_constant_current_voltage_charge_command,
        adjust_constant_power_discharge_command, calibration_command, connect_command,
        continue_constant_current_discharge_command,
        continue_constant_current_voltage_charge_command,
        continue_constant_power_discharge_command, disconnect_command,
        start_constant_current_discharge_command, start_constant_current_voltage_charge_command,
        start_constant_power_discharge_command, stop_command, timer_sync_command,
    },
    constants::{DeviceType, END_BYTE, INBOUND_FRAME_SIZE, OUTBOUND_FRAME_SIZE, START_BYTE},
    device_capabilities::DeviceCapabilities,
    report::{
        ChargeReport, DeviceMode, DischargeConstantCurrentReport, DischargeConstantPowerReport,
        FirmwareReport, StatusReportType,
    },
};

pub fn process_buffer(buf: &mut Vec<u8>) -> Vec<(InboundFrame, Vec<u8>)> {
    let mut frames = Vec::new();
    loop {
        if let Some(start) = buf.iter().position(|&b| b == START_BYTE) {
            if start > 0 {
                buf.drain(..start);
            }
        } else {
            buf.clear();
            break;
        }
        if let Some(end) = buf[1..].iter().position(|&b| b == END_BYTE) {
            let frame_end = end + 2; // +1 for slice offset, +1 for inclusive
            let raw = buf.drain(..frame_end).collect::<Vec<u8>>();
            match InboundFrame::try_from(raw.as_slice()) {
                Ok(f) => frames.push((f, raw)),
                Err(e) => tracing::debug!("Failed to parse frame: {e}"),
            }
        } else {
            break;
        }
    }
    frames
}

pub struct OutboundFrame(OutboundFrameKind);

#[derive(Debug, Clone)]
enum OutboundFrameKind {
    Connect,
    Disconnect,
    Stop,
    StartConstantCurrentDischarge {
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    },
    AdjustConstantCurrentDischarge {
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    },
    ContinueConstantCurrentDischarge {
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    },
    StartConstantPowerDischarge {
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    },
    AdjustConstantPowerDischarge {
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    },
    ContinueConstantPowerDischarge {
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    },
    StartConstantCurrentVoltageCharge {
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    },
    AdjustConstantCurrentVoltageCharge {
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    },
    ContinueConstantCurrentVoltageCharge {
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    },
    TimerSync {
        time_in_min: u16,
    },
    CalibrateVoltageLow {
        mv: u16,
    },
    CalibrateVoltageHigh {
        mv: u16,
    },
    CalibrateCurrentLow {
        ma: u16,
    },
    CalibrateCurrentHigh {
        ma: u16,
    },
    CalibrateConfirm,
}

impl OutboundFrame {
    pub fn connect() -> Self {
        OutboundFrame(OutboundFrameKind::Connect)
    }
    pub fn disconnect() -> Self {
        OutboundFrame(OutboundFrameKind::Disconnect)
    }
    pub fn stop() -> Self {
        OutboundFrame(OutboundFrameKind::Stop)
    }
    pub fn timer_sync(time_in_min: u16) -> Self {
        OutboundFrame(OutboundFrameKind::TimerSync { time_in_min })
    }
    pub fn calibrate_voltage_low(mv: u16) -> Self {
        OutboundFrame(OutboundFrameKind::CalibrateVoltageLow { mv })
    }
    pub fn calibrate_voltage_high(mv: u16) -> Self {
        OutboundFrame(OutboundFrameKind::CalibrateVoltageHigh { mv })
    }
    pub fn calibrate_current_low(ma: u16) -> Self {
        OutboundFrame(OutboundFrameKind::CalibrateCurrentLow { ma })
    }
    pub fn calibrate_current_high(ma: u16) -> Self {
        OutboundFrame(OutboundFrameKind::CalibrateCurrentHigh { ma })
    }
    pub fn calibrate_confirm() -> Self {
        OutboundFrame(OutboundFrameKind::CalibrateConfirm)
    }
    pub fn start_constant_current_discharge(
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        device_capabilities: &DeviceCapabilities,
    ) -> color_eyre::Result<Self> {
        device_capabilities.check_constant_current_discharge_command_parameters(
            discharge_current_ma,
            cutoff_voltage_mv,
            cutoff_time_min,
        )?;
        Ok(Self(OutboundFrameKind::StartConstantCurrentDischarge {
            discharge_current_ma,
            cutoff_voltage_mv,
            cutoff_time_min,
        }))
    }
    pub fn adjust_constant_current_discharge(
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        device_capabilities: &DeviceCapabilities,
    ) -> color_eyre::Result<Self> {
        device_capabilities.check_constant_current_discharge_command_parameters(
            discharge_current_ma,
            cutoff_voltage_mv,
            cutoff_time_min,
        )?;
        Ok(Self(OutboundFrameKind::AdjustConstantCurrentDischarge {
            discharge_current_ma,
            cutoff_voltage_mv,
            cutoff_time_min,
        }))
    }
    pub fn continue_constant_current_discharge(
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        device_capabilities: &DeviceCapabilities,
    ) -> color_eyre::Result<Self> {
        device_capabilities.check_constant_current_discharge_command_parameters(
            discharge_current_ma,
            cutoff_voltage_mv,
            cutoff_time_min,
        )?;
        Ok(Self(OutboundFrameKind::ContinueConstantCurrentDischarge {
            discharge_current_ma,
            cutoff_voltage_mv,
            cutoff_time_min,
        }))
    }
    pub fn start_constant_power_discharge(
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        device_capabilities: &DeviceCapabilities,
    ) -> color_eyre::Result<Self> {
        device_capabilities.check_constant_power_discharge_command_parameters(
            discharge_power_w,
            cutoff_voltage_mv,
            cutoff_time_min,
        )?;
        Ok(Self(OutboundFrameKind::StartConstantPowerDischarge {
            discharge_power_w,
            cutoff_voltage_mv,
            cutoff_time_min,
        }))
    }
    pub fn adjust_constant_power_discharge(
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        device_capabilities: &DeviceCapabilities,
    ) -> color_eyre::Result<Self> {
        device_capabilities.check_constant_power_discharge_command_parameters(
            discharge_power_w,
            cutoff_voltage_mv,
            cutoff_time_min,
        )?;
        Ok(Self(OutboundFrameKind::AdjustConstantPowerDischarge {
            discharge_power_w,
            cutoff_voltage_mv,
            cutoff_time_min,
        }))
    }
    pub fn continue_constant_power_discharge(
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        device_capabilities: &DeviceCapabilities,
    ) -> color_eyre::Result<Self> {
        device_capabilities.check_constant_power_discharge_command_parameters(
            discharge_power_w,
            cutoff_voltage_mv,
            cutoff_time_min,
        )?;
        Ok(Self(OutboundFrameKind::ContinueConstantPowerDischarge {
            discharge_power_w,
            cutoff_voltage_mv,
            cutoff_time_min,
        }))
    }
    pub fn start_constant_current_voltage_charge(
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
        device_capabilities: &DeviceCapabilities,
    ) -> color_eyre::Result<Self> {
        device_capabilities.check_constant_current_voltage_charge_command_parameters(
            charge_current_ma,
            charge_voltage_mv,
            cutoff_current_ma,
        )?;
        Ok(Self(OutboundFrameKind::StartConstantCurrentVoltageCharge {
            charge_current_ma,
            charge_voltage_mv,
            cutoff_current_ma,
        }))
    }
    pub fn adjust_constant_current_voltage_charge(
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
        device_capabilities: &DeviceCapabilities,
    ) -> color_eyre::Result<Self> {
        device_capabilities.check_constant_current_voltage_charge_command_parameters(
            charge_current_ma,
            charge_voltage_mv,
            cutoff_current_ma,
        )?;
        Ok(Self(
            OutboundFrameKind::AdjustConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
            },
        ))
    }
    pub fn continue_constant_current_voltage_charge(
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
        device_capabilities: &DeviceCapabilities,
    ) -> color_eyre::Result<Self> {
        device_capabilities.check_constant_current_voltage_charge_command_parameters(
            charge_current_ma,
            charge_voltage_mv,
            cutoff_current_ma,
        )?;
        Ok(Self(
            OutboundFrameKind::ContinueConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
            },
        ))
    }
}

impl std::convert::From<OutboundFrame> for [u8; OUTBOUND_FRAME_SIZE] {
    fn from(frame: OutboundFrame) -> Self {
        match frame {
            OutboundFrame(OutboundFrameKind::Connect) => connect_command(),
            OutboundFrame(OutboundFrameKind::Disconnect) => disconnect_command(),
            OutboundFrame(OutboundFrameKind::Stop) => stop_command(),
            OutboundFrame(OutboundFrameKind::StartConstantCurrentDischarge {
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
            }) => start_constant_current_discharge_command(
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
            ),
            OutboundFrame(OutboundFrameKind::AdjustConstantCurrentDischarge {
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
            }) => adjust_constant_current_discharge_command(
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
            ),
            OutboundFrame(OutboundFrameKind::ContinueConstantCurrentDischarge {
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
            }) => continue_constant_current_discharge_command(
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
            ),
            OutboundFrame(OutboundFrameKind::StartConstantPowerDischarge {
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
            }) => start_constant_power_discharge_command(
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
            ),
            OutboundFrame(OutboundFrameKind::AdjustConstantPowerDischarge {
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
            }) => adjust_constant_power_discharge_command(
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
            ),
            OutboundFrame(OutboundFrameKind::ContinueConstantPowerDischarge {
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
            }) => continue_constant_power_discharge_command(
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
            ),
            OutboundFrame(OutboundFrameKind::StartConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
            }) => start_constant_current_voltage_charge_command(
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
            ),
            OutboundFrame(OutboundFrameKind::AdjustConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
            }) => adjust_constant_current_voltage_charge_command(
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
            ),
            OutboundFrame(OutboundFrameKind::ContinueConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
            }) => continue_constant_current_voltage_charge_command(
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
            ),
            OutboundFrame(OutboundFrameKind::TimerSync { time_in_min }) => {
                timer_sync_command(time_in_min)
            }
            OutboundFrame(OutboundFrameKind::CalibrateVoltageLow { mv }) => {
                calibration_command(0x00, mv)
            }
            OutboundFrame(OutboundFrameKind::CalibrateVoltageHigh { mv }) => {
                calibration_command(0x01, mv)
            }
            OutboundFrame(OutboundFrameKind::CalibrateCurrentLow { ma }) => {
                calibration_command(0x02, ma)
            }
            OutboundFrame(OutboundFrameKind::CalibrateCurrentHigh { ma }) => {
                calibration_command(0x03, ma)
            }
            OutboundFrame(OutboundFrameKind::CalibrateConfirm) => calibration_command(0x04, 0),
        }
    }
}

#[derive(Clone, Debug)]
pub enum InboundFrame {
    Firmware(FirmwareReport),
    DischargeConstantCurrent(DischargeConstantCurrentReport),
    DischargeConstantPower(DischargeConstantPowerReport),
    Charge(ChargeReport),
}

impl InboundFrame {
    fn construct_firmware_report(payload: &[u8]) -> Self {
        let command_byte = payload[0];
        let in_progress = command_byte
            == StatusReportType::ChargeConstantCurrentOnFirmwareReport as u8
            || command_byte == StatusReportType::DischargeConstantPowerOnFirmwareReport as u8
            || command_byte == StatusReportType::DischargeConstantCurrentOnFirmwareReport as u8;
        let device_mode = if command_byte
            == StatusReportType::ChargeConstantCurrentOnFirmwareReport as u8
            || command_byte == StatusReportType::ChargeConstantCurrentOffFirmwareReport as u8
        {
            DeviceMode::ChargeConstantVoltage
        } else if command_byte == StatusReportType::DischargeConstantPowerOnFirmwareReport as u8
            || command_byte == StatusReportType::DischargeConstantPowerOffFirmwareReport as u8
        {
            DeviceMode::DischargeConstantPower
        } else {
            DeviceMode::DischargeConstantCurrent
        };
        let version = decode_base240(payload[9], payload[10]);
        let major = version / 100;
        let minor = (version % 100) / 10;
        let patch = version % 10;

        Self::Firmware(FirmwareReport {
            device_mode,
            in_progress,
            current_ma: decode_base240(payload[1], payload[2]) * 10,
            voltage_mv: decode_base240(payload[3], payload[4]),
            milli_ampere_hours: decode_base240(payload[5], payload[6]),
            unknown: decode_base240(payload[7], payload[8]),
            firmware_version: format!("{major}.{minor}.{patch}"),
            unknown1: decode_base240(payload[11], payload[12]),
            unknown2: decode_base240(payload[13], payload[14]),
            device_type: DeviceType::try_from(payload[15]).unwrap(),
        })
    }

    fn construct_charge_report(payload: &[u8]) -> Self {
        let command_byte = payload[0];
        Self::Charge(ChargeReport {
            in_progress: command_byte == StatusReportType::ChargeConstantCurrentOnReport as u8,
            current_ma: decode_base240(payload[1], payload[2]) * 10,
            voltage_mv: decode_base240(payload[3], payload[4]),
            milli_ampere_hours: decode_base240(payload[5], payload[6]),
            unknown: decode_base240(payload[7], payload[8]),
            charge_current_ma: decode_base240(payload[9], payload[10]) * 10,
            charge_voltage_mv: decode_base240(payload[11], payload[12]),
            cutoff_current_ma: decode_base240(payload[13], payload[14]),
            device_type: DeviceType::try_from(payload[15]).unwrap(),
        })
    }

    fn construct_discharge_constant_current_report(payload: &[u8]) -> Self {
        let command_byte = payload[0];
        Self::DischargeConstantCurrent(DischargeConstantCurrentReport {
            in_progress: command_byte == StatusReportType::DischargeConstantCurrentOnReport as u8,
            current_ma: decode_base240(payload[1], payload[2]) * 10,
            voltage_mv: decode_base240(payload[3], payload[4]),
            milli_ampere_hours: decode_base240(payload[5], payload[6]),
            unknown: decode_base240(payload[7], payload[8]),
            discharge_current_ma: decode_base240(payload[9], payload[10]) * 10,
            cutoff_voltage_mv: decode_base240(payload[11], payload[12]) * 10,
            cutoff_time_min: decode_base240(payload[13], payload[14]),
            device_type: DeviceType::try_from(payload[15]).unwrap(),
        })
    }

    fn construct_discharge_constant_power_report(payload: &[u8]) -> Self {
        let command_byte = payload[0];
        Self::DischargeConstantPower(DischargeConstantPowerReport {
            in_progress: command_byte == StatusReportType::DischargeConstantPowerOnReport as u8,
            current_ma: decode_base240(payload[1], payload[2]) * 10,
            voltage_mv: decode_base240(payload[3], payload[4]),
            milli_ampere_hours: decode_base240(payload[5], payload[6]),
            unknown: decode_base240(payload[7], payload[8]),
            discharge_power_w: decode_base240(payload[9], payload[10]),
            cutoff_voltage_mv: decode_base240(payload[11], payload[12]),
            cutoff_time_min: decode_base240(payload[13], payload[14]),
            device_type: DeviceType::try_from(payload[15]).unwrap(),
        })
    }
}

impl TryFrom<&[u8]> for InboundFrame {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != INBOUND_FRAME_SIZE {
            return Err(format!(
                "Frame length mismatch: expected {}, got {}",
                INBOUND_FRAME_SIZE,
                value.len()
            ));
        }
        if value[0] != START_BYTE {
            return Err(format!(
                "Invalid start byte: expected {START_BYTE:#04x}, got {:#04x}",
                value[0]
            ));
        }
        if value[value.len() - 1] != END_BYTE {
            return Err(format!(
                "Invalid end byte: expected {END_BYTE:#04x}, got {:#04x}",
                value[value.len() - 1]
            ));
        }
        let payload = &value[1..value.len() - 2];
        let checksum = value[value.len() - 2];
        let calculated_checksum = xor_checksum(payload);
        // It seems there is a bug in the device firmware. When you discharge to
        // 3.3V with 1A. The checksum byte is wrong after about 2 mins of
        // discharging. If the checksum is ignored, the frame still has correct
        // data in it. This happens with firmware version 3.0.2 to me at least.
        // So we log the checksum error instead.
        if calculated_checksum != checksum {
            tracing::debug!(
                "Invalid checksum: expected {calculated_checksum:#04x}, got {checksum:#04x}"
            );
        }
        let command_byte = payload[0];
        match command_byte {
            // Charging, discharge (constant power and current) and idle
            // firmware reports have same frame structure. The difference is
            // that when charge is idle. It will send idle firmware report. When
            // charging, firmware report is sent for few seconds and not after
            // that. starting charge.
            x if x == StatusReportType::ChargeConstantCurrentOnFirmwareReport as u8
                || x == StatusReportType::ChargeConstantCurrentOffFirmwareReport as u8
                || x == StatusReportType::DischargeConstantPowerOnFirmwareReport as u8
                || x == StatusReportType::DischargeConstantPowerOffFirmwareReport as u8
                || x == StatusReportType::DischargeConstantCurrentOnFirmwareReport as u8
                || x == StatusReportType::DischargeConstantCurrentOffFirmwareReport as u8 =>
            {
                Ok(Self::construct_firmware_report(payload))
            }
            x if x == StatusReportType::ChargeConstantCurrentOnReport as u8
                || x == StatusReportType::ChargeConstantCurrentOffReport as u8
                || x == StatusReportType::ChargeConstantCurrentEnd as u8 =>
            {
                Ok(Self::construct_charge_report(payload))
            }
            x if x == StatusReportType::DischargeConstantCurrentOnReport as u8
                || x == StatusReportType::DischargeConstantCurrentOffReport as u8
                || x == StatusReportType::DischargeConstantCurrentEnd as u8 =>
            {
                Ok(Self::construct_discharge_constant_current_report(payload))
            }
            x if x == StatusReportType::DischargeConstantPowerOnReport as u8
                || x == StatusReportType::DischargeConstantPowerOffReport as u8
                || x == StatusReportType::DischargeConstantPowerEnd as u8 =>
            {
                Ok(Self::construct_discharge_constant_power_report(payload))
            }
            _ => Err(format!("Unknown command byte: {command_byte:#04x}")),
        }
    }
}

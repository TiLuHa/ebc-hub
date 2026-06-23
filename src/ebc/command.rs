use crate::ebc::codec::{encode_base240, xor_checksum};
use crate::ebc::constants::{END_BYTE, OUTBOUND_FRAME_SIZE, START_BYTE};

enum CommmandType {
    Connect = 0x05,
    Disconnect = 0x06,
    Stop = 0x02,
    StartConstantCurrentDischarge = 0x01,
    AdjustConstantCurrentDischarge = 0x07,
    ContinueConstantCurrentDischarge = 0x08,
    StartConstantPowerDischarge = 0x11,
    AdjustConstantPowerDischarge = 0x17,
    ContinueConstantPowerDischarge = 0x18,
    StartConstantCurrentVoltageCharge = 0x21,
    AdjustConstantCurrentVoltageCharge = 0x27,
    ContinueConstantCurrentVoltageCharge = 0x28,
    TimerSync = 0x0A,
    Calibration = 0x04,
}

fn build_frame(payload: [u8; 7]) -> [u8; OUTBOUND_FRAME_SIZE] {
    let mut frame = [0u8; OUTBOUND_FRAME_SIZE];
    frame[0] = START_BYTE;
    frame[1..8].copy_from_slice(&payload);
    frame[8] = xor_checksum(&payload);
    frame[9] = END_BYTE;
    frame
}

pub fn connect_command() -> [u8; OUTBOUND_FRAME_SIZE] {
    build_frame([
        CommmandType::Connect as u8,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ])
}

pub fn disconnect_command() -> [u8; OUTBOUND_FRAME_SIZE] {
    build_frame([
        CommmandType::Disconnect as u8,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
    ])
}

pub fn stop_command() -> [u8; OUTBOUND_FRAME_SIZE] {
    build_frame([CommmandType::Stop as u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
}

pub fn timer_sync_command(minutes: u16) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (min_h, min_l) = encode_base240(minutes);
    build_frame([
        CommmandType::TimerSync as u8,
        min_h,
        min_l,
        0x00,
        0x00,
        0x00,
        0x00,
    ])
}

pub fn calibration_command(sub: u8, value: u16) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (val_h, val_l) = encode_base240(value);
    build_frame([
        CommmandType::Calibration as u8,
        sub,
        val_h,
        val_l,
        0x00,
        0x00,
        0x00,
    ])
}

pub fn start_constant_current_discharge_command(
    discharge_current_ma: u16,
    cutoff_voltage_mv: u16,
    cutoff_time_min: u16,
) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (current_h, current_l) = encode_base240(discharge_current_ma / 10);
    let (cutoff_h, cutoff_l) = encode_base240(cutoff_voltage_mv / 10);
    let (time_h, time_l) = encode_base240(cutoff_time_min);
    build_frame([
        CommmandType::StartConstantCurrentDischarge as u8,
        current_h,
        current_l,
        cutoff_h,
        cutoff_l,
        time_h,
        time_l,
    ])
}

pub fn adjust_constant_current_discharge_command(
    discharge_current_ma: u16,
    cutoff_voltage_mv: u16,
    cutoff_time_min: u16,
) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (current_h, current_l) = encode_base240(discharge_current_ma / 10);
    let (cutoff_h, cutoff_l) = encode_base240(cutoff_voltage_mv / 10);
    let (time_h, time_l) = encode_base240(cutoff_time_min);
    build_frame([
        CommmandType::AdjustConstantCurrentDischarge as u8,
        current_h,
        current_l,
        cutoff_h,
        cutoff_l,
        time_h,
        time_l,
    ])
}

pub fn continue_constant_current_discharge_command(
    discharge_current_ma: u16,
    cutoff_voltage_mv: u16,
    cutoff_time_min: u16,
) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (current_h, current_l) = encode_base240(discharge_current_ma / 10);
    let (cutoff_h, cutoff_l) = encode_base240(cutoff_voltage_mv / 10);
    let (time_h, time_l) = encode_base240(cutoff_time_min);
    build_frame([
        CommmandType::ContinueConstantCurrentDischarge as u8,
        current_h,
        current_l,
        cutoff_h,
        cutoff_l,
        time_h,
        time_l,
    ])
}

pub fn start_constant_power_discharge_command(
    discharge_power_w: u16,
    cutoff_voltage_mv: u16,
    cutoff_time_min: u16,
) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (power_h, power_l) = encode_base240(discharge_power_w);
    let (cutoff_h, cutoff_l) = encode_base240(cutoff_voltage_mv / 10);
    let (time_h, time_l) = encode_base240(cutoff_time_min);
    build_frame([
        CommmandType::StartConstantPowerDischarge as u8,
        power_h,
        power_l,
        cutoff_h,
        cutoff_l,
        time_h,
        time_l,
    ])
}

pub fn adjust_constant_power_discharge_command(
    discharge_power_w: u16,
    cutoff_voltage_mv: u16,
    cutoff_time_min: u16,
) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (power_h, power_l) = encode_base240(discharge_power_w);
    let (cutoff_h, cutoff_l) = encode_base240(cutoff_voltage_mv / 10);
    let (time_h, time_l) = encode_base240(cutoff_time_min);
    build_frame([
        CommmandType::AdjustConstantPowerDischarge as u8,
        power_h,
        power_l,
        cutoff_h,
        cutoff_l,
        time_h,
        time_l,
    ])
}

pub fn continue_constant_power_discharge_command(
    discharge_power_w: u16,
    cutoff_voltage_mv: u16,
    cutoff_time_min: u16,
) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (power_h, power_l) = encode_base240(discharge_power_w);
    let (cutoff_h, cutoff_l) = encode_base240(cutoff_voltage_mv / 10);
    let (time_h, time_l) = encode_base240(cutoff_time_min);
    build_frame([
        CommmandType::ContinueConstantPowerDischarge as u8,
        power_h,
        power_l,
        cutoff_h,
        cutoff_l,
        time_h,
        time_l,
    ])
}

pub fn start_constant_current_voltage_charge_command(
    charge_current_ma: u16,
    charge_voltage_mv: u16,
    cutoff_current_ma: u16,
) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (current_h, current_l) = encode_base240(charge_current_ma / 10);
    let (charge_voltage_h, charge_voltage_l) = encode_base240(charge_voltage_mv / 10);
    let (cutoff_current_h, cutoff_current_l) = encode_base240(cutoff_current_ma / 10);
    build_frame([
        CommmandType::StartConstantCurrentVoltageCharge as u8,
        current_h,
        current_l,
        charge_voltage_h,
        charge_voltage_l,
        cutoff_current_h,
        cutoff_current_l,
    ])
}

pub fn adjust_constant_current_voltage_charge_command(
    charge_current_ma: u16,
    charge_voltage_mv: u16,
    cutoff_current_ma: u16,
) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (current_h, current_l) = encode_base240(charge_current_ma / 10);
    let (charge_voltage_h, charge_voltage_l) = encode_base240(charge_voltage_mv / 10);
    let (cutoff_current_h, cutoff_current_l) = encode_base240(cutoff_current_ma / 10);
    build_frame([
        CommmandType::AdjustConstantCurrentVoltageCharge as u8,
        current_h,
        current_l,
        charge_voltage_h,
        charge_voltage_l,
        cutoff_current_h,
        cutoff_current_l,
    ])
}

pub fn continue_constant_current_voltage_charge_command(
    charge_current_ma: u16,
    charge_voltage_mv: u16,
    cutoff_current_ma: u16,
) -> [u8; OUTBOUND_FRAME_SIZE] {
    let (current_h, current_l) = encode_base240(charge_current_ma / 10);
    let (charge_voltage_h, charge_voltage_l) = encode_base240(charge_voltage_mv / 10);
    let (cutoff_current_h, cutoff_current_l) = encode_base240(cutoff_current_ma / 10);
    build_frame([
        CommmandType::ContinueConstantCurrentVoltageCharge as u8,
        current_h,
        current_l,
        charge_voltage_h,
        charge_voltage_l,
        cutoff_current_h,
        cutoff_current_l,
    ])
}

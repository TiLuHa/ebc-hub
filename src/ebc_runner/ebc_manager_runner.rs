use color_eyre::Result;
use std::time::Duration;
use tokio::{
    sync::{
        broadcast,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
    time,
};

use crate::{
    ebc::{
        device::Device,
        device_capabilities::DeviceCapabilities,
        frame::{InboundFrame, OutboundFrame},
    },
    ebc_runner::{Command, Event},
};

pub struct EbcRunner {
    ebc: Device,
    cmd_rx: UnboundedReceiver<Command>,
    cmd_tx: UnboundedSender<Command>,
    event_tx: broadcast::Sender<Event>,
    latest_report: InboundFrame,
    devivce_capabilities: DeviceCapabilities,
}

impl EbcRunner {
    pub fn new(mut ebc: Device) -> Result<Self> {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<Command>();
        let event_tx = broadcast::Sender::new(1000);
        let t = ebc.read_until_firmware_report(time::Duration::from_millis(2000))?;
        let caps = DeviceCapabilities::from(t.device_type);
        Ok(EbcRunner {
            cmd_rx,
            cmd_tx,
            ebc,
            latest_report: InboundFrame::Firmware(t),
            event_tx,
            devivce_capabilities: caps,
        })
    }

    pub fn cmd_tx(&self) -> UnboundedSender<Command> {
        self.cmd_tx.clone()
    }

    async fn handle_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Status { callback } => {
                callback
                    .send(Event {
                        frame: self.latest_report.clone(),
                    })
                    .ok();
            }
            Command::SubReports { callback } => {
                callback.send(self.event_tx.subscribe()).ok();
            }
            Command::Stop { callback } => {
                callback.send(self.ebc.send(OutboundFrame::stop())).ok();
            }
            Command::StartConstantCurrentDischarge {
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let frame = OutboundFrame::start_constant_current_discharge(
                        discharge_current_ma,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &self.devivce_capabilities,
                    )?;
                    self.ebc.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            Command::AdjustConstantCurrentDischarge {
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let frame = OutboundFrame::adjust_constant_current_discharge(
                        discharge_current_ma,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &self.devivce_capabilities,
                    )?;
                    self.ebc.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            Command::ContinueConstantCurrentDischarge {
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let frame = OutboundFrame::continue_constant_current_discharge(
                        discharge_current_ma,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &self.devivce_capabilities,
                    )?;
                    self.ebc.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            Command::StartConstantPowerDischarge {
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let frame = OutboundFrame::start_constant_power_discharge(
                        discharge_power_w,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &self.devivce_capabilities,
                    )?;
                    self.ebc.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            Command::AdjustConstantPowerDischarge {
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let frame = OutboundFrame::adjust_constant_power_discharge(
                        discharge_power_w,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &self.devivce_capabilities,
                    )?;
                    self.ebc.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            Command::ContinueConstantPowerDischarge {
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let frame = OutboundFrame::continue_constant_power_discharge(
                        discharge_power_w,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &self.devivce_capabilities,
                    )?;
                    self.ebc.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            Command::StartConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let frame = OutboundFrame::start_constant_current_voltage_charge(
                        charge_current_ma,
                        charge_voltage_mv,
                        cutoff_current_ma,
                        &self.devivce_capabilities,
                    )?;
                    self.ebc.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            Command::AdjustConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let frame = OutboundFrame::adjust_constant_current_voltage_charge(
                        charge_current_ma,
                        charge_voltage_mv,
                        cutoff_current_ma,
                        &self.devivce_capabilities,
                    )?;
                    self.ebc.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            Command::ContinueConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let frame = OutboundFrame::continue_constant_current_voltage_charge(
                        charge_current_ma,
                        charge_voltage_mv,
                        cutoff_current_ma,
                        &self.devivce_capabilities,
                    )?;
                    self.ebc.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
        }
    }

    async fn read_frames(&mut self) {
        self.ebc.read_frames().unwrap().into_iter().for_each(|f| {
            self.event_tx.send(Event { frame: f.clone() }).ok();
            self.latest_report = f;
        });
    }

    pub async fn run(mut self) {
        let mut tick = time::interval(Duration::from_millis(100));
        tick.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

        loop {
            tokio::select! {
                Some(cmd) = self.cmd_rx.recv() => {
                    self.handle_cmd(cmd).await;
                }
                _ = tick.tick() => {
                    self.read_frames().await;
                }
            }
        }
    }
}

use color_eyre::Result;
use std::{collections::HashMap, time::Duration};
use tokio::{
    sync::{
        broadcast,
        mpsc::{self, UnboundedReceiver, UnboundedSender},
    },
    time,
};

use crate::{
    config::EbcConfig,
    ebc::{
        constants::DeviceType,
        device::EbcDevice,
        device_capabilities::DeviceCapabilities,
        frame::{InboundFrame, OutboundFrame},
    },
    ebc_manager_commands::EbcCommand,
};

#[derive(Clone, Debug)]
pub struct EbcEvent {
    pub frame: InboundFrame,
}

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub device_type: DeviceType,
    pub fw_version: String,
    pub capabilities: DeviceCapabilities,
}

struct ManagedEbc {
    dev: EbcDevice,
    report_sender: broadcast::Sender<EbcEvent>,
    info: DeviceInfo,
    latest_report: InboundFrame,
}

pub struct EbcManager {
    ebc_configs: HashMap<String, EbcConfig>,
    ebcs: HashMap<String, ManagedEbc>,
    cmd_rx: UnboundedReceiver<EbcCommand>,
    cmd_tx: UnboundedSender<EbcCommand>,
}

impl EbcManager {
    pub fn new(config: HashMap<String, EbcConfig>) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<EbcCommand>();
        EbcManager {
            ebc_configs: config,
            cmd_rx,
            cmd_tx,
            ebcs: HashMap::new(),
        }
    }

    pub fn cmd_tx(&self) -> UnboundedSender<EbcCommand> {
        self.cmd_tx.clone()
    }

    async fn handle_cmd(&mut self, cmd: EbcCommand) {
        match cmd {
            EbcCommand::Connect { id, callback } => {
                let result = (|| -> Result<DeviceInfo> {
                    if self.ebcs.contains_key(&id) {
                        return Err(color_eyre::eyre::eyre!(
                            "The given id is already connected!"
                        ));
                    }

                    let config = self.ebc_configs.get(&id).ok_or_else(|| {
                        color_eyre::eyre::eyre!("This config doesn't exist: {}", &id)
                    })?;

                    let mut dev = EbcDevice::new(&config.port)?;
                    dev.send(OutboundFrame::connect())?;
                    let (event_tx, _event_rx) = broadcast::channel::<EbcEvent>(1000);
                    let report = dev.read_until_firmware_report(Duration::from_millis(5000))?;
                    let dev_info = DeviceInfo {
                        device_type: report.device_type,
                        fw_version: report.firmware_version.clone(),
                        capabilities: DeviceCapabilities::from(report.device_type),
                    };
                    self.ebcs.insert(
                        id,
                        ManagedEbc {
                            dev,
                            report_sender: event_tx,
                            info: dev_info.clone(),
                            latest_report: InboundFrame::Firmware(report.clone()),
                        },
                    );

                    Ok(dev_info)
                })();

                callback.send(result).ok();
            }
            EbcCommand::Disconnect { id, callback } => {
                let result = (|| -> Result<()> {
                    let mut ebc = self
                        .ebcs
                        .remove(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;

                    if let Err(err) = ebc.dev.send(OutboundFrame::disconnect()) {
                        return Err(color_eyre::eyre::eyre!(
                            "Error when trying to disconnect. Assuming disconnected. Error: {:?}",
                            err
                        ));
                    }

                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::Status { id, callback } => {
                let result = (|| -> Result<EbcEvent> {
                    let managed = self
                        .ebcs
                        .get(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;

                    Ok(EbcEvent {
                        frame: managed.latest_report.clone(),
                    })
                })();
                callback.send(result).ok();
            }
            EbcCommand::SubReports { id, callback } => {
                let result = (|| -> Result<broadcast::Receiver<EbcEvent>> {
                    let ebc = self
                        .ebcs
                        .get(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    Ok(ebc.report_sender.subscribe())
                })();
                callback.send(result).ok();
            }
            EbcCommand::Stop { id, callback } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    ebc.dev.send(OutboundFrame::stop())?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::StartConstantCurrentDischarge {
                id,
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    let frame = OutboundFrame::start_constant_current_discharge(
                        discharge_current_ma,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &ebc.info.capabilities,
                    )?;
                    ebc.dev.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::AdjustConstantCurrentDischarge {
                id,
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    let frame = OutboundFrame::adjust_constant_current_discharge(
                        discharge_current_ma,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &ebc.info.capabilities,
                    )?;
                    ebc.dev.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::ContinueConstantCurrentDischarge {
                id,
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    let frame = OutboundFrame::continue_constant_current_discharge(
                        discharge_current_ma,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &ebc.info.capabilities,
                    )?;
                    ebc.dev.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::StartConstantPowerDischarge {
                id,
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    let frame = OutboundFrame::start_constant_power_discharge(
                        discharge_power_w,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &ebc.info.capabilities,
                    )?;
                    ebc.dev.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::AdjustConstantPowerDischarge {
                id,
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    let frame = OutboundFrame::adjust_constant_power_discharge(
                        discharge_power_w,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &ebc.info.capabilities,
                    )?;
                    ebc.dev.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::ContinueConstantPowerDischarge {
                id,
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    let frame = OutboundFrame::continue_constant_power_discharge(
                        discharge_power_w,
                        cutoff_voltage_mv,
                        cutoff_time_min,
                        &ebc.info.capabilities,
                    )?;
                    ebc.dev.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::StartConstantCurrentVoltageCharge {
                id,
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    let frame = OutboundFrame::start_constant_current_voltage_charge(
                        charge_current_ma,
                        charge_voltage_mv,
                        cutoff_current_ma,
                        &ebc.info.capabilities,
                    )?;
                    ebc.dev.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::AdjustConstantCurrentVoltageCharge {
                id,
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    let frame = OutboundFrame::adjust_constant_current_voltage_charge(
                        charge_current_ma,
                        charge_voltage_mv,
                        cutoff_current_ma,
                        &ebc.info.capabilities,
                    )?;
                    ebc.dev.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
            EbcCommand::ContinueConstantCurrentVoltageCharge {
                id,
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback,
            } => {
                let result = (|| -> Result<()> {
                    let ebc = self
                        .ebcs
                        .get_mut(&id)
                        .ok_or_else(|| color_eyre::eyre::eyre!("The given id is not connected!"))?;
                    let frame = OutboundFrame::continue_constant_current_voltage_charge(
                        charge_current_ma,
                        charge_voltage_mv,
                        cutoff_current_ma,
                        &ebc.info.capabilities,
                    )?;
                    ebc.dev.send(frame)?;
                    Ok(())
                })();
                callback.send(result).ok();
            }
        }
    }

    async fn read_frames(&mut self) {
        self.ebcs.iter_mut().for_each(|(_, ebc)| {
            ebc.dev.read_frames().unwrap().into_iter().for_each(|f| {
                ebc.report_sender.send(EbcEvent { frame: f.clone() }).ok();
                ebc.latest_report = f;
            });
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

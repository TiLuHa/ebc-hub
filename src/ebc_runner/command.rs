use color_eyre::Result;
use tokio::sync::{broadcast, mpsc, oneshot};

use crate::ebc_runner::Event;

pub enum Command {
    Status {
        callback: oneshot::Sender<Event>,
    },
    SubReports {
        callback: oneshot::Sender<broadcast::Receiver<Event>>,
    },
    Stop {
        callback: oneshot::Sender<Result<()>>,
    },
    StartConstantCurrentDischarge {
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    AdjustConstantCurrentDischarge {
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    ContinueConstantCurrentDischarge {
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    StartConstantPowerDischarge {
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    AdjustConstantPowerDischarge {
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    ContinueConstantPowerDischarge {
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    StartConstantCurrentVoltageCharge {
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    AdjustConstantCurrentVoltageCharge {
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    ContinueConstantCurrentVoltageCharge {
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
        callback: oneshot::Sender<Result<()>>,
    },
}

impl Command {
    pub async fn status(cmd_tx: mpsc::UnboundedSender<Command>) -> Result<Event> {
        let (tx, rx) = oneshot::channel();
        cmd_tx.send(Command::Status { callback: tx }).ok();
        Ok(rx.await?)
    }

    pub async fn sub_reports(
        cmd_tx: mpsc::UnboundedSender<Command>,
    ) -> Result<broadcast::Receiver<Event>> {
        let (tx, rx) = oneshot::channel();
        cmd_tx.send(Command::SubReports { callback: tx }).ok();
        Ok(rx.await?)
    }

    pub async fn stop(cmd_tx: mpsc::UnboundedSender<Command>) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx.send(Command::Stop { callback: tx }).ok();
        rx.await?
    }

    pub async fn start_constant_current_discharge_command(
        cmd_tx: mpsc::UnboundedSender<Command>,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::StartConstantCurrentDischarge {
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn adjust_constant_current_discharge_command(
        cmd_tx: mpsc::UnboundedSender<Command>,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::AdjustConstantCurrentDischarge {
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn continue_constant_current_discharge_command(
        cmd_tx: mpsc::UnboundedSender<Command>,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::ContinueConstantCurrentDischarge {
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn start_constant_power_discharge_command(
        cmd_tx: mpsc::UnboundedSender<Command>,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::StartConstantPowerDischarge {
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn adjust_constant_power_discharge_command(
        cmd_tx: mpsc::UnboundedSender<Command>,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::AdjustConstantPowerDischarge {
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn continue_constant_power_discharge_command(
        cmd_tx: mpsc::UnboundedSender<Command>,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::ContinueConstantPowerDischarge {
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn start_constant_current_voltage_charge_command(
        cmd_tx: mpsc::UnboundedSender<Command>,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::StartConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn adjust_constant_current_voltage_charge_command(
        cmd_tx: mpsc::UnboundedSender<Command>,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::AdjustConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn continue_constant_current_voltage_charge_command(
        cmd_tx: mpsc::UnboundedSender<Command>,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::ContinueConstantCurrentVoltageCharge {
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback: tx,
            })
            .ok();
        rx.await?
    }
}

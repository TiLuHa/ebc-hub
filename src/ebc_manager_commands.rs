use color_eyre::Result;
use tokio::sync::{broadcast, mpsc, oneshot};

use crate::ebc_manager::{DeviceInfo, EbcEvent};

pub enum EbcCommand {
    Connect {
        id: String,
        callback: oneshot::Sender<Result<DeviceInfo>>,
    },
    Disconnect {
        id: String,
        callback: oneshot::Sender<Result<()>>,
    },
    Status {
        id: String,
        callback: oneshot::Sender<Result<EbcEvent>>,
    },
    SubReports {
        id: String,
        callback: oneshot::Sender<Result<broadcast::Receiver<EbcEvent>>>,
    },
    Stop {
        id: String,
        callback: oneshot::Sender<Result<()>>,
    },
    StartConstantCurrentDischarge {
        id: String,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    AdjustConstantCurrentDischarge {
        id: String,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    ContinueConstantCurrentDischarge {
        id: String,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    StartConstantPowerDischarge {
        id: String,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    AdjustConstantPowerDischarge {
        id: String,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    ContinueConstantPowerDischarge {
        id: String,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    StartConstantCurrentVoltageCharge {
        id: String,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    AdjustConstantCurrentVoltageCharge {
        id: String,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
        callback: oneshot::Sender<Result<()>>,
    },
    ContinueConstantCurrentVoltageCharge {
        id: String,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
        callback: oneshot::Sender<Result<()>>,
    },
}

impl EbcCommand {
    pub async fn connect(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
    ) -> Result<DeviceInfo> {
        let (tx, rx) = oneshot::channel();
        cmd_tx.send(EbcCommand::Connect { id, callback: tx }).ok();
        rx.await?
    }

    pub async fn disconnect(cmd_tx: mpsc::UnboundedSender<EbcCommand>, id: String) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::Disconnect { id, callback: tx })
            .ok();
        rx.await?
    }

    pub async fn status(cmd_tx: mpsc::UnboundedSender<EbcCommand>, id: String) -> Result<EbcEvent> {
        let (tx, rx) = oneshot::channel();
        cmd_tx.send(EbcCommand::Status { id, callback: tx }).ok();
        rx.await?
    }

    pub async fn sub_reports(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
    ) -> Result<broadcast::Receiver<EbcEvent>> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::SubReports { id, callback: tx })
            .ok();
        rx.await?
    }

    pub async fn stop(cmd_tx: mpsc::UnboundedSender<EbcCommand>, id: String) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx.send(EbcCommand::Stop { id, callback: tx }).ok();
        rx.await?
    }

    pub async fn start_constant_current_discharge_command(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::StartConstantCurrentDischarge {
                id,
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn adjust_constant_current_discharge_command(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::AdjustConstantCurrentDischarge {
                id,
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn continue_constant_current_discharge_command(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::ContinueConstantCurrentDischarge {
                id,
                discharge_current_ma,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn start_constant_power_discharge_command(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::StartConstantPowerDischarge {
                id,
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn adjust_constant_power_discharge_command(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::AdjustConstantPowerDischarge {
                id,
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn continue_constant_power_discharge_command(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::ContinueConstantPowerDischarge {
                id,
                discharge_power_w,
                cutoff_voltage_mv,
                cutoff_time_min,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn start_constant_current_voltage_charge_command(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::StartConstantCurrentVoltageCharge {
                id,
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn adjust_constant_current_voltage_charge_command(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::AdjustConstantCurrentVoltageCharge {
                id,
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn continue_constant_current_voltage_charge_command(
        cmd_tx: mpsc::UnboundedSender<EbcCommand>,
        id: String,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    ) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(EbcCommand::ContinueConstantCurrentVoltageCharge {
                id,
                charge_current_ma,
                charge_voltage_mv,
                cutoff_current_ma,
                callback: tx,
            })
            .ok();
        rx.await?
    }
}

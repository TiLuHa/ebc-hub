use super::state::State;
use color_eyre::Result;
use tokio::sync::{mpsc, oneshot};

pub enum Command {
    StartTest {
        test_id: i64,
        callback: oneshot::Sender<Result<()>>,
    },
    StopTest {
        callback: oneshot::Sender<Result<()>>,
    },
    ResumeTest {
        callback: oneshot::Sender<Result<()>>,
    },
    Status {
        callback: oneshot::Sender<Result<State>>,
    },
}

impl Command {
    pub async fn start_test(cmd_tx: mpsc::UnboundedSender<Command>, test_id: i64) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx
            .send(Command::StartTest {
                test_id,
                callback: tx,
            })
            .ok();
        rx.await?
    }

    pub async fn stop_test(cmd_tx: mpsc::UnboundedSender<Command>) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx.send(Command::StopTest { callback: tx }).ok();
        rx.await?
    }

    pub async fn resume_test(cmd_tx: mpsc::UnboundedSender<Command>) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        cmd_tx.send(Command::ResumeTest { callback: tx }).ok();
        rx.await?
    }
}

// pub async fn start_constant_current_discharge_command(
//     cmd_tx: mpsc::UnboundedSender<Command>,
//     discharge_current_ma: u16,
//     cutoff_voltage_mv: u16,
//     cutoff_time_min: u16,
// ) -> Result<()> {
//     let (tx, rx) = oneshot::channel();
//     cmd_tx
//         .send(Command::StartConstantCurrentDischarge {
//             discharge_current_ma,
//             cutoff_voltage_mv,
//             cutoff_time_min,
//             callback: tx,
//         })
//         .ok();
//     rx.await?
// }

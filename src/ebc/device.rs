// src/ebc/device.rs
use crate::ebc::frame::InboundFrame;
use crate::ebc::frame::OutboundFrame;
use crate::ebc::frame::process_buffer;
use crate::ebc::report::FirmwareReport;
use color_eyre::Result;
use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;
use std::time::Instant;

pub struct Device {
    port: Box<dyn SerialPort>,
    buffer: Vec<u8>,
}

impl Device {
    pub fn new(path: &str) -> Result<Self> {
        let port = serialport::new(path, 9600)
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::Odd)
            .stop_bits(serialport::StopBits::One)
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_millis(10))
            .open()?;

        Ok(Self {
            port,
            buffer: Vec::new(),
        })
    }

    pub fn send_raw(&mut self, data: &[u8]) -> Result<()> {
        self.port.write_all(data)?;
        self.port.flush()?;
        Ok(())
    }

    pub fn read_raw_once(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 256];

        match self.port.read(&mut buf) {
            Ok(n) => Ok(buf[..n].to_vec()),
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(Vec::new()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn send(&mut self, frame: OutboundFrame) -> Result<()> {
        let raw: [u8; 10] = frame.into();
        self.port.write_all(&raw)?;
        self.port.flush()?;
        Ok(())
    }

    pub fn read_frames(&mut self) -> Result<Vec<InboundFrame>> {
        let mut tmp = [0u8; 256];

        match self.port.read(&mut tmp) {
            Ok(n) => {
                self.buffer.extend_from_slice(&tmp[..n]);
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {}
            Err(e) => return Err(e.into()),
        }

        Ok(process_buffer(&mut self.buffer)
            .into_iter()
            .map(|(frame, _raw)| frame)
            .collect())
    }

    pub fn read_until_firmware_report(&mut self, timeout: Duration) -> Result<FirmwareReport> {
        let deadline = Instant::now() + timeout;

        loop {
            if Instant::now() >= deadline {
                return Err(color_eyre::eyre::eyre!(
                    "Timed out waiting for firmware report"
                ));
            }

            let frames = self.read_frames()?;

            for frame in frames {
                if let InboundFrame::Firmware(report) = frame {
                    return Ok(report);
                }
            }
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        self.send(OutboundFrame::disconnect()).ok();
    }
}

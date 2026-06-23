use crate::ebc::constants::DeviceType;

#[derive(Debug, Clone)]
pub struct DeviceCapabilities {
    min_discharge_current_ma: u16,
    max_discharge_current_ma: u16,
    min_charge_current_ma: u16,
    max_charge_current_ma: u16,
    min_charge_cutoff_current_ma: u16,
    max_charge_cutoff_current_ma: u16,
    min_power_w: u16,
    max_power_w: u16,
    min_voltage_mv: u16,
    max_voltage_mv: u16,
    min_cutoff_time_min: u16,
    max_cutoff_time_min: u16,
    auto_mode_time_min_mins: u16,
    auto_mode_time_max_mins: u16,
}

impl DeviceCapabilities {
    pub fn min_discharge_current_ma(&self) -> u16 {
        self.min_discharge_current_ma
    }
    pub fn max_discharge_current_ma(&self) -> u16 {
        self.max_discharge_current_ma
    }
    pub fn min_charge_current_ma(&self) -> u16 {
        self.min_charge_current_ma
    }
    pub fn max_charge_current_ma(&self) -> u16 {
        self.max_charge_current_ma
    }
    pub fn min_charge_cutoff_current_ma(&self) -> u16 {
        self.min_charge_cutoff_current_ma
    }
    pub fn max_charge_cutoff_current_ma(&self) -> u16 {
        self.max_charge_cutoff_current_ma
    }
    pub fn min_power_w(&self) -> u16 {
        self.min_power_w
    }
    pub fn max_power_w(&self) -> u16 {
        self.max_power_w
    }
    pub fn min_voltage_mv(&self) -> u16 {
        self.min_voltage_mv
    }
    pub fn max_voltage_mv(&self) -> u16 {
        self.max_voltage_mv
    }
    pub fn min_cutoff_time_min(&self) -> u16 {
        self.min_cutoff_time_min
    }
    pub fn max_cutoff_time_min(&self) -> u16 {
        self.max_cutoff_time_min
    }
    pub fn auto_mode_time_min_mins(&self) -> u16 {
        self.auto_mode_time_min_mins
    }
    pub fn auto_mode_time_max_mins(&self) -> u16 {
        self.auto_mode_time_max_mins
    }
}

impl From<DeviceType> for DeviceCapabilities {
    fn from(model: DeviceType) -> Self {
        match model {
            DeviceType::EbcA20 => Self {
                min_discharge_current_ma: 10,
                max_discharge_current_ma: 20000,
                min_charge_current_ma: 10,
                max_charge_current_ma: 5000,
                min_charge_cutoff_current_ma: 10,
                max_charge_cutoff_current_ma: 5000,
                min_power_w: 1,
                max_power_w: 85,
                min_voltage_mv: 10,
                max_voltage_mv: 30000,
                min_cutoff_time_min: 0,
                max_cutoff_time_min: 999,
                auto_mode_time_min_mins: 0,
                auto_mode_time_max_mins: 10,
            },
            _ => todo!(),
        }
    }
}
impl DeviceCapabilities {
    pub fn check_constant_current_discharge_command_parameters(
        &self,
        discharge_current_ma: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> color_eyre::Result<()> {
        if !(self.min_discharge_current_ma()..=self.max_discharge_current_ma())
            .contains(&discharge_current_ma)
        {
            return Err(color_eyre::eyre::eyre!(
                "Current must be between {}mA and {}mA",
                self.min_discharge_current_ma(),
                self.max_discharge_current_ma()
            ));
        }
        if !(self.min_voltage_mv()..=self.max_voltage_mv()).contains(&cutoff_voltage_mv) {
            return Err(color_eyre::eyre::eyre!(
                "Cutoff voltage must be between {}mV and {}mV",
                self.min_voltage_mv(),
                self.max_voltage_mv()
            ));
        }
        if cutoff_time_min > self.max_cutoff_time_min() {
            return Err(color_eyre::eyre::eyre!(
                "Cutoff time must be between 0 and {} minutes",
                self.max_cutoff_time_min()
            ));
        }
        Ok(())
    }

    pub fn check_constant_power_discharge_command_parameters(
        &self,
        discharge_power_w: u16,
        cutoff_voltage_mv: u16,
        cutoff_time_min: u16,
    ) -> color_eyre::Result<()> {
        if !(self.min_power_w()..=self.max_power_w()).contains(&discharge_power_w) {
            return Err(color_eyre::eyre::eyre!(
                "Watts must be between {}W and {}W",
                self.min_power_w(),
                self.max_power_w()
            ));
        }
        if !(self.min_voltage_mv()..=self.max_voltage_mv()).contains(&cutoff_voltage_mv) {
            return Err(color_eyre::eyre::eyre!(
                "Cutoff voltage must be between {}mV and {}mV",
                self.min_voltage_mv(),
                self.max_voltage_mv()
            ));
        }
        if cutoff_time_min > self.max_cutoff_time_min() {
            return Err(color_eyre::eyre::eyre!(
                "Cutoff time must be between 0 and {} minutes",
                self.max_cutoff_time_min()
            ));
        }
        Ok(())
    }
    pub fn check_constant_current_voltage_charge_command_parameters(
        &self,
        charge_current_ma: u16,
        charge_voltage_mv: u16,
        cutoff_current_ma: u16,
    ) -> color_eyre::Result<()> {
        if !(self.min_charge_current_ma()..=self.max_charge_current_ma())
            .contains(&charge_current_ma)
        {
            return Err(color_eyre::eyre::eyre!(
                "Current must be between {}mA and {}mA",
                self.min_charge_current_ma(),
                self.max_charge_current_ma()
            ));
        }
        if !(self.min_voltage_mv()..=self.max_voltage_mv()).contains(&charge_voltage_mv) {
            return Err(color_eyre::eyre::eyre!(
                "Charge voltage must be between {}mV and {}mV",
                self.min_voltage_mv(),
                self.max_voltage_mv()
            ));
        }
        if !(self.min_charge_cutoff_current_ma()..=self.max_charge_cutoff_current_ma())
            .contains(&cutoff_current_ma)
        {
            return Err(color_eyre::eyre::eyre!(
                "Cutoff current must be between {}mA and {}mA",
                self.min_charge_cutoff_current_ma(),
                self.max_charge_cutoff_current_ma()
            ));
        }
        Ok(())
    }
}

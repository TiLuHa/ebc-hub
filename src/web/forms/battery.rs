use serde::{Deserialize, Deserializer};

use crate::db_access::models::{Battery, BatteryIntake, BatteryType};

#[derive(Debug, Deserialize)]
pub struct CreateBatteryTypeForm {
    pub manufacturer: String,
    pub model: String,
    pub chemistry: String,
    pub nominal_voltage_mv: i64,
    pub nominal_capacity_mah: i64,
    pub charge_termination_voltage_mv: i64,
    pub discharge_cutoff_voltage_mv: i64,
}

#[derive(Debug, Deserialize)]
pub struct BatteryTypeForm {
    pub manufacturer: String,
    pub model: String,
    pub chemistry: String,
    pub nominal_voltage_mv: i64,
    pub nominal_capacity_mah: i64,
    pub charge_termination_voltage_mv: i64,
    pub discharge_cutoff_voltage_mv: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBatteryForm {
    pub battery_id: String,
    pub battery_type_id: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBatteryForm {
    pub battery_type_id: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatteryIntakeForm {
    pub serial_number: Option<String>,

    pub purchase_date: Option<String>,
    pub delivery_date: Option<String>,

    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub voltage_at_delivery_mv: Option<i64>,

    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub internal_resistance_at_delivery_uohm: Option<i64>,

    pub visual_inspection: Option<String>,
    pub notes: Option<String>,
}

impl CreateBatteryForm {
    pub fn into_battery(self) -> Battery {
        Battery {
            battery_id: self.battery_id.trim().to_owned(),
            battery_type_id: self.battery_type_id,
            notes: normalize_optional_string(self.notes),
        }
    }
}

impl UpdateBatteryForm {
    pub fn into_battery(
        self,
        battery_id: String,
    ) -> Battery {
        Battery {
            battery_id,
            battery_type_id: self.battery_type_id,
            notes: normalize_optional_string(self.notes),
        }
    }
}

impl BatteryIntakeForm {
    pub fn into_battery_intake(
        self,
        battery_id: String,
    ) -> BatteryIntake {
        BatteryIntake {
            battery_id,

            serial_number:
                normalize_optional_string(self.serial_number),

            purchase_date:
                normalize_optional_string(self.purchase_date),

            delivery_date:
                normalize_optional_string(self.delivery_date),

            voltage_at_delivery_mv:
                self.voltage_at_delivery_mv,

            internal_resistance_at_delivery_uohm:
                self.internal_resistance_at_delivery_uohm,

            visual_inspection:
                normalize_optional_string(self.visual_inspection),

            notes:
                normalize_optional_string(self.notes),
        }
    }
}

impl BatteryTypeForm {
    pub fn into_battery_type(self, id: i64) -> BatteryType {
        BatteryType {
            id,
            manufacturer: self.manufacturer.trim().to_owned(),
            model: self.model.trim().to_owned(),
            chemistry: self.chemistry.trim().to_owned(),
            nominal_voltage_mv: self.nominal_voltage_mv,
            nominal_capacity_mah: self.nominal_capacity_mah,
            charge_termination_voltage_mv: self.charge_termination_voltage_mv,
            discharge_cutoff_voltage_mv: self.discharge_cutoff_voltage_mv,
            notes: self
                .notes
                .map(|notes| notes.trim().to_owned())
                .filter(|notes| !notes.is_empty()),
        }
    }
}

fn normalize_optional_string(
    value: Option<String>,
) -> Option<String> {
    value
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn deserialize_optional_i64<'de, D>(
    deserializer: D,
) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;

    let Some(value) = value else {
        return Ok(None);
    };

    let value = value.trim();

    if value.is_empty() {
        return Ok(None);
    }

    value
        .parse::<i64>()
        .map(Some)
        .map_err(serde::de::Error::custom)
}
use askama::Template;

use crate::db_access::models::BatteryType;

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub title: &'static str,
}

#[derive(Debug, Template)]
#[template(path = "battery_types.html")]
pub struct BatteryTypesTemplate {
    pub title: &'static str,
    pub battery_types: Vec<BatteryType>,
}

#[derive(Debug, Template)]
#[template(path = "battery_type_new.html")]
pub struct NewBatteryTypeTemplate {
    pub title: &'static str,
    pub error: Option<String>,
}

#[derive(Debug, Template)]
#[template(path = "battery_type_detail.html")]
pub struct BatteryTypeDetailTemplate {
    pub title: &'static str,
    pub battery_type: BatteryType,
}

use askama::Template;

use crate::{db_access::models::{Battery, BatteryIntake, BatteryType}, web::view_models::BatteryListItem};

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


#[derive(Debug, Template)]
#[template(path = "batteries.html")]
pub struct BatteriesTemplate {
    pub batteries: Vec<BatteryListItem>,
}

#[derive(Debug, Template)]
#[template(path = "new_battery.html")]
pub struct NewBatteryTemplate {
    pub battery_types: Vec<BatteryType>,
}

#[derive(Debug, Template)]
#[template(path = "battery_detail.html")]
pub struct BatteryDetailTemplate {
    pub battery: Battery,
    pub battery_type: BatteryType,
    pub battery_types: Vec<BatteryType>,
    pub intake: Option<BatteryIntake>,
}
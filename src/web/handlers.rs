use askama::Template;
use axum::{
    extract::{Form, Path, State}, response::{Html, Redirect},
};

use crate::web::{AppError, forms::{BatteryIntakeForm, CreateBatteryForm, UpdateBatteryForm, battery::{BatteryTypeForm, CreateBatteryTypeForm}}, templates::{BatteriesTemplate, BatteryDetailTemplate, BatteryTypeDetailTemplate, NewBatteryTemplate}, validation::battery::{validate_battery_intake, validate_create_battery, validate_update_battery}};

use super::{
    AppState,
    templates::{BatteryTypesTemplate, IndexTemplate, NewBatteryTypeTemplate},
};

pub async fn index(State(_state): State<AppState>) -> Result<Html<String>, AppError> {
    render(IndexTemplate { title: "Open battery forge" })
}

pub async fn battery_types(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let battery_types = state
        .storage
        .list_battery_types()
        .await?;

    render(BatteryTypesTemplate {
        title: "Batterietypen",
        battery_types,
    })
}

pub async fn new_battery_type(State(_state): State<AppState>) -> Result<Html<String>, AppError> {
    render(NewBatteryTypeTemplate {
        title: "Batterietyp anlegen",
        error: None,
    })
}

pub async fn create_battery_type(
    State(state): State<AppState>,
    Form(form): Form<CreateBatteryTypeForm>,
) -> Result<Redirect, AppError> {
    let manufacturer = form.manufacturer.trim();
    let model = form.model.trim();
    let chemistry = form.chemistry.trim();

    if manufacturer.is_empty() {
        return Err(AppError::bad_request(
            "Der Hersteller darf nicht leer sein.",
        ));
    }

    if model.is_empty() {
        return Err(AppError::bad_request("Das Modell darf nicht leer sein."));
    }

    if chemistry.is_empty() {
        return Err(AppError::bad_request(
            "Die Zellchemie muss ausgewählt werden.",
        ));
    }

    if form.nominal_voltage_mv <= 0 {
        return Err(AppError::bad_request(
            "Die Nennspannung muss größer als 0 sein.",
        ));
    }

    if form.nominal_capacity_mah <= 0 {
        return Err(AppError::bad_request(
            "Die Nennkapazität muss größer als 0 sein.",
        ));
    }

    state
        .storage
        .create_battery_type(
            manufacturer,
            model,
            chemistry,
            form.nominal_voltage_mv,
            form.nominal_capacity_mah,
            form.charge_termination_voltage_mv,
            form.discharge_cutoff_voltage_mv,
        )
        .await?;

    Ok(Redirect::to("/battery-types"))
}

pub async fn battery_type_detail(
    State(state): State<AppState>,
    Path(battery_type_id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let battery_type = state
        .storage
        .get_battery_type(battery_type_id)
        .await?
        .ok_or_else(|| AppError::not_found("Der Batterietyp wurde nicht gefunden."))?;

    render(BatteryTypeDetailTemplate {
        title: "Batterietyp",
        battery_type,
    })
}

fn validate_battery_type_form(
    form: &BatteryTypeForm,
) -> Result<(), AppError> {
    if form.manufacturer.trim().is_empty() {
        return Err(AppError::bad_request(
            "Der Hersteller darf nicht leer sein.",
        ));
    }

    if form.model.trim().is_empty() {
        return Err(AppError::bad_request(
            "Das Modell darf nicht leer sein.",
        ));
    }

    if form.chemistry.trim().is_empty() {
        return Err(AppError::bad_request(
            "Die Chemie darf nicht leer sein.",
        ));
    }

    if form.nominal_voltage_mv <= 0 {
        return Err(AppError::bad_request(
            "Die Nennspannung muss größer als 0 sein.",
        ));
    }

    if form.nominal_capacity_mah <= 0 {
        return Err(AppError::bad_request(
            "Die Nennkapazität muss größer als 0 sein.",
        ));
    }

    if form.charge_termination_voltage_mv
        <= form.nominal_voltage_mv
    {
        return Err(AppError::bad_request(
            "Die Ladeschlussspannung muss über der Nennspannung liegen.",
        ));
    }

    if form.discharge_cutoff_voltage_mv
        >= form.nominal_voltage_mv
    {
        return Err(AppError::bad_request(
            "Die Entladeschlussspannung muss unter der Nennspannung liegen.",
        ));
    }

    Ok(())
}

pub async fn update_battery_type(
    State(state): State<AppState>,
    Path(battery_type_id): Path<i64>,
    Form(form): Form<BatteryTypeForm>,
) -> Result<Redirect, AppError> {
    validate_battery_type_form(&form)?;

    let battery_type = form.into_battery_type(battery_type_id);

    state
        .storage
        .update_battery_type(&battery_type)
        .await?;

    Ok(Redirect::to(&format!(
        "/battery-types/{}",
        battery_type.id
    )))
}

fn render<T>(template: T) -> Result<Html<String>, AppError>
where
    T: Template,
{
    let html = template.render()?;
    Ok(Html(html))
}

pub async fn batteries(
    State(state): State<AppState>,
) -> Result<Html<String>, AppError> {
    let batteries = state
        .storage
        .list_batteries()
        .await?;

    render(BatteriesTemplate {
        batteries,
    })
}

pub async fn new_battery(
    State(state): State<AppState>,
) -> Result<Html<String>, AppError> {
    let battery_types = state
        .storage
        .list_battery_types()
        .await?;

    render(NewBatteryTemplate {
        battery_types,
    })
}

pub async fn create_battery(
    State(state): State<AppState>,
    Form(form): Form<CreateBatteryForm>,
) -> Result<Redirect, AppError> {
    validate_create_battery(&form)?;

    let battery = form.into_battery();

    state
        .storage
        .create_battery(&battery)
        .await?;

    Ok(Redirect::to(&format!(
        "/batteries/{}",
        battery.battery_id
    )))
}

pub async fn battery_detail(
    State(state): State<AppState>,
    Path(battery_id): Path<String>,
) -> Result<Html<String>, AppError> {
    let battery = state
        .storage
        .get_battery(&battery_id)
        .await?
        .ok_or_else(|| {
            AppError::not_found(format!(
                "Die Batterie '{battery_id}' wurde nicht gefunden."
            ))
        })?;

    let battery_type = state
        .storage
        .get_battery_type(battery.battery_type_id)
        .await?
        .ok_or_else(|| {
            AppError::not_found(format!(
                "Der Batterietyp {} wurde nicht gefunden.",
                battery.battery_type_id
            ))
        })?;

    let battery_types = state
        .storage
        .list_battery_types()
        .await?;

    let intake = state
        .storage
        .get_battery_intake(&battery_id)
        .await?;

    render(BatteryDetailTemplate {
        battery,
        battery_type,
        battery_types,
        intake,
    })
}

pub async fn update_battery(
    State(state): State<AppState>,
    Path(battery_id): Path<String>,
    Form(form): Form<UpdateBatteryForm>,
) -> Result<Redirect, AppError> {
    validate_update_battery(&form)?;

    let battery = form.into_battery(
        battery_id.clone(),
    );

    state
        .storage
        .update_battery(&battery)
        .await?;

    Ok(Redirect::to(&format!(
        "/batteries/{battery_id}"
    )))
}

pub async fn save_battery_intake(
    State(state): State<AppState>,
    Path(battery_id): Path<String>,
    Form(form): Form<BatteryIntakeForm>,
) -> Result<Redirect, AppError> {
    validate_battery_intake(&form)?;

    let battery_exists = state
        .storage
        .get_battery(&battery_id)
        .await?
        .is_some();

    if !battery_exists {
        return Err(AppError::not_found(format!(
            "Die Batterie '{battery_id}' wurde nicht gefunden."
        )));
    }

    let intake = form.into_battery_intake(
        battery_id.clone(),
    );

    state
        .storage
        .upsert_battery_intake(&intake)
        .await?;

    Ok(Redirect::to(&format!(
        "/batteries/{battery_id}"
    )))
}
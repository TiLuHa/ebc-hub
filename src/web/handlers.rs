use askama::Template;
use axum::{
    extract::{Form, Path, State}, http::StatusCode, response::{Html, IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::{db_access::models::BatteryType, web::templates::BatteryTypeDetailTemplate};

use super::{
    AppState,
    templates::{BatteryTypesTemplate, IndexTemplate, NewBatteryTypeTemplate},
};

pub async fn index(State(_state): State<AppState>) -> Result<Html<String>, AppError> {
    render(IndexTemplate { title: "EBC Hub" })
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
        .get_battery_type(&battery_type_id.to_string())
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

#[derive(Debug)]
pub struct AppError {
    status: StatusCode,
    message: String,
    error: Option<color_eyre::Report>,
}

impl AppError {
    pub fn internal(error: impl Into<color_eyre::Report>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Die Anfrage konnte nicht verarbeitet werden.".to_owned(),
            error: Some(error.into()),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
            error: None,
        }
    }
    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
            error: None,
        }
    }
}

impl From<color_eyre::Report> for AppError {
    fn from(error: color_eyre::Report) -> Self {
        Self::internal(error)
    }
}

impl From<askama::Error> for AppError {
    fn from(error: askama::Error) -> Self {
        Self::internal(error)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::internal(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        if let Some(error) = &self.error {
            tracing::error!(
                error = ?error,
                status = %self.status,
                "HTTP request failed"
            );
        } else {
            tracing::warn!(
                status = %self.status,
                message = %self.message,
                "invalid HTTP request"
            );
        }

        let heading = match self.status {
            StatusCode::BAD_REQUEST => "Ungültige Eingabe",
            StatusCode::NOT_FOUND => "Nicht gefunden",
            _ => "Interner Fehler",
        };

        let (back_link, back_text) = match self.status {
            StatusCode::BAD_REQUEST => (
                "/battery-types/new",
                "Zurück zum Formular",
            ),
            StatusCode::NOT_FOUND => (
                "/battery-types",
                "Zurück zu den Batterietypen",
            ),
            _ => (
                "/",
                "Zurück zur Startseite",
            ),
        };

        let html = format!(
            r#"
            <!doctype html>
            <html lang="de">
                <head>
                    <meta charset="utf-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <title>{heading} – EBC Hub</title>
                </head>
                <body>
                    <main>
                        <h1>{heading}</h1>
                        <p>{message}</p>
                        <p>
                            <a href="{back_link}">{back_text}</a>
                        </p>
                    </main>
                </body>
            </html>
            "#,
            heading = heading,
            message = self.message,
            back_link = back_link,
            back_text = back_text,
        );

        (self.status, Html(html)).into_response()
    }
}

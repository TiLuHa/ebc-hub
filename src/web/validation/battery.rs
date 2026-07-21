use crate::web::{
    forms::{
        BatteryIntakeForm,
        CreateBatteryForm,
        UpdateBatteryForm,
    },
    AppError,
};

pub fn validate_create_battery(
    form: &CreateBatteryForm,
) -> Result<(), AppError> {
    validate_battery_id(&form.battery_id)?;

    if form.battery_type_id <= 0 {
        return Err(AppError::bad_request(
            "Es muss ein gültiger Batterietyp ausgewählt werden.",
        ));
    }

    Ok(())
}

pub fn validate_update_battery(
    form: &UpdateBatteryForm,
) -> Result<(), AppError> {
    if form.battery_type_id <= 0 {
        return Err(AppError::bad_request(
            "Es muss ein gültiger Batterietyp ausgewählt werden.",
        ));
    }

    Ok(())
}

pub fn validate_battery_intake(
    form: &BatteryIntakeForm,
) -> Result<(), AppError> {
    if form
        .voltage_at_delivery_mv
        .is_some_and(|value| value <= 0)
    {
        return Err(AppError::bad_request(
            "Die Spannung bei Lieferung muss größer als 0 mV sein.",
        ));
    }

    if form
        .internal_resistance_at_delivery_uohm
        .is_some_and(|value| value < 0)
    {
        return Err(AppError::bad_request(
            "Der Innenwiderstand darf nicht negativ sein.",
        ));
    }

    Ok(())
}

fn validate_battery_id(
    battery_id: &str,
) -> Result<(), AppError> {
    let battery_id = battery_id.trim();

    if battery_id.is_empty() {
        return Err(AppError::bad_request(
            "Die Batterie-ID darf nicht leer sein.",
        ));
    }

    if battery_id.len() > 100 {
        return Err(AppError::bad_request(
            "Die Batterie-ID darf höchstens 100 Zeichen enthalten.",
        ));
    }

    let is_valid = battery_id
        .chars()
        .all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(
                    character,
                    '-' | '_' | '.'
                )
        });

    if !is_valid {
        return Err(AppError::bad_request(
            "Die Batterie-ID darf nur Buchstaben, Zahlen, Bindestriche, Unterstriche und Punkte enthalten.",
        ));
    }

    Ok(())
}
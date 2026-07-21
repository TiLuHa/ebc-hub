use axum::{http::StatusCode, response::{Html, IntoResponse, Response}};

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
                    <title>{heading} – Open battery forge</title>
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

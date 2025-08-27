use axum::{
    body::Bytes,
    extract::{FromRequest, Multipart},
};

use crate::{errors::RequestError, prelude::ApplicationConfig, web::Context};

#[derive(Debug)]
pub struct MultipartField {
    pub name: Option<String>,
    pub file_name: Option<String>,
    pub content_type: Option<String>,
    pub data: Bytes,
}

#[derive(Debug)]
pub struct MultipartData {
    fields: Vec<MultipartField>,
}

impl MultipartData {
    pub(crate) fn new(fields: Vec<MultipartField>) -> Self {
        Self { fields }
    }

    pub fn fields(&self) -> &Vec<MultipartField> {
        &self.fields
    }

    pub fn into_fields(self) -> Vec<MultipartField> {
        self.fields
    }
}

impl Context {
    pub async fn multipart(&self) -> Result<MultipartData, RequestError> {
        let mut multipart = Multipart::from_request(self.clone().try_into()?, &())
            .await
            .map_err(|err| {
                RequestError::ParseError(
                    "Failed to parse multipart data",
                    format!("Error parsing multipart: {err}"),
                )
            })?;

        let allowed_mime_types = self
            .config::<ApplicationConfig>()
            .map_err(|e| {
                eprintln!("Error retrieving application config: {e}");
                RequestError::InternalError("Failed to retrieve application config".to_string())
            })?
            .allowed_mime_types
            .clone();

        let mut fields = Vec::new();

        while let Some(field) = multipart.next_field().await.map_err(|err| {
            RequestError::ParseError(
                "Failed to read multipart field",
                format!("Error reading field: {err}"),
            )
        })? {
            let name = field.name().map(|s| s.to_string());
            let file_name = field.file_name().map(|s| s.to_string());
            let content_type = field.content_type().map(|s| s.to_string());
            let data = field.bytes().await.map_err(|err| {
                RequestError::ParseError(
                    "Failed to read field data",
                    format!("Error reading bytes: {err}"),
                )
            })?;

            if file_name.is_some()
                && let Some(kind) = infer::get(&data)
            {
                let mime_type = kind.mime_type().to_string();

                if !allowed_mime_types.contains(&mime_type) {
                    return Err(RequestError::UnsupportedMediaType(format!(
                        "MIME type \"{mime_type}\" is not allowed",
                    )));
                }
            }

            fields.push(MultipartField {
                name,
                file_name,
                content_type,
                data,
            });
        }

        Ok(MultipartData::new(fields))
    }
}

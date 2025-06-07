use std::{collections::HashMap, str::FromStr};

use axum::{
    body::{Bytes, to_bytes},
    extract::{FromRequest, OptionalFromRequestParts, Path, Request as AxumRequest},
};

use serde_json::json;
use validator::Validate;

use crate::http::{HttpResponse, Result};
use serde::de::DeserializeOwned;

pub struct Request {
    params: HashMap<String, String>,
    body_bytes: Option<Bytes>,
    method: String,
    headers: HashMap<String, String>,
    query_string: Option<String>,
    uri: String,
}

impl<S> FromRequest<S> for Request
where
    S: Send + Sync + Clone,
{
    type Rejection = HttpResponse;

    async fn from_request(req: AxumRequest, _: &S) -> Result<Self> {
        let (mut parts, body) = req.into_parts();

        //-------------------------------------------------------

        let mut params = HashMap::new();
        let path_result =
            Path::<HashMap<String, String>>::from_request_parts(&mut parts, &()).await;

        if let Ok(Some(path_params)) = path_result {
            params.extend(path_params.0);
        }

        //-------------------------------------------------------

        let body_bytes = to_bytes(body, usize::MAX).await.ok();

        //-------------------------------------------------------

        let mut headers = HashMap::new();

        for (key, value) in parts.headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        //-------------------------------------------------------

        let query_string = parts.uri.query().map(|q| q.to_string());

        //-------------------------------------------------------

        Ok(Self {
            params,
            body_bytes,
            method: parts.method.to_string(),
            headers,
            query_string,
            uri: parts.uri.to_string(),
        })
    }
}

impl Request {
    pub fn param<T: FromStr>(&self, key: &str) -> Result<T> {
        if let Some(value) = self.params.get(key) {
            let Ok(param) = value.parse::<T>() else {
                return Err(
                    HttpResponse::BadRequest().message(format!("Invalid parameter: {}", key))
                );
            };

            return Ok(param);
        }

        Err(HttpResponse::NotFound().message(format!("Parameter not found: {}", key)))
    }

    pub fn body<T: DeserializeOwned>(&self) -> Result<T> {
        if self.method == "GET" || self.method == "HEAD" {
            return Err(HttpResponse::BadRequest()
                .message(format!("Method {} does not support body", self.method)));
        }

        let Some(bytes) = &self.body_bytes else {
            return Err(HttpResponse::BadRequest().message("Missing request body"));
        };

        if bytes.is_empty() {
            return Err(HttpResponse::BadRequest().message("Empty request body"));
        }

        serde_json::from_slice(bytes).map_err(|err| {
            HttpResponse::BadRequest().message(format!("Invalid JSON body: {}", err))
        })
    }

    pub fn header(&self, key: &str) -> Option<String> {
        self.headers.get(key).cloned()
    }

    pub fn query<T>(&self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let Some(query_str) = &self.query_string else {
            return serde_qs::from_str("").map_err(|err| {
                HttpResponse::BadRequest().message(format!("Invalid query parameters: {}", err))
            });
        };

        serde_qs::from_str(query_str).map_err(|err| {
            HttpResponse::BadRequest().message(format!("Invalid query parameters: {}", err))
        })
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn validated_body<T>(&self) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Validate,
    {
        let body = self.body::<T>()?;

        body.validate().map_err(|error| {
            HttpResponse::BadRequest()
                .message("Invalid request body")
                .data(json!({
                    "type": "ValidationError",
                    "errors":  crate::validation::format_errors(&error)
                }))
        })?;

        Ok(body)
    }

    pub fn validated_query<T>(&self) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Validate,
    {
        let query = self.query::<T>()?;

        query.validate().map_err(|error| {
            HttpResponse::BadRequest()
                .message("Invalid query parameters")
                .data(json!({
                    "type": "ValidationError",
                    "errors": crate::validation::format_errors(&error)
                }))
        })?;

        Ok(query)
    }
}

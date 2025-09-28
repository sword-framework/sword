use crate::{errors::RequestError, web::Context};
use garde::Validate;
use serde::de::DeserializeOwned;

pub trait GardeRequestValidation {
    fn body_garde<T: DeserializeOwned + Validate>(&self) -> Result<T, RequestError>
    where
        <T as Validate>::Context: Default;

    fn query_garde<T: DeserializeOwned + Validate>(
        &self,
    ) -> Result<Option<T>, RequestError>
    where
        <T as Validate>::Context: Default;

    fn params_garde<T: DeserializeOwned + Validate>(
        &self,
    ) -> Result<T, RequestError>
    where
        <T as Validate>::Context: Default;
}

impl GardeRequestValidation for Context {
    fn body_garde<T: DeserializeOwned + Validate>(&self) -> Result<T, RequestError>
    where
        <T as Validate>::Context: Default,
    {
        let body = self.body::<T>()?;

        body.validate().map_err(|report| {
            RequestError::GardeError("Invalid request body", report)
        })?;

        Ok(body)
    }

    fn query_garde<T: DeserializeOwned + Validate>(
        &self,
    ) -> Result<Option<T>, RequestError>
    where
        <T as Validate>::Context: Default,
    {
        match self.query::<T>()? {
            Some(query) => {
                query.validate().map_err(|report| {
                    RequestError::GardeError("Invalid request query", report)
                })?;

                Ok(Some(query))
            }
            None => Ok(None),
        }
    }

    fn params_garde<T: DeserializeOwned + Validate>(&self) -> Result<T, RequestError>
    where
        <T as Validate>::Context: Default,
    {
        let params = serde_json::to_value(self.params.clone()).map_err(|e| {
            RequestError::ParseError("Failed to serialize params", e.to_string())
        })?;

        let deserialized: T = serde_json::from_value(params).map_err(|e| {
            RequestError::ParseError(
                "Failed to deserialize params to the target type",
                e.to_string(),
            )
        })?;

        deserialized.validate().map_err(|report| {
            RequestError::GardeError("Invalid request params", report)
        })?;

        Ok(deserialized)
    }
}

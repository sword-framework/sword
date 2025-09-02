pub mod request;

#[cfg(feature = "multipart")]
pub mod multipart;

use axum::{
    body::Bytes,
    http::{Extensions, Method, Uri},
};

use serde::de::DeserializeOwned;
use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "shaku-di")]
use shaku::{HasComponent, Interface, Module};

use crate::{
    core::State,
    core::{Config, ConfigItem},
    errors::{ConfigError, StateError},
};

/// Context represents the incoming request context in the Sword framework.
/// It contains request parameters, body bytes, HTTP method, headers, URI and more.
///
/// Also contains the state of the application, which can be used to store and retrieve
/// shared data across the application, such as database connections, configuration settings,
/// and other application state.
#[derive(Debug, Clone)]
pub struct Context {
    params: HashMap<String, String>,
    body_bytes: Bytes,
    method: Method,
    headers: HashMap<String, String>,
    uri: Uri,
    state: State,
    pub extensions: Extensions,
}

impl Context {
    /// Method to get the `T` type from the context state.
    /// This method will return an error if the type is not found in the state.
    /// The error can be converted automatically to a `HttpResponse` using `?` operator.`
    pub fn get_state<T>(&self) -> Result<Arc<T>, StateError>
    where
        T: Send + Sync + 'static + Clone,
    {
        let value = self.state.get::<T>().map_err(|_| StateError::TypeNotFound)?;

        Ok(value)
    }

    /// Method to get the `T` type from the `shaku` dependency injection module.
    /// This method will return an error if the type is not found in the module.
    /// The error can be converted automatically to a `HttpResponse` using `?` operator
    ///
    /// Parameter `M` is the module type that contains the dependency.
    /// Parameter `I` is the interface type that is being resolved. (must be a `dyn Interface`)
    #[cfg(feature = "shaku-di")]
    pub fn di<M, I>(&self) -> Result<Arc<I>, StateError>
    where
        M: Module + HasComponent<I> + Send + Sync + 'static,
        I: Interface + ?Sized + 'static,
    {
        let module = self.state.get::<M>().map_err(|_| StateError::TypeNotFound)?;

        let interface = module.resolve();

        Ok(interface)
    }

    pub fn config<T: DeserializeOwned + ConfigItem>(&self) -> Result<T, ConfigError> {
        let config = self
            .state
            .get::<Config>()
            .map_err(|e| ConfigError::GetConfigError(e.to_string()))?;

        config.get::<T>()
    }
}

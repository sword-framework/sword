pub mod request;

use std::{collections::HashMap, sync::Arc};

use axum::{
    body::Bytes,
    http::{Extensions, Method, Uri},
};
use shaku::{HasComponent, Interface, Module};

use crate::{application::SwordState, http::errors::ContextError};

/// Context represents the incoming request context in the Sword framework.
/// It contains request parameters, body bytes, HTTP method, headers, URI and more.
///
/// Also contains the state of the application, which can be used to store and retrieve
/// shared data across the application, such as database connections, configuration settings,
/// and other application state.
pub struct Context {
    params: HashMap<String, String>,
    body_bytes: Bytes,
    method: Method,
    headers: HashMap<String, String>,
    uri: Uri,
    state: SwordState,
    pub extensions: Extensions,
}

impl Context {
    /// Method to get the `T` type from the context state.
    /// This method will return an error if the type is not found in the state.
    /// The error can be converted automatically to a `HttpResponse` using `?` operator.`
    pub fn get_state<T>(&self) -> Result<T, ContextError>
    where
        T: Send + Sync + 'static + Clone,
    {
        let value = self
            .state
            .get::<T>()
            .cloned()
            .ok_or_else(|| ContextError::StateNotFound(std::any::type_name::<T>()))?;

        Ok(value)
    }

    /// Method to get the `T` type from the `shaku` dependency injection module.
    /// This method will return an error if the type is not found in the module.
    /// The error can be converted automatically to a `HttpResponse` using `?` operator
    ///
    /// Parameter `M` is the module type that contains the dependency.
    /// Parameter `I` is the interface type that is being resolved. (must be a `dyn Interface`)
    pub fn get_dependency<M, I>(&self) -> Result<Arc<I>, ContextError>
    where
        M: Module + HasComponent<I> + Send + Sync + 'static,
        I: Interface + ?Sized + 'static,
    {
        let module = self
            .state
            .get::<M>()
            .ok_or_else(|| ContextError::DependencyNotFound(std::any::type_name::<I>()))?;

        let interface = module.resolve();

        Ok(interface)
    }
}

pub mod request;

use std::{collections::HashMap, sync::Arc};

use axum::{
    body::Bytes,
    http::{Extensions, Method, Uri},
};
use shaku::{HasComponent, Interface, Module};

use crate::{application::SwordState, http::errors::ContextError};

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

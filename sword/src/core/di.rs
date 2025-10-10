use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use crate::core::State;

type Dependency = Arc<dyn Any + Send + Sync>;
type DependencyBuilder = Box<dyn Fn(&State) -> Dependency>;

pub trait Injectable {
    fn build(state: &State) -> Self;
    fn dependencies() -> Vec<TypeId>;
}

pub struct DependencyContainer {
    pub(crate) instances: HashMap<TypeId, Dependency>,
    pub(crate) dependency_builders: HashMap<TypeId, DependencyBuilder>,
    pub(crate) dependency_graph: HashMap<TypeId, Vec<TypeId>>,
}

impl DependencyContainer {
    pub fn builder() -> Self {
        Self {
            instances: HashMap::new(),
            dependency_builders: HashMap::new(),
            dependency_graph: HashMap::new(),
        }
    }

    pub fn register<T>(mut self) -> Self
    where
        T: Injectable + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();

        let dependency_builder =
            Box::new(|state: &State| Arc::new(T::build(state)) as Dependency);

        self.dependency_graph.insert(type_id, T::dependencies());
        self.dependency_builders.insert(type_id, dependency_builder);

        self
    }

    pub fn register_instance<T>(mut self, instance: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.instances.insert(TypeId::of::<T>(), Arc::new(instance));
        self
    }

    pub fn build(self) -> Self {
        self
    }
}

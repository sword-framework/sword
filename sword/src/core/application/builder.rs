use std::convert::Infallible;

use axum::{
    extract::Request as AxumRequest,
    response::IntoResponse,
    routing::{Route, Router},
};

use tower::{Layer, Service};
use tower_http::limit::RequestBodyLimitLayer;

#[cfg(feature = "cookies")]
use tower_cookies::CookieManagerLayer;

use crate::{
    __private::mw_with_state,
    core::{
        application::{Application, ApplicationConfig},
        config::Config,
        router::RouterProvider,
        state::State,
    },
    errors::{ApplicationError, StateError},
    web::ContentTypeCheck,
};

#[derive(Debug, Clone)]
pub struct ApplicationBuilder {
    router: Router,
    state: State,
    pub config: Config,
}

impl ApplicationBuilder {
    pub fn new() -> Result<Self, ApplicationError> {
        let state = State::new();
        let config = Config::new()?;

        state.insert(config.clone()).unwrap();

        let router = Router::new().with_state(state.clone());

        Ok(Self {
            router,
            state,
            config,
        })
    }

    /// Register a controller in the application.
    /// This method allows you to add a controller to the application's router.
    pub fn with_controller<R: RouterProvider>(self) -> Self {
        let controller_router = R::router(self.state.clone());
        let router = self.router.clone().merge(controller_router);

        Self {
            router,
            state: self.state,
            config: self.config,
        }
    }

    /// Register a layer in the application.
    /// This method allows you to add middleware or other layers to the application's router.
    /// This is useful to add tower based middleware or other layers that implement the `Layer` trait.
    pub fn with_layer<L>(self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + Sync + 'static,
        L::Service: Service<AxumRequest> + Clone + Send + Sync + 'static,
        <L::Service as Service<AxumRequest>>::Response: IntoResponse + 'static,
        <L::Service as Service<AxumRequest>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<AxumRequest>>::Future: Send + 'static,
    {
        let router = self.router.layer(layer);

        Self {
            router,
            state: self.state,
            config: self.config,
        }
    }

    /// Register a state in the application.
    /// This method allows you to add a shared state to the application's router.
    /// The state can be any type that implements `Sync` and `Send`, and is
    /// stored in the application's state.
    ///
    /// It's not necesary to use your state wrapped in `Arc`, as the sword `State`
    /// already does that for you.
    pub fn with_state<S: Sync + Send + 'static>(self, state: S) -> Result<Self, StateError> {
        self.state.insert(state)?;

        let router = Router::new().with_state(self.state.clone());

        Ok(Self {
            router,
            state: self.state,
            config: self.config,
        })
    }

    /// Register a dependency injection module in the application.
    /// This method allows you to add a Shaku module to the application's state.
    /// Behind the scenes, it will register the module in the sword `State` so you can
    /// retrieve it later using the `get_dependency` method.
    #[cfg(feature = "shaku-di")]
    pub fn with_shaku_di_module<M: Sync + Send + 'static>(
        self,
        module: M,
    ) -> Result<Self, StateError> {
        self.with_state(module)
    }

    pub fn build(self) -> Application {
        let mut router = self.router.clone();
        let app_config = self.config.get::<ApplicationConfig>().unwrap();

        router = router
            .layer(mw_with_state(self.state.clone(), ContentTypeCheck::layer))
            .layer(RequestBodyLimitLayer::new(app_config.body_limit));

        if cfg!(feature = "cookies") {
            router = router.layer(CookieManagerLayer::new());
        }

        Application {
            router,
            config: self.config,
        }
    }
}

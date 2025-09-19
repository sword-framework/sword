use axum::{Router as AxumRouter, response::IntoResponse, routing::get};
use std::sync::Arc;
use sword::{
    core::State,
    web::{Context, Controller, ControllerError, HttpResponse},
};

#[derive(Clone)]
pub struct ControllerExample {
    u32_number: u32,
}

impl ControllerExample {
    async fn get_number(&self, ctx: Context) -> HttpResponse {
        let num = self.u32_number;
        println!("Number from state: {}", ctx.get_state::<u32>().unwrap());

        HttpResponse::Ok().message(format!("Number: {}", num))
    }
}

impl Controller for ControllerExample {
    fn build_from_state(state: State) -> Result<Self, ControllerError> {
        let u32_number = state.get::<u32>().map_err(|e| {
            ControllerError::StateExtractionError(format!(
                "Failed to extract u32 from state: {}",
                e
            ))
        })?;

        Ok(ControllerExample {
            u32_number: *u32_number,
        })
    }

    fn router(state: State) -> AxumRouter {
        let controller_result = Arc::new(Self::build_from_state(state.clone()));

        let handler = move |ctx: Context| {
            let controller_result = Arc::clone(&controller_result);

            async move {
                match controller_result.as_ref() {
                    Err(e) => HttpResponse::InternalServerError()
                        .message(format!("Controller build error: {e}"))
                        .into_response(),
                    Ok(controller) => {
                        controller.get_number(ctx).await.into_response()
                    }
                }
            }
        };

        AxumRouter::new()
            .route("/number", get(handler))
            .with_state(state)
    }
}

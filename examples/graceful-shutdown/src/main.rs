use sword::prelude::*;
use tokio::signal;

#[controller("/")]
struct AppController {}

#[routes]
impl AppController {
    #[get("/")]
    async fn get_data() -> HttpResponse {
        HttpResponse::Ok()
    }
}

#[controller("/admin")]
struct AdminController {}

#[routes]
impl AdminController {
    #[get("/")]
    async fn get_admin_data() -> HttpResponse {
        HttpResponse::Ok()
    }
}

#[sword::main]
async fn main() {
    let app = Application::builder()?
        .with_controller::<AppController>()
        .with_controller::<AdminController>()
        .build();

    app.run_with_graceful_shutdown(shutdown_signal()).await?;
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

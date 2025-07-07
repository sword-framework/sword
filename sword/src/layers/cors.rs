use tower_http::cors::CorsLayer;

pub struct CorsConfig {
    allowed_http_methods: Vec<String>,
    allowed_http_headers: Vec<String>,
    allow_credentials: bool,
}

pub struct Cors {
    layer: CorsLayer,
}

impl Cors {
    pub fn new(_: CorsConfig) -> Self {
        unimplemented!("Implement Cors layer with the provided config");
    }
}

use axum_test::TestServer;
use sword::prelude::*;

use sword::web::helmet::{
    ContentSecurityPolicy, CrossOriginEmbedderPolicy, CrossOriginOpenerPolicy,
    CrossOriginResourcePolicy, Helmet, OriginAgentCluster, ReferrerPolicy,
    StrictTransportSecurity, XContentTypeOptions, XDNSPrefetchControl,
    XDownloadOptions, XFrameOptions, XPermittedCrossDomainPolicies, XPoweredBy,
    XXSSProtection,
};

#[controller("/test")]
struct HelmetTestController;

#[routes]
impl HelmetTestController {
    #[get("/")]
    async fn index(&self, _: Context) -> HttpResponse {
        HttpResponse::Ok().message("Hello from helmet test")
    }
}

#[tokio::test]
async fn test_basic_security_headers() -> Result<(), Box<dyn std::error::Error>> {
    let helmet = Helmet::builder()
        .with_header(XContentTypeOptions::nosniff())
        .with_header(XFrameOptions::deny())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);

    let headers = response.headers();

    assert_eq!(headers.get("x-content-type-options").unwrap(), "nosniff");
    assert_eq!(headers.get("x-frame-options").unwrap(), "DENY");

    Ok(())
}

#[tokio::test]
async fn test_x_content_type_options() -> Result<(), Box<dyn std::error::Error>> {
    let helmet = Helmet::builder()
        .with_header(XContentTypeOptions::nosniff())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("x-content-type-options").unwrap(),
        "nosniff"
    );

    Ok(())
}

#[tokio::test]
async fn test_x_frame_options() -> Result<(), Box<dyn std::error::Error>> {
    // Test DENY
    let helmet_deny = Helmet::builder().with_header(XFrameOptions::deny()).build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet_deny)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(response.headers().get("x-frame-options").unwrap(), "DENY");

    // Test SAMEORIGIN
    let helmet_sameorigin = Helmet::builder()
        .with_header(XFrameOptions::same_origin())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet_sameorigin)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("x-frame-options").unwrap(),
        "SAMEORIGIN"
    );

    Ok(())
}

#[tokio::test]
async fn test_x_xss_protection() -> Result<(), Box<dyn std::error::Error>> {
    // Test XSS Protection ON
    let helmet_on = Helmet::builder().with_header(XXSSProtection::on()).build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet_on)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(response.headers().get("x-xss-protection").unwrap(), "1");

    // Test XSS Protection OFF
    let helmet_off = Helmet::builder().with_header(XXSSProtection::off()).build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet_off)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(response.headers().get("x-xss-protection").unwrap(), "0");

    Ok(())
}

#[tokio::test]
async fn test_strict_transport_security() -> Result<(), Box<dyn std::error::Error>> {
    let helmet = Helmet::builder()
        .with_header(StrictTransportSecurity::default())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    let hsts_header = response.headers().get("strict-transport-security");
    assert!(hsts_header.is_some());
    // HSTS header should contain max-age
    assert!(hsts_header.unwrap().to_str().unwrap().contains("max-age"));

    Ok(())
}

#[tokio::test]
async fn test_referrer_policy() -> Result<(), Box<dyn std::error::Error>> {
    let helmet = Helmet::builder()
        .with_header(ReferrerPolicy::no_referrer())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("referrer-policy").unwrap(),
        "no-referrer"
    );

    Ok(())
}

#[tokio::test]
async fn test_x_dns_prefetch_control() -> Result<(), Box<dyn std::error::Error>> {
    // Test DNS prefetch OFF
    let helmet_off = Helmet::builder()
        .with_header(XDNSPrefetchControl::off())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet_off)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("x-dns-prefetch-control").unwrap(),
        "off"
    );

    // Test DNS prefetch ON
    let helmet_on = Helmet::builder()
        .with_header(XDNSPrefetchControl::on())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet_on)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("x-dns-prefetch-control").unwrap(),
        "on"
    );

    Ok(())
}

#[tokio::test]
async fn test_x_download_options() -> Result<(), Box<dyn std::error::Error>> {
    let helmet = Helmet::builder()
        .with_header(XDownloadOptions::noopen())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("x-download-options").unwrap(),
        "noopen"
    );

    Ok(())
}

#[tokio::test]
async fn test_x_powered_by() -> Result<(), Box<dyn std::error::Error>> {
    // X-Powered-By with custom value
    let helmet = Helmet::builder()
        .with_header(XPoweredBy::new("Sword Framework"))
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("x-powered-by").unwrap(),
        "Sword Framework"
    );

    Ok(())
}

#[tokio::test]
async fn test_x_permitted_cross_domain_policies()
-> Result<(), Box<dyn std::error::Error>> {
    let helmet = Helmet::builder()
        .with_header(XPermittedCrossDomainPolicies::none())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response
            .headers()
            .get("x-permitted-cross-domain-policies")
            .unwrap(),
        "none"
    );

    Ok(())
}

#[tokio::test]
async fn test_cross_origin_embedder_policy() -> Result<(), Box<dyn std::error::Error>>
{
    let helmet = Helmet::builder()
        .with_header(CrossOriginEmbedderPolicy::require_corp())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response
            .headers()
            .get("cross-origin-embedder-policy")
            .unwrap(),
        "require-corp"
    );

    Ok(())
}

#[tokio::test]
async fn test_cross_origin_opener_policy() -> Result<(), Box<dyn std::error::Error>>
{
    let helmet = Helmet::builder()
        .with_header(CrossOriginOpenerPolicy::same_origin())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response
            .headers()
            .get("cross-origin-opener-policy")
            .unwrap(),
        "same-origin"
    );

    Ok(())
}

#[tokio::test]
async fn test_cross_origin_resource_policy() -> Result<(), Box<dyn std::error::Error>>
{
    let helmet = Helmet::builder()
        .with_header(CrossOriginResourcePolicy::cross_origin())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response
            .headers()
            .get("cross-origin-resource-policy")
            .unwrap(),
        "cross-origin"
    );

    Ok(())
}

#[tokio::test]
async fn test_origin_agent_cluster() -> Result<(), Box<dyn std::error::Error>> {
    let helmet = Helmet::builder()
        .with_header(OriginAgentCluster::new(true))
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    assert_eq!(
        response.headers().get("origin-agent-cluster").unwrap(),
        "?1"
    );

    Ok(())
}

#[tokio::test]
async fn test_content_security_policy() -> Result<(), Box<dyn std::error::Error>> {
    // Use default CSP policy
    let helmet = Helmet::builder()
        .with_header(ContentSecurityPolicy::default())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    let csp_header = response.headers().get("content-security-policy");
    assert!(csp_header.is_some());
    // Just verify that some CSP value is present
    assert!(!csp_header.unwrap().to_str().unwrap().is_empty());

    Ok(())
}

#[tokio::test]
async fn test_multiple_security_headers() -> Result<(), Box<dyn std::error::Error>> {
    // Test combining multiple security headers
    let helmet = Helmet::builder()
        .with_header(XContentTypeOptions::nosniff())
        .with_header(XFrameOptions::deny())
        .with_header(XXSSProtection::on())
        .with_header(ReferrerPolicy::no_referrer())
        .with_header(XDNSPrefetchControl::off())
        .with_header(XDownloadOptions::noopen())
        .with_header(XPermittedCrossDomainPolicies::none())
        .with_header(CrossOriginEmbedderPolicy::require_corp())
        .with_header(CrossOriginOpenerPolicy::same_origin())
        .with_header(CrossOriginResourcePolicy::same_site())
        .with_header(OriginAgentCluster::new(true))
        .with_header(StrictTransportSecurity::default())
        .build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    let headers = response.headers();

    // Verify all headers are present
    assert_eq!(headers.get("x-content-type-options").unwrap(), "nosniff");
    assert_eq!(headers.get("x-frame-options").unwrap(), "DENY");
    assert_eq!(headers.get("x-xss-protection").unwrap(), "1");
    assert_eq!(headers.get("referrer-policy").unwrap(), "no-referrer");
    assert_eq!(headers.get("x-dns-prefetch-control").unwrap(), "off");
    assert_eq!(headers.get("x-download-options").unwrap(), "noopen");
    assert_eq!(
        headers.get("x-permitted-cross-domain-policies").unwrap(),
        "none"
    );
    assert_eq!(
        headers.get("cross-origin-embedder-policy").unwrap(),
        "require-corp"
    );
    assert_eq!(
        headers.get("cross-origin-opener-policy").unwrap(),
        "same-origin"
    );
    assert_eq!(
        headers.get("cross-origin-resource-policy").unwrap(),
        "same-site"
    );
    assert_eq!(headers.get("origin-agent-cluster").unwrap(), "?1");
    assert!(headers.get("strict-transport-security").is_some());

    Ok(())
}

#[tokio::test]
async fn test_empty_helmet() -> Result<(), Box<dyn std::error::Error>> {
    // Test helmet with no headers configured
    let helmet = Helmet::builder().build();

    let app = Application::builder()?
        .with_controller::<HelmetTestController>()
        .with_layer(helmet)
        .build();

    let server = TestServer::new(app.router())?;
    let response = server.get("/test").await;

    assert_eq!(response.status_code(), 200);
    // Should still work with no security headers - check JSON response
    assert!(response.text().contains("Hello from helmet test"));

    Ok(())
}

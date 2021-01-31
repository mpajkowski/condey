use anyhow::Result;
use condey::Condey;
use condey::Handler;
use http::method::Method;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().try_init().unwrap();

    let routes = vec![
        Handler::new(Method::GET, "/employees", "root callback"),
        Handler::new(Method::GET, "/employees/:id", "id callback"),
        Handler::new(Method::GET, "/employees/:id/assignments", "id callback"),
        Handler::new(
            Method::GET,
            "/employees/:id/assignments/:assignment_id",
            "id callback",
        ),
    ];

    Condey::init().mount("/api/test", routes).serve().await?;

    Ok(())
}

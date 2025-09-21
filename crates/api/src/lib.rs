
use axum::{routing::get, Router};
use tracing::info;

use voip_common::Result;

pub fn router() -> Router {
    Router::new().route("/health", get(health)).route("/info", get(info))
}

async fn health() -> &'static str {
    "OK"
}

async fn info() -> &'static str {
    "voip-api"
}

pub async fn serve(addr: &str) -> Result<()> {
    info!(%addr, "starting HTTP API");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let app = router();
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}

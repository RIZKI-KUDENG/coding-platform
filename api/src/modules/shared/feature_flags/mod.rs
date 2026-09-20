use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use crate::state::AppState;
pub mod require_feature;
pub use require_feature::check_feature;


pub async fn require_feature(
    state: AppState,
    key: &'static str,
    display_name: Option<&'static str>,
    req: Request,
    next: Next,
) -> Result<Response, Response> {
    check_feature(&state, key, display_name).await?;

    Ok(next.run(req).await)
}

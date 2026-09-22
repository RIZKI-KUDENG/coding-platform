use super::run_code;
use super::submission;
use crate::modules::shared::feature_flags::check_feature;
use crate::state::AppState;
use axum::{
    Router,
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
    routing::post,
};

async fn require_submission_feature(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, Response> {
    check_feature(
        &state,
        "courses:submission",
        Some("Pengiriman Solusi Latihan"),
    )
    .await?;
    Ok(next.run(req).await)
}

async fn require_playground_feature(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, Response> {
    check_feature(&state, "playground", Some("Area Uji Coba (Playground)")).await?;
    Ok(next.run(req).await)
}

pub fn execution_routes(state: AppState) -> Router<AppState> {
    let submission_route = Router::new()
        .nest(
            "/api/v1/exercises/{exercise_id}",
            Router::new().route("/submissions", post(submission::submit)),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_submission_feature,
        ));

    let playground_route = Router::new()
        .route("/api/v1/playground/run", post(run_code::run))
        .route_layer(middleware::from_fn_with_state(
            state,
            require_playground_feature,
        ));

    submission_route.merge(playground_route)
}

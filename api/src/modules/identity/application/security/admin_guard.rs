use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response}
};
use serde_json::json;
use uuid::Uuid;


use crate::{
    modules::identity::{
        ValidateSessionQuery,ValidateSessionQueryHandler,
        infrastructure::repositories::{
            session_repository::SessionRepository,
            user_repository::UserRepository,
        },
    },
    state::AppState
};

#[derive(Clone, Debug)]
pub struct AdminUser{
    pub user_id: Uuid,
}

pub async fn require_admin(
    State(state): State<AppState>,
    mut req: Request,
    next: Next
) -> Result<Response, Response> {
    let token = match req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
        {
            Some(token) if !token.is_empty() => token,
            _ => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": {
                            "code": "AUTH_UNAUTHORIZED",
                            "message": "missing or invalid Authorization header"
                        }
                    })),
                )
                    .into_response());
            }
        };

       let session_repo = SessionRepository::new(state.db.clone());
       let handler  = ValidateSessionQueryHandler::new(session_repo);
       let user_id = match handler.handle(ValidateSessionQuery { raw_token: token.to_string() }).await {
           Ok(id) => id,
           Err(_) => {
               return Err((
                   StatusCode::UNAUTHORIZED,
                   Json(json!({
                       "error": {
                           "code": "AUTH_UNAUTHORIZED",
                           "message": "Invalid or expired token"
                       }
                   }))
               )
            .into_response());
           }
       };

       let user_repo = UserRepository::new(state.db.clone());
       let user  = match user_repo.find_by_id(user_id).await{
           Ok(Some(u)) => u,
           _=> {
               return Err((
                   StatusCode::UNAUTHORIZED,
                   Json(json!({
                       "error": {
                           "code": "USER_NOT_FOUND",
                           "message": "User not found"
                       }
                   }))
                   ).into_response()
               );
           }
       };
       if user.role.as_deref() != Some("admin") {
           return Err((
               StatusCode::UNAUTHORIZED,
               Json(json!({
                   "error": {
                       "code": "FORBIDDEN_RESOURCE",
                       "message": "Access hanya untuk admin"
                   }
               })),
               ).into_response());
       }

       req.extensions_mut().
           insert(AdminUser { user_id });

        Ok(next.run(req).await)
}

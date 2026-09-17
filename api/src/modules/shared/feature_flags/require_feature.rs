use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{
    modules::system::{
        IsFeatureEnabledQuery, IsFeatureEnabledQueryHandler,
        infrastructure::repositories::feature_flag_repository::FeatureFlagRepository,
    },
    state::AppState,
};

/// Memeriksa status fitur langsung dan murni dari database tanpa default value.
///
/// Hasil evaluasi murni berdasarkan database:
/// 1. Baris ada & `is_enabled == 1` => Lolos (`Ok(())`).
/// 2. Baris ada & `is_enabled == 0` => HTTP 503 `FEATURE_MAINTENANCE`.
/// 3. Baris TIDAK ADA di DB (`None`) => HTTP 503 `FEATURE_NOT_FOUND`.
/// 4. Koneksi Database Error => HTTP 500 `INTERNAL_ERROR` (tidak disembunyikan).
pub async fn check_feature(
    state: &AppState,
    key: &str,
    display_name: Option<&str>,
) -> Result<(), Response> {
    let repo = FeatureFlagRepository::new(state.db.clone());
    let handler = IsFeatureEnabledQueryHandler::new(repo);

    let name = display_name.unwrap_or(key);

    match handler.handle(IsFeatureEnabledQuery::new(key)).await {
        Ok(Some(true)) => Ok(()),
        Ok(Some(false)) => Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": {
                    "code": "FEATURE_MAINTENANCE",
                    "message": format!("Fitur '{}' sedang dalam pemeliharaan berkala.", name),
                    "feature": key
                }
            })),
        )
            .into_response()),
        Ok(None) => Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "error": {
                    "code": "FEATURE_NOT_FOUND",
                    "message": format!("Fitur '{}' belum terdaftar atau belum diaktifkan pada sistem.", name),
                    "feature": key
                }
            })),
        )
            .into_response()),
        Err(err) => {
            eprintln!("Database error saat memeriksa status feature flag '{}': {:?}", key, err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": {
                        "code": "INTERNAL_ERROR",
                        "message": "Gagal membaca status fitur dari database."
                    }
                })),
            )
                .into_response())
        }
    }
}

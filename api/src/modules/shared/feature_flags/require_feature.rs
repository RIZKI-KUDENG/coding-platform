use axum::response::{IntoResponse, Response};

use crate::{
    modules::{
        shared::http::ApiError,
        system::{IsFeatureEnabledQuery, IsFeatureEnabledQueryHandler},
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
    let handler = IsFeatureEnabledQueryHandler::from_pool(state.db.clone());
    let name = display_name.unwrap_or(key);

    match handler.handle(IsFeatureEnabledQuery::new(key)).await {
        Ok(Some(true)) => Ok(()),
        Ok(Some(false)) => Err(ApiError::service_unavailable(
            "FEATURE_MAINTENANCE",
            format!("Fitur '{}' sedang dalam pemeliharaan berkala.", name),
        )
        .with_feature(key)
        .into_response()),
        Ok(None) => Err(ApiError::service_unavailable(
            "FEATURE_NOT_FOUND",
            format!(
                "Fitur '{}' belum terdaftar atau belum diaktifkan pada sistem.",
                name
            ),
        )
        .with_feature(key)
        .into_response()),
        Err(err) => {
            eprintln!(
                "Error saat memeriksa status feature flag '{}': {:?}",
                key, err
            );
            Err(ApiError::new(
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                "Gagal membaca status fitur dari database.",
            )
            .into_response())
        }
    }
}

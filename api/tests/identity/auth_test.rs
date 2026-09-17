use api::config::Config;
use api::modules::identity::application::security::password_hasher::{
    hash_password, verify_password,
};
use api::modules::identity::application::security::session_token::generate_session_token;
use api::modules::identity::application::use_cases::auth::commands::login::login_user_command::{
    LoginCommand, LoginCommandHandler, LoginError,
};
use api::modules::identity::application::use_cases::auth::commands::register::register_user_command::{
    RegisterCommand, RegisterCommandHandler, RegisterError,
};
use api::modules::identity::infrastructure::repositories::session_repository::SessionRepository;
use api::modules::identity::infrastructure::repositories::user_repository::UserRepository;
use api::modules::identity::{
    ValidateSessionError, ValidateSessionQuery, ValidateSessionQueryHandler,
};
use api::state::AppState;
use chrono::{Duration, Utc};
use uuid::Uuid;

#[test]
fn test_password_hashing_and_verification() {
    let password = "SuperSecretPassword123!";
    let hashed = hash_password(password).expect("Hashing should succeed");

    assert_ne!(password, hashed);

    // Verify with correct password
    let is_valid = verify_password(password, &hashed).expect("Verification should succeed");
    assert!(is_valid);

    // Verify with wrong password
    let is_invalid =
        verify_password("WrongPassword", &hashed).expect("Verification should succeed");
    assert!(!is_invalid);
}

#[test]
fn test_session_token_generation() {
    let token1 = generate_session_token();
    let token2 = generate_session_token();

    assert_eq!(token1.raw.len(), 64);
    assert_eq!(token1.hash.len(), 64);
    assert_ne!(token1.raw, token2.raw);
    assert_ne!(token1.hash, token2.hash);
}

#[tokio::test]
async fn test_register_and_login_flow() {
    let config = Config::load();
    let state = AppState::new(&config).await;

    let user_repo = UserRepository::new(state.db.clone());
    let session_repo = SessionRepository::new(state.db.clone());

    let register_handler = RegisterCommandHandler::new(user_repo.clone(), session_repo.clone());
    let login_handler = LoginCommandHandler::new(user_repo.clone(), session_repo.clone());

    let unique_id = Uuid::new_v4().to_string()[..8].to_string();
    let email = format!("test_{}@example.com", unique_id);
    let username = format!("user_{}", unique_id);
    let password = "TestPassword123!";

    // 1. Test Register Success
    let register_cmd = RegisterCommand {
        username: username.clone(),
        email: email.clone(),
        password: password.to_string(),
    };

    let register_result = register_handler
        .handle(register_cmd)
        .await
        .expect("Registration should succeed");

    assert_eq!(register_result.user.email, email);
    assert_eq!(register_result.user.username, username);
    assert!(!register_result.access_token.is_empty());

    // 2. Test Duplicate Register (Email already exists)
    let duplicate_cmd = RegisterCommand {
        username: format!("other_{}", unique_id),
        email: email.clone(),
        password: password.to_string(),
    };

    let duplicate_result = register_handler.handle(duplicate_cmd).await;
    match duplicate_result {
        Err(RegisterError::EmailAlreadyExists) => {}
        other => panic!("Expected EmailAlreadyExists, got {:?}", other),
    }

    // 3. Test Login with email
    let login_with_email_cmd = LoginCommand {
        identifier: email.clone(),
        password: password.to_string(),
    };

    let login_result = login_handler
        .handle(login_with_email_cmd)
        .await
        .expect("Login with email should succeed");

    assert_eq!(login_result.user.id, register_result.user.id);
    assert!(!login_result.access_token.is_empty());

    // 4. Test Login with username
    let login_with_username_cmd = LoginCommand {
        identifier: username.clone(),
        password: password.to_string(),
    };

    let login_user_result = login_handler
        .handle(login_with_username_cmd)
        .await
        .expect("Login with username should succeed");

    assert_eq!(login_user_result.user.id, register_result.user.id);

    // 5. Test Login with wrong password
    let wrong_pass_cmd = LoginCommand {
        identifier: username.clone(),
        password: "WrongPassword999!".to_string(),
    };

    let wrong_pass_result = login_handler.handle(wrong_pass_cmd).await;
    match wrong_pass_result {
        Err(LoginError::InvalidCredentials) => {}
        other => panic!("Expected InvalidCredentials, got {:?}", other),
    }

    // 6. Test Login with non-existent user
    let unknown_user_cmd = LoginCommand {
        identifier: "non_existent_user_xyz".to_string(),
        password: password.to_string(),
    };

    let unknown_result = login_handler.handle(unknown_user_cmd).await;
    match unknown_result {
        Err(LoginError::InvalidCredentials) => {}
        other => panic!("Expected InvalidCredentials, got {:?}", other),
    }
}

#[tokio::test]
async fn test_validate_session_query() {
    let config = Config::load();
    let state = AppState::new(&config).await;

    let user_repo = UserRepository::new(state.db.clone());
    let session_repo = SessionRepository::new(state.db.clone());

    let register_handler = RegisterCommandHandler::new(user_repo, session_repo.clone());
    let validate_handler = ValidateSessionQueryHandler::new(session_repo.clone());

    let unique_id = Uuid::new_v4().to_string()[..8].to_string();
    let register_cmd = RegisterCommand {
        username: format!("val_user_{}", unique_id),
        email: format!("val_{}@example.com", unique_id),
        password: "TestPassword123!".to_string(),
    };

    let register_result = register_handler
        .handle(register_cmd)
        .await
        .expect("Registration should succeed");

    // 1. Valid token should return user_id
    let valid_query = ValidateSessionQuery {
        raw_token: register_result.access_token.clone(),
    };
    let user_id = validate_handler
        .handle(valid_query)
        .await
        .expect("Session validation should succeed");
    assert_eq!(user_id, register_result.user.id);

    // 2. Non-existent / invalid token should fail
    let invalid_query = ValidateSessionQuery {
        raw_token: "invalid_random_token_999".to_string(),
    };
    let invalid_err = validate_handler.handle(invalid_query).await;
    match invalid_err {
        Err(ValidateSessionError::InvalidOrExpiredToken) => {}
        other => panic!("Expected InvalidOrExpiredToken, got {:?}", other),
    }

    // 3. Expired token should fail
    let expired_token = generate_session_token();
    let expired_at = Utc::now() - Duration::hours(1);
    session_repo
        .create(register_result.user.id, expired_token.hash, expired_at)
        .await
        .expect("Inserting expired session should succeed");

    let expired_query = ValidateSessionQuery {
        raw_token: expired_token.raw,
    };
    let expired_err = validate_handler.handle(expired_query).await;
    match expired_err {
        Err(ValidateSessionError::InvalidOrExpiredToken) => {}
        other => panic!("Expected InvalidOrExpiredToken, got {:?}", other),
    }
}

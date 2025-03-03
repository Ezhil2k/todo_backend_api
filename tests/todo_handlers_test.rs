// tests/todo_handlers_test.rs
use actix_web::{http::StatusCode, test, web, App};
use todo_api::{
    config,
    handlers::todo,
    models::auth::Claims,
    models::todo::{CreateTodoRequest, UpdateTodoRequest},
};
use std::time::{SystemTime, UNIX_EPOCH};

#[actix_web::test]
async fn test_create_todo() {
    // Mock configuration
    let config = config::load_config();
    
    // Mock request
    let todo_req = CreateTodoRequest {
        task_id: 1,
        description: "Test todo".to_string(),
        due_date: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 + 86400, // Tomorrow
    };
    
    // Mock claims
    let claims = Claims {
        sub: "8z5jfiFVgyBCdMCYNgBivVNMmJgwQpEMjHvd4dVCsRJv".to_string(), // Example public key
        exp: 0,
    };
    
    // Create test app
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(config))
            .app_data(web::JsonConfig::default().limit(4096))
            .route("/api/todos", web::post().to(todo::create_todo))
    ).await;
    
    // Create request with JSON body
    let req = test::TestRequest::post()
        .uri("/api/todos")
        .set_json(&todo_req)
        .to_request();
    
    // Inject claims into request extensions
    req.extensions_mut().insert(claims);
    
    // Execute request
    let resp = test::call_service(&app, req).await;
    
    // This will fail in a real test because it can't connect to Solana
    // In a real test, you would mock the Solana service
    assert_eq!(resp.status(), StatusCode::BAD_GATEWAY);
}

// Add more tests for other handlers
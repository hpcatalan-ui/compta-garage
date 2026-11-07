//! Garage Compta API - Main entry point

use garage_compta_api::routes;
use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize logging
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Create application state with default configuration
    let state = match garage_compta_api::state::AppState::new_with_defaults("./data/garage_compta.db").await {
        Ok(state) => state,
        Err(e) => {
            eprintln!("Failed to initialize application state: {}", e);
            std::process::exit(1);
        }
    };

    // Run database migrations
    if let Err(e) = garage_compta_data::migrations::run_migrations(&state.db.pool).await {
        eprintln!("Failed to run database migrations: {}", e);
        std::process::exit(1);
    }

    // Create the app with state
    let app = routes::create_app(state.clone()).await;

    // Determine the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Start the server
    println!("🚀 Garage Compta API server started!");
    println!("   Listening on: http://{}", addr);
    println!("   Environment: development");
    println!("   Database: ./data/garage_compta.db");
    println!("");
    println!("📋 Available endpoints:");
    println!("   GET  /health           - Health check");
    println!("   GET  /ready            - Readiness check");
    println!("   POST /api/auth/login   - Login (JWT)");
    println!("   GET  /api/clients      - List clients");
    println!("   POST /api/clients      - Create client");
    println!("   GET  /api/clients/:id  - Get client by ID");
    println!("   PUT  /api/clients/:id  - Update client");
    println!("   DELETE /api/clients/:id- Delete client");
    println!("   GET  /api/vehicles      - List vehicles");
    println!("   POST /api/vehicles      - Create vehicle");
    println!("   GET  /api/invoices      - List invoices");
    println!("   POST /api/invoices      - Create invoice");
    println!("   GET  /api/parts         - List parts");
    println!("   POST /api/parts         - Create part");
    println!("   GET  /api/suppliers     - List suppliers");
    println!("   POST /api/suppliers     - Create supplier");
    println!("");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    
    // In Axum 0.7, Router<AppState> needs to have its state filled
    // by calling .with_state() to get Router<()>
    let app_with_state = app.with_state(state);
    
    // Now Router<()> can be used directly with axum::serve
    axum::serve(listener, app_with_state)
        .await
        .unwrap();
}

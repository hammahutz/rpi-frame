use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    async fn index() -> &'static str {
        "Homes"
    }
    async fn about() -> &'static str {
        "About"
    }

    async fn list_users() -> &'static str {
        "List users"
    }

    async fn create_users() -> &'static str {
        "Create users"
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/users", get(list_users).post(create_users))
        .route("/about", get(about));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    print!("Listening to requests...");
    axum::serve(listener, app).await.unwrap();
}

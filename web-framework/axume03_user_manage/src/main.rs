use axum::{
    response::IntoResponse,
    routing::post,
    Json, Router,
};

//#derive(thiserror::Error, Debug)
pub enum Error {
    NotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        ("NotFound".to_string()).into_response()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    user_id: i64,
    username: String,
    password: String,
    sex: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct NewUser {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/users", post(create_user));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_user(Json(req): Json<NewUser>) -> Result<Json<NewUser>, Error> {
    Ok(Json(NewUser {
        username: "hello".to_string(),
        password: "world".to_string(),
    }))
}

use axum::{extract::Multipart, http::Method, response::Result, routing::{get, post}, Json, Router};
use tokio::{fs, net::TcpListener};
use tokio_stream::wrappers::ReadDirStream;
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};
use uuid::Uuid;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let make_service = Router::new()
        .route("/list", get(list))
        .route("/upload", post(upload))
        .layer(CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any))
        .fallback_service(ServeDir::new("dist"))
        .fallback_service(ServeDir::new("uploads"));
    axum::serve(tcp_listener, make_service).await.unwrap();
}

async fn upload(mut multipart: Multipart) -> Result<Json<serde_json::Value>>  {
    let mut file_names = Vec::new();
    while let Ok(Some( field)) = multipart.next_field().await {
        let extension = field.file_name().unwrap().split(".").last().ok_or("")?;
        let uuid = Uuid::new_v4().to_string();
        let file_name = format!("{}.{}", uuid, extension);

        let name = field.name().ok_or("")?.to_string();
        let data = field.bytes().await.map_err(|_| "")?;

        println!("Length of `{}` is {} bytes", name, data.len());

        fs::create_dir_all("uploads").await.map_err(|_| "")?;
        let file_path = format!("uploads/{}", file_name);
        fs::write(file_path, data).await.map_err(|_| "")?;
        file_names.push(file_name)
    }
    Ok(Json(serde_json::json!(file_names)))
}

async fn list() ->Result<Json<serde_json::Value>>{
    fs::create_dir_all("uploads").await.map_err(|_| "1")?;
    let mut file_names: Vec<String> = Vec::new();
    let read_dir = fs::read_dir("uploads").await.map_err(|_| "2")?;
    let mut read_dir_stream = ReadDirStream::new(read_dir);
    while let Some(Ok(entry)) = read_dir_stream.next().await {
        let file_name = entry.file_name().into_string().map_err(|_| "4")?;
        file_names.push(file_name)
    }
    Ok(Json(serde_json::json!(file_names)))
}
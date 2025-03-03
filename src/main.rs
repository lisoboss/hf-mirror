mod agent;
mod error;
mod etag;
mod git;

use axum::{routing::get, Router};
use std::cell::LazyCell;
use std::env;
use tokio::net::TcpListener;

// Local Hugging Face image path
pub(crate) const HF_MIRROR_PATH: LazyCell<String> =
    LazyCell::new(|| env::var("HF_MIRROR_PATH").unwrap_or_else(|_| "/hf_mirror".to_string()));
// Listening address
const HF_MIRROR_ADDR: LazyCell<String> = LazyCell::new(|| {
    let host = env::var("HF_MIRROR_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("HF_MIRROR_PORT").unwrap_or_else(|_| "8080".to_string());
    format!("{}:{}", host, port)
});

#[tokio::main]
async fn main() {
    let app = Router::<()>::new()
        // https://huggingface.co/api/models/meta-llama/Llama-3.3-70B-Instruct/tree/main?recursive=True&expand=False
        .route(
            "/api/{resource_name}/{user_id}/{repo_id}/tree/{revision}",
            get(agent::hf_list_repo_files),
        )
        // https://huggingface.co/api/models/casperhansen/deepseek-r1-distill-llama-8b-awq/revision/main
        .route(
            "/api/{resource_name}/{user_id}/{repo_id}/revision/{revision}",
            get(agent::hf_repo_info),
        )
        // https://huggingface.co/api/models/casperhansen/deepseek-r1-distill-llama-8b-awq/revision/main
        .route(
            "/api/{resource_name}/{user_id}/{repo_id}",
            get(agent::hf_repo_info),
        )
        // https://huggingface.co/datasets/deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B/resolve/main/generation_config.json
        .route(
            "/{resource_name}/{user_id}/{repo_id}/resolve/{revision}/{*file_path}",
            get(agent::hf_hub_download),
        )
        // https://huggingface.co/deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B/resolve/main/generation_config.json
        .route(
            "/{user_id}/{repo_id}/resolve/{revision}/{*file_path}",
            get(agent::hf_hub_download),
        );

    let listener = TcpListener::bind(&*HF_MIRROR_ADDR).await.unwrap();
    println!("listener: http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

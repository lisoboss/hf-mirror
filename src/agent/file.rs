use crate::{error::AppError, etag::etag_sha256, git::commit_head_id, HF_MIRROR_PATH};
use axum::{
    extract::Path,
    http::{
        header::{self, HeaderValue},
        HeaderName,
    },
    response::{IntoResponse, Response},
};
use axum_extra::{headers::Range, TypedHeader};
use axum_range::{KnownSize, Ranged};
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs::File;

const HEADER_X_REPO_COMMIT: HeaderName = HeaderName::from_static("x-repo-commit");

#[derive(Deserialize)]
pub(crate) struct HFFile {
    resource_name: Option<String>,
    user_id: String,
    repo_id: String,
    file_path: String,
}

// https://huggingface.co/deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B/resolve/main/generation_config.json
pub(crate) async fn hf_hub_download(
    Path(hf_file): Path<HFFile>,
    range: Option<TypedHeader<Range>>,
) -> Result<Response, AppError> {
    let repo_path = PathBuf::from(&*HF_MIRROR_PATH)
        .join(match &hf_file.resource_name {
            Some(resource_name) => resource_name,
            None => "models",
        })
        .join(&hf_file.user_id)
        .join(&hf_file.repo_id);

    let commit_id = commit_head_id(&repo_path)?;
    let commit_header = HeaderValue::from_str(&commit_id)?;

    let file_path = repo_path.join(&hf_file.file_path);

    let etag = etag_sha256(&file_path).unwrap_or_default();
    let etag_header = HeaderValue::from_str(&etag)?;

    let file = File::open(file_path).await?;
    let body = KnownSize::file(file).await?;
    let range = range.map(|TypedHeader(range)| range);

    let mut response = Ranged::new(range, body).into_response();
    let headers = response.headers_mut();
    let _ = headers.insert(HEADER_X_REPO_COMMIT, commit_header);
    let _ = headers.insert(header::ETAG, etag_header);

    Ok(response)
}

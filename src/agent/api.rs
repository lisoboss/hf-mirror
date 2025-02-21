use crate::{
    error::AppError,
    git::{commit_head_id, file_tree},
    HF_MIRROR_PATH,
};
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize)]
pub(crate) struct HFApi {
    resource_name: String,
    user_id: String,
    repo_id: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct RepoFile {
    #[serde(rename = "type")]
    file_type: String,
    oid: String,
    size: usize,
    path: String,
}

impl Default for RepoFile {
    fn default() -> Self {
        Self {
            file_type: "file".to_string(), // 设定默认值
            oid: String::new(),
            size: 0,
            path: String::new(),
        }
    }
}

// https://huggingface.co/api/models/casperhansen/deepseek-r1-distill-llama-8b-awq/tree/main?recursive=True&expand=False
pub(crate) async fn hf_list_repo_files(
    Path(hf_api): Path<HFApi>,
) -> Result<Json<Vec<RepoFile>>, AppError> {
    let repo_path = PathBuf::from(&*HF_MIRROR_PATH)
        .join(&hf_api.resource_name)
        .join(&hf_api.user_id)
        .join(&hf_api.repo_id);

    let mut repo_files: Vec<RepoFile> = Vec::new();
    file_tree(&repo_path, |path, oid, size| {
        repo_files.push(RepoFile {
            oid,
            size,
            path,
            ..Default::default()
        });
    })?;

    Ok(Json(repo_files))
}

#[derive(Debug, Serialize)]
pub(crate) struct Sibling {
    rfilename: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct RepoInfo {
    id: String,
    sha: String,
    siblings: Vec<Sibling>,
}

pub(crate) async fn hf_repo_info(Path(hf_api): Path<HFApi>) -> Result<Json<RepoInfo>, AppError> {
    let repo_path = PathBuf::from(&*HF_MIRROR_PATH)
        .join(&hf_api.resource_name)
        .join(&hf_api.user_id)
        .join(&hf_api.repo_id);

    let sha = commit_head_id(&repo_path)?;
    let mut siblings: Vec<Sibling> = Vec::new();
    file_tree(&repo_path, |path, _, _| {
        siblings.push(Sibling { rfilename: path });
    })?;

    Ok(Json(RepoInfo {
        id: format!("{}/{}", hf_api.user_id, hf_api.repo_id),
        sha,
        siblings,
    }))
}

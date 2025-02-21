use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

pub(crate) fn etag_sha256(path: &PathBuf) -> Option<String> {
    let metadata = path.metadata().ok()?;
    let modified = metadata
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();
    let size = metadata.len();

    let mut file = File::open(path).ok()?;
    let mut hasher = Sha1::new();
    let mut buffer = [0; 4096];

    // 计算 size 和 modified
    hasher.update(size.to_be_bytes());
    hasher.update(modified.to_be_bytes());

    // 读取前 4KB
    if file.read_exact(&mut buffer).is_ok() {
        hasher.update(&buffer);
    }

    // 读取后 4KB（仅当文件大于 8KB 时）
    if size > 8192 {
        file.seek(SeekFrom::End(-4096)).ok()?;
        if file.read_exact(&mut buffer).is_ok() {
            hasher.update(&buffer);
        }
    }

    // 计算最终 SHA-256 哈希
    let hash = hasher.finalize();

    // 取前 16 字符（减少长度）
    Some(format!("\"{:.x}\"", hash))
}

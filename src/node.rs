use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Node {
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_binary_or_image: bool,
    pub line_count: usize,
    pub byte_count: u64,
}

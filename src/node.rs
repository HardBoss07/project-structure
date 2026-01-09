use std::path::PathBuf;

#[derive(Debug)]
pub struct Node {
    pub path: PathBuf,
    pub is_dir: bool,
}

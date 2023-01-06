use std::{fs};
use std::path::Path;
use std::str::from_utf8;

use sha2::{Digest, Sha256};

use crate::utils::{git::{get_git_files, is_within_git_repo}, reference::Reference};

use crate::resolve::resolver_trait::Resolved;

pub struct ResolvedFilePath {
    pub path: String,
}

impl Resolved for ResolvedFilePath {
    fn create_reference(&self) -> Option<Reference> {
        let text_bytes = match fs::read(self.path.clone()) {
            Ok(bytes) => bytes,
            Err(_e) => {
                log::debug!("Could not read file: {}", self.path);
                return None
            }
        };
        match from_utf8(&text_bytes) {
            Ok(text) => {
                let mut hasher = Sha256::new();
                hasher.update(&text_bytes);
                let file_hash = format!("{:x}", hasher.finalize());
                let reference = Reference::new("file".to_string(), file_hash, text.to_string(), text_bytes.len(), self.path.clone());
                Some(reference)
            },
            Err(_e) => {
                log::debug!("Could not convert bytes to utf8: {}", self.path);
                None
            }
        }
    }

    fn r#type(&self) -> String {
        "file".to_string()  
    }

    fn size_bytes(&self) -> Option<u64> {
        match fs::metadata(self.path.clone()) {
            Ok(meta_data) => return Some(meta_data.len()),
            Err(_e) => {
                log::debug!("Could not read file: {}", self.path);
                return None
            }
        };
    }

    fn text_hash(&self) -> Option<String> {
        let text_bytes = match fs::read(self.path.clone()) {
            Ok(bytes) => bytes,
            Err(_e) => {
                log::debug!("Could not read file: {}", self.path);
                return None
            }
        };
        let mut hasher = Sha256::new();
        hasher.update(text_bytes);
        let file_hash = format!("{:x}", hasher.finalize());
        Some(file_hash)
    }
}

pub(crate) struct PathResolver {}

impl PathResolver {
    pub fn extract_files(&self, path: &Path) -> Vec<String> {
        // println!("Extracting files from: {}", path.to_string_lossy());
        if path.is_dir() {
            if is_within_git_repo(path) {
                get_git_files(path)
            } 
            else {
                let mut file_paths: Vec<String> = Vec::new();
                for entry in fs::read_dir(path).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_dir() {
                        file_paths.extend(self.extract_files(&path));
                    } else {
                        file_paths.push(path.to_string_lossy().to_string());
                    }
                }
                file_paths
            }
        } else {
            vec![path.to_string_lossy().to_string()] 
        }
    }

    pub fn should_resolve(&self, path_string: &String) -> bool {
        let path = Path::new(path_string);
        path.is_dir() || path.is_file()
    }

    pub async fn resolve(&self, path: &str) -> Vec<ResolvedFilePath> {
        let file_paths = self.extract_files(Path::new(path));
        // Create a ResolvedPath for each file path and return it as a vec
        let resolved_paths: Vec<ResolvedFilePath> = file_paths.iter().map(|file_path| ResolvedFilePath { path: file_path.to_string() }).collect();
        resolved_paths
    }
}

use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BatchPair {
    pub folder_name: String,
    pub path_a: PathBuf,
    pub path_b: PathBuf,
    pub status: String, // "Found", "Analyzed", "Error"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchScanResult {
    pub total_folders_scanned: usize,
    pub pairs: Vec<BatchPair>,
    pub incomplete_folders: Vec<String>,
}

pub struct BatchProcessor;

impl BatchProcessor {
    pub fn scan_folder(root: &Path, max_depth: usize) -> BatchScanResult {
        let mut pairs = Vec::new();
        let mut incomplete = Vec::new();
        let mut folder_map: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();

        // 1. Recursive Scan
        for entry in WalkDir::new(root).max_depth(max_depth) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            
            if entry.file_type().is_file() {
                let path = entry.path().to_path_buf();
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().eq_ignore_ascii_case("xlsx") {
                        if let Some(parent) = path.parent() {
                            folder_map.entry(parent.to_path_buf())
                                .or_default()
                                .push(path);
                        }
                    }
                }
            }
        }

        // 2. Identification & Pairing
        for (folder, files) in &folder_map {
            let mut file_a: Option<PathBuf> = None;
            let mut file_b: Option<PathBuf> = None;
            let mut count_a = 0;
            let mut count_b = 0;

            for file in files {
                let filename = file.file_name().unwrap().to_string_lossy().to_string();
                let lower_name = filename.to_lowercase();

                // Heuristic for File A (Transaction)
                // Avoid plain "a" matching "Data", use specific separators or keywords
                if lower_name.contains("_a") || lower_name.starts_with("a") || 
                   lower_name.contains("transaction") || 
                   lower_name.contains("account") || 
                   lower_name.contains("帳號") {
                    file_a = Some(file.clone());
                    count_a += 1;
                }
                
                // Heuristic for File B (IP Log)
                if lower_name.contains("_b") || lower_name.starts_with("b") || 
                   lower_name.contains("ip") || 
                   lower_name.contains("log") || 
                   lower_name.contains("登入") {
                    file_b = Some(file.clone());
                    count_b += 1;
                }
            }

            // 3. Strict Rule: Exactly 1 A and 1 B
            if count_a == 1 && count_b == 1 {
                if let (Some(a), Some(b)) = (file_a, file_b) {
                    pairs.push(BatchPair {
                        folder_name: folder.file_name().unwrap_or_default().to_string_lossy().to_string(),
                        path_a: a,
                        path_b: b,
                        status: "Found".to_string(),
                    });
                }
            } else {
                if count_a > 0 || count_b > 0 {
                    incomplete.push(folder.to_string_lossy().to_string());
                }
            }
        }
        
        pairs.sort_by(|a, b| a.folder_name.cmp(&b.folder_name));

        BatchScanResult {
            total_folders_scanned: folder_map.len(),
            pairs,
            incomplete_folders: incomplete,
        }
    }
}

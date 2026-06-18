use git2::Repository;
use std::fs;
#[cfg(unix)]
use std::os::unix::prelude::MetadataExt;
use std::path::Path;

#[cfg(unix)]
const BLOCK_SIZE: u64 = 512;

pub fn get_repo_size(repository: &Repository) -> String {
    let path = repository.path();

    if let Ok(metadata) = fs::metadata(path)
        && !metadata.is_dir()
    {
        let file_size = {
            #[cfg(unix)]
            {
                // On Unix, use actual allocated blocks (counts sparse files accurately)
                (metadata.blocks() * BLOCK_SIZE) as usize
            }
            #[cfg(not(unix))]
            {
                // On Windows/other platforms, fallback to standard file length
                metadata.len() as usize
            }
        };

        let common_path = repository.commondir();
        let total_size = file_size + walk_dir_disk_size(common_path);
        return humanize_size(total_size);
    }

    humanize_size(walk_dir_disk_size(path))
}

fn walk_dir_disk_size(path: &Path) -> usize {
    let mut total_size = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    total_size += walk_dir_disk_size(&entry.path());
                } else {
                    total_size += {
                        #[cfg(unix)]
                        {
                            // On Unix, use actual allocated blocks (counts sparse files accurately)
                            (metadata.blocks() * BLOCK_SIZE) as usize
                        }
                        #[cfg(not(unix))]
                        {
                            // On Windows/other platforms, fallback to standard file length
                            metadata.len() as usize
                        }
                    };
                }
            }
        }
    }

    total_size
}

fn humanize_size(bytes: usize) -> String {
    if bytes == 0 {
        return "0".to_string();
    }
    let suffix = ["", "K", "M", "G", "T"];
    let i = ((bytes as f64).ln() / (1024.0_f64.ln())).floor() as usize;
    let index = i.min(suffix.len() - 1);
    let value = bytes as f64 / 1024.0_f64.powi(index as i32);

    if index == 0 {
        format!("{}{}", bytes, suffix[index])
    } else {
        let formatted_val = format!("{}", value);

        format!("{}{}", formatted_val, suffix[index])
    }
}

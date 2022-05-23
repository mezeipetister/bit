use std::path::{Path, PathBuf};

use walkdir::WalkDir;

#[derive(Default, Debug)]
pub struct Doc {
    name: String,
    relative_path: PathBuf,
    note: Option<String>,
}

impl Doc {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn relative_path(&self) -> &Path {
        self.relative_path.as_path()
    }
    pub fn note(&self) -> Option<&String> {
        self.note.as_ref()
    }
}

pub fn get_files_recours(
    root_path: &Path,
    extension: Option<&'static str>,
) -> Result<Vec<Doc>, String> {
    if !root_path.exists() {
        return Err(format!("Path {:?} not exist", &root_path));
    }
    if !root_path.is_dir() {
        return Err(format!("Path {:?} is not a directory", &root_path));
    }
    let mut res: Vec<Doc> = Vec::new();
    for entry in WalkDir::new(root_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if entry.metadata().unwrap().is_file() {
            if let Some(ext) = extension {
                if entry.path().extension().unwrap() != ext {
                    continue;
                }
            }
            res.push(Doc {
                name: f_name.to_string(),
                relative_path: entry.into_path(),
                note: None,
            });
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_get_all_files() {
        let files = get_files_recours(Path::new("example"), None);
        assert_eq!(files.is_ok(), true);
        println!("{:?}", files);
    }
}

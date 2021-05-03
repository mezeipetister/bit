use std::path::{Path, PathBuf};

use packman::Pack;

use crate::file::{File, FilesDiff};
pub struct FileIndex {
  index_path: PathBuf,
  files: Pack<Vec<File>>,
}

impl FileIndex {
  pub fn load(index_path: &Path) -> Self {
    let files =
      Pack::load_or_init(index_path.to_path_buf(), "files").expect("Error loading files db");
    Self {
      index_path: index_path.to_path_buf(),
      files,
    }
  }
  // pub fn check_new<'a>(&'a self) -> FilesDiff<'a> {
  //   let new_files = crate::file::read_files_recurs(&self.index_path);
  //   crate::file::diff_files(&*self.files, &new_files)
  // }
}

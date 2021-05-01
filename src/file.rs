use serde::{Deserialize, Serialize};
use std::{
  ffi::OsString,
  fs,
  path::{Path, PathBuf},
  time::SystemTime,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
  path: PathBuf,
  name: Option<OsString>,
  modified: SystemTime,
}

pub fn read_files_recurs(folder: &Path) -> Vec<File> {
  let mut res = Vec::new();
  for entry in fs::read_dir(&folder).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();

    let metadata = fs::metadata(&path).unwrap();
    // let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
    let modified = metadata.modified().unwrap();
    let name = path.file_name().map(|n| n.into());

    res.push(File {
      path,
      name,
      modified,
    });
  }
  res
}

#[derive(Debug)]
pub struct FilesDiff<'a> {
  new: Vec<&'a File>,
  removed: Vec<&'a File>,
  modified: Vec<&'a File>,
  unmodified: Vec<&'a File>,
}

impl<'a> FilesDiff<'a> {
  pub fn new() -> Self {
    Self {
      new: Vec::new(),
      removed: Vec::new(),
      modified: Vec::new(),
      unmodified: Vec::new(),
    }
  }
  pub fn is_clean(&self) -> bool {
    self.new.is_empty() && self.removed.is_empty() && self.modified.is_empty()
  }
}

pub fn diff_files<'a>(past: &'a Vec<File>, now: &'a Vec<File>) -> FilesDiff<'a> {
  let mut pfiles: Vec<&File> = past.iter().map(|f| f).collect();
  let nfiles: Vec<&File> = now.iter().map(|f| f).collect();
  let mut res = FilesDiff::new();
  for nf in nfiles {
    // If we have this file
    if let Some(pos) = pfiles.iter().position(|_pf| _pf.path == nf.path) {
      if let Some(_pf) = pfiles.get(pos) {
        if _pf.modified < nf.modified {
          res.modified.push(nf);
        } else {
          res.unmodified.push(nf);
        }
        // Remove file from pfiles
        pfiles.remove(pos);
      }
    }
    // This file is a new file
    else {
      res.new.push(nf);
    }
  }
  // Now add removed files to result
  pfiles.iter().for_each(|f| res.removed.push(*f));
  // Return result
  res
}

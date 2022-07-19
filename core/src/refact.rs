pub struct Project {
  project_root_path: PathBuf,
  settings: Settings,
  accounts: Vec<Account>,
  notes: Vec<Note>,
  pub ledger: Ledger,
}

struct Note {
  has_pdf: bool,
  note: (),
}

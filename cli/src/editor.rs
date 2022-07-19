trait ExtBuffer {
  fn display(&self);
  fn set_key_event(&mut self, key_event: ());
}

struct Cli {
  window_size: (usize, usize),
  buffer: Box<dyn ExtBuffer>,
  cli_mode: CliMode,
}

enum CliMode {
  Edit,
  View,
}

struct Editor {
  content: Vec<u8>,
}

struct Buffer {}

// Editor::new(String) -> Buffer;

// :al ->

// in build.rs
use clap::{Clap, IntoApp};
use clap_generate::{generate_to, generators::Bash};

include!("src/main.rs");

fn main() {
    let mut app = Args::into_app();
    app.set_bin_name("bit");

    let outdir = env!("CARGO_MANIFEST_DIR");
    generate_to::<Bash, _, _>(&mut app, "bit", outdir);
}

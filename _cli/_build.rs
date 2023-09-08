use bit::App;

fn main() {
    clap_complete::generate_to(
        clap_complete_fig::Fig,
        &mut App::into_app(),
        "bit",
        "fig_spec",
    )
    .expect("Unable to generate Fig spec");
}

use editor::Document;

fn main() {
    let clbk = Box::new(|c| {
        println!("Saved!");
        Ok(())
    });
    let doc = Document::new("Demo".into(), "abc".into(), clbk);
    editor::Editor::new(doc).unwrap().run().unwrap();
}

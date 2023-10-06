use editor::Document;

fn main() {
    let clbk = Box::new(|c| {
        println!("Saved!");
        Ok(())
    });
    let doc = Document::new("Demo".into(), "".into(), clbk);
    let res = editor::Editor::new(doc).unwrap().run().unwrap();
    println!("{}", res);
}

struct FileObject {
    local: Option<Vec<u8>>,
    remote: Vec<()>,
}

struct Commit {
    id: (),
    uid: (),
    dtime: (),
    objects: Vec<Object>,
    signature: (),
}

struct Object {
    id: (),
    data: Vec<u8>,
    signature: (),
}

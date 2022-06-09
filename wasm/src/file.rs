use std::{path::PathBuf, rc::Rc};
enum Filetype {
    File,
    Dir(Option<Vec<Rc<FileInfo>>>),
    Symboric(Rc<FileInfo>),
}
struct Authority {
    owner: u8,
    group: u8,
    other: u8,
}

struct FileInfo {
    name: String,
    ftype: Filetype,
    auth: Authority,
    path: PathBuf,
}

pub struct FileTree {
    pwd: Rc<FileInfo>,
    root: Rc<FileInfo>,
    home: Rc<FileInfo>,
}

impl Authority {
    fn new() -> Self {
        return Authority {
            owner: 7,
            group: 7,
            other: 7,
        };
    }
}

impl FileInfo {
    fn new(ftype: Filetype, name: String, path: PathBuf) -> Self {
        return FileInfo {
            name,
            ftype,
            auth: Authority::new(),
            path,
        };
    }
    fn ls(self) -> Vec<Rc<FileInfo>> {
        // let mut files = Vec::new();
        let infos = match self.ftype {
            Filetype::File => vec![Rc::new(self)],
            Filetype::Dir(v) => v,
            Filetype::Symboric(_) => vec![Rc::new(self)],
        };
        eprintln!("/////////////////////////////");
        for info in &infos {
            eprintln!("{} ", info.name);
        }
        infos
    }
}

impl FileTree {
    pub fn new() -> Self {
        let root = FileInfo::new(
            Filetype::Dir(Option::None),
            "root".to_string(),
            PathBuf::from("/"),
        );
        FileTree {
            pwd: Rc::new(root),
            root: Rc::new(root),
            home: Rc::new(root),
        }
    }
    pub fn json_to_filetree() -> Self {
        let root = FileInfo::new(
            Filetype::Dir(Option::None),
            "root".to_string(),
            PathBuf::new(),
        );
        FileTree {
            pwd: Rc::new(root),
            root: Rc::new(root),
            home: Rc::new(root),
        }
    }
}

#[test]
fn ls_test() {
    let info = FileInfo::new(Filetype::File, "hello".to_string(), PathBuf::new());
    info.ls();
}

#[test]
fn make_sample() {
    let output_path = "sample/sample.json";
    let mut tree = FileTree::new();
}

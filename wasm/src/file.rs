use std::{path::PathBuf, rc::Rc};
#[derive(Debug, Clone)]
enum Filetype {
    File,
    Dir(Option<Vec<Rc<FileInfo>>>),
    Symboric(Rc<FileInfo>),
}
#[derive(Debug, Clone)]
struct FileAuthority {
    owner: Authority,
    group: Authority,
    other: Authority,
}

#[derive(Debug, Clone)]
struct Authority(u8);

#[derive(Debug, Clone)]
struct FileInfo {
    name: String,
    ftype: Filetype,
    auth: FileAuthority,
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FileTree {
    pwd: Rc<FileInfo>,
    root: Rc<FileInfo>,
    home: Rc<FileInfo>,
}

impl FileAuthority {
    fn new() -> Self {
        return FileAuthority {
            owner: Authority(7),
            group: Authority(7),
            other: Authority(7),
        };
    }
}

impl FileInfo {
    fn new(ftype: Filetype, name: String, path: PathBuf) -> Self {
        return FileInfo {
            name,
            ftype,
            auth: FileAuthority::new(),
            path,
        };
    }
    fn ls(fileinfo: Rc<FileInfo>) -> Vec<Rc<FileInfo>> {
        // let to_self = Rc::new(self);

        let infos = match fileinfo.ftype.clone() {
            Filetype::File => vec![fileinfo.clone()],
            Filetype::Dir(Some(v)) => v.clone(),
            Filetype::Dir(None) => Vec::new(),
            Filetype::Symboric(_) => vec![fileinfo.clone()],
        };
        for info in &infos {
            println!("{} ", info.name);
        }
        infos
    }
}

impl FileTree {
    pub fn new() -> Self {
        let root = Rc::new(FileInfo::new(
            Filetype::Dir(Option::None),
            "root".to_string(),
            PathBuf::from("/"),
        ));
        let pwd = root.clone();
        let home = root.clone();

        FileTree { pwd, root, home }
    }
    pub fn json_to_filetree() -> Self {
        FileTree::new()
    }
    fn dir_to_dir(from_dir: PathBuf, to_dir: PathBuf) -> Vec<FileInfo> {
        Vec::new()
    }

    // pub fn cd(mut self, path: &PathBuf) {
    //     let pwd_files;
    //     if path.as_bytes()[0] == b'/' {
    //         // todo
    //         pwd_files = FileInfo::ls(self.root.clone());
    //     } else {
    //         pwd_files = FileInfo::ls(self.pwd.clone());
    //     }

    //     let mut flag = false;
    //     for file in pwd_files {
    //         if let Filetype::Dir(_) = file.ftype {
    //             if file.name == path {
    //                 flag = true;
    //                 self.pwd = file.clone();
    //                 println!("{:?}", self.pwd);
    //                 return;
    //             }
    //         }
    //     }
    //     println!("not found");
    // }
}

#[test]
fn ls_test() {
    let info = FileInfo::new(Filetype::File, "hello".to_string(), PathBuf::new());
    // info.ls();
}

#[test]
fn make_sample() {
    let output_path = "sample/sample.json";
    let mut tree = FileTree::new();
}

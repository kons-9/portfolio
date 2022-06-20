use std::{path::PathBuf, rc::Rc};
#[derive(Debug, Clone)]
enum Filetype {
    File,
    Dir(Vec<Rc<FileInfo>>),
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
pub struct FileInfo {
    pub name: String,
    ftype: Filetype,
    auth: FileAuthority,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct FileTree {
    pub pwd: Rc<FileInfo>,
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
    fn new(name: String, ftype: Filetype, path: PathBuf) -> Self {
        return FileInfo {
            name,
            ftype,
            auth: FileAuthority::new(),
            path,
        };
    }
    fn ls(fileinfo: Rc<FileInfo>) -> Vec<Rc<FileInfo>> {
        // let to_self = Rc::new(self);
        let infos: Vec<Rc<FileInfo>> = match fileinfo.ftype {
            Filetype::File => vec![fileinfo.clone()],
            Filetype::Dir(ref v) => v.iter().map(|x| x.clone()).collect(),
            Filetype::Symboric(_) => vec![fileinfo.clone()],
        };
        for info in &infos {
            println!("{} ", info.name);
        }
        infos
    }
    fn mkdir(&mut self, name: impl Into<String>) -> Result<(), String> {
        let name = name.into();
        match &mut self.ftype {
            Filetype::Dir(x) => {
                for file in x.iter() {
                    if file.name == name {
                        return Err(format!("{}: File exists", name));
                    }
                }
                let path = self.path.clone().join(&name);
                Ok(x.push(Rc::new(FileInfo::new(
                    name,
                    Filetype::Dir(Vec::new()),
                    path,
                ))))
            }
            x => panic!("current position must be 'directory', but {:?}", x),
        }
    }
    fn cd(&self, name: impl Into<String>) -> Result<Rc<FileInfo>, String> {
        let name = name.into();
        match &self.ftype {
            Filetype::Dir(x) => {
                for file in x.iter() {
                    if file.name == name {
                        return Ok(file.clone());
                    }
                }
            }
            x => panic!("current position must be 'directory', but {:?}", x),
        }
        Err(format!("{}: File exists", name))
    }
}

impl FileTree {
    pub fn new() -> Self {
        let home = Filetype::Dir(Vec::new());

        let root = Rc::new(FileInfo::new(
            "root".to_string(),
            Filetype::Dir(Vec::new()),
            PathBuf::from("/"),
        ));
        // root.mkdir("usr");
        // root.mkdir("bin");
        // root.mkdir("User");
        // root.mkdir("");
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
    pub fn find_current_dir(&self, name: &str) -> Option<Rc<FileInfo>> {
        let pwd = self.pwd.clone();
        let infos = FileInfo::ls(pwd);
        for info in infos {
            if name == info.name {
                return Some(info);
            }
        }
        None
    }
}

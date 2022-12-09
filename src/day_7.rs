#![allow(dead_code)]

use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
pub struct Folder {
    children: Vec<Rc<Resource>>,
    parent: Option<Rc<RefCell<Folder>>>,
    path: String,
}

impl Folder {
    fn touch(&mut self, file: &File) {
        self.children.push(Rc::new(Resource::File(file.clone())));
    }
    fn mkdir(&mut self, folder: &Folder) {
        self.children
            .push(Rc::new(Resource::Folder(folder.clone())));
    }
}

#[derive(Debug)]
pub struct FileSystem {
    tree: Vec<Rc<RefCell<Folder>>>,
    current: Option<usize>,
}

impl Default for FileSystem {
    fn default() -> Self {
        Self {
            tree: vec![],
            current: None,
        }
    }
}

impl FileSystem {
    fn find(&self, path: &str) -> usize {
        self.tree
            .iter()
            .position(|x| x.borrow().path == path)
            .unwrap()
    }
    fn retrieve(&self, idx: usize) -> Ref<Folder> {
        self.tree.get(idx).unwrap().borrow()
    }
    fn execute(&mut self, cmd: &Command) {
        match cmd {
            Command::Root => {
                if self.tree.len() > 0 {
                    self.current = Some(0);
                }
            }
            Command::Back => {
                if let Some(ref current) = self.current {
                    let current = self.retrieve(current.clone());
                    let parent = current.parent.clone();
                    drop(current);
                    if let Some(parent) = parent {
                        let idx = self.find(parent.borrow().path.as_str());
                        self.current = Some(idx);
                    }
                }
            }
            Command::To(path) => {
                let idx = self.find(path);
                self.current = Some(idx);
            }
            Command::Ls => {}
        };
    }
    fn populate(&mut self, stdout: &StdOut) {
        for line in stdout.0.iter() {
            match line {
                StdOutLine::Cmd(cmd) => {
                    self.execute(cmd);
                }
                StdOutLine::Output(resource) => match resource {
                    Resource::File(file) => {
                        let idx = self.current.unwrap();
                        let folder = &**self.tree.get(idx).unwrap();
                        let mut guard = folder.borrow_mut();
                        guard.touch(file);
                    }
                    Resource::Folder(child) => {
                        if self.tree.len() > 0 {
                            let idx = self.current.unwrap();
                            let initial = self.tree.get(idx).unwrap();
                            let folder = &**initial;
                            let mut guard = folder.borrow_mut();
                            let child = Folder {
                                children: vec![],
                                parent: Some(Rc::clone(initial)),
                                path: child.path.clone(),
                            };
                            guard.mkdir(&child);
                        } else {
                            let root = Folder {
                                children: vec![],
                                parent: None,
                                path: child.path.clone(),
                            };
                            self.tree.push(Rc::new(RefCell::new(root)));
                            self.current = Some(0);
                        }
                    }
                },
            };
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    To(String),
    Root,
    Back,
    Ls,
}

#[derive(Debug)]
pub enum Resource {
    File(File),
    Folder(Folder),
}

pub trait Size {
    fn size(&self) -> usize;
}

impl Size for File {
    fn size(&self) -> usize {
        self.size
    }
}

impl Size for Folder {
    fn size(&self) -> usize {
        self.children.iter().map(|x| x.size()).sum::<usize>()
    }
}

impl Size for Resource {
    fn size(&self) -> usize {
        match self {
            Resource::File(file) => file.size(),
            Resource::Folder(folder) => folder.size(),
        }
    }
}

#[derive(Debug)]
pub enum StdOutLine {
    Cmd(Command),
    Output(Resource),
}

#[derive(Debug)]
pub struct StdOut(Vec<StdOutLine>);

impl From<&str> for StdOut {
    fn from(v: &str) -> Self {
        let mut stdout = vec![];
        for line in v.lines() {
            stdout.push(StdOutLine::from(line));
        }
        Self(stdout)
    }
}

impl From<&str> for StdOutLine {
    fn from(v: &str) -> Self {
        if v.starts_with('$') {
            return StdOutLine::Cmd(Command::from(v));
        }
        if v.starts_with("dir") {
            return StdOutLine::Output(Resource::Folder(Folder::from(v)));
        }
        StdOutLine::Output(Resource::File(File::from(v)))
    }
}

impl From<&str> for Folder {
    fn from(v: &str) -> Self {
        Self {
            children: vec![],
            parent: None,
            path: v[4..].to_string(),
        }
    }
}

impl From<&str> for File {
    fn from(v: &str) -> Self {
        let parts: Vec<&str> = v.split(" ").collect();
        assert!(parts.len() == 2);
        let size = parts
            .get(0)
            .expect("filesize")
            .parse()
            .expect("should be a digit");
        let name = parts.get(1).expect("filename").deref();
        Self {
            name: name.into(),
            size,
        }
    }
}

impl From<&str> for Command {
    fn from(v: &str) -> Self {
        match &v[2..] {
            "ls" => Command::Ls,
            "cd .." => Command::Back,
            "cd /" => Command::Root,
            _ => Command::To(v[5..].to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, File, FileSystem, Folder, StdOut};

    const INPUT: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn parse() {
        let line = "$ ls";
        let cmd = Command::from(line);
        assert_eq!(cmd, Command::Ls);

        let line = "$ cd /";
        let cmd = Command::from(line);
        assert_eq!(cmd, Command::Root);

        let line = "$ cd ..";
        let cmd = Command::from(line);
        assert_eq!(cmd, Command::Back);

        let line = "$ cd e";
        let cmd = Command::from(line);
        assert_eq!(cmd, Command::To("e".into()));

        let line = "dir a";
        let folder = Folder::from(line);
        assert_eq!(folder.path.as_str(), "a");

        let line = "8033020 d.log";
        let file = File::from(line);
        assert_eq!(file.size, 8033020);
        assert_eq!(file.name.as_str(), "d.log");
    }

    #[test]
    fn sample() {
        let stdout = StdOut::from(INPUT);
        println!("{stdout:#?}");
        let mut fs = FileSystem::default();
        fs.populate(&stdout);
        println!("{fs:#?}");
        assert!(false);
    }
}

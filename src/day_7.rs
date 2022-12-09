#![allow(dead_code)]

use std::{
    borrow::Borrow,
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

impl Default for Folder {
    fn default() -> Self {
        Self {
            children: vec![],
            parent: None,
            path: String::from("/"),
        }
    }
}

impl Folder {
    fn touch(&mut self, file: &File) {
        println!("touch {file:#?}");
        self.children.push(Rc::new(Resource::File(file.clone())));
    }
    fn mkdir(&mut self, folder: &Folder) {
        println!("mkdir {folder:#?}");
        self.children
            .push(Rc::new(Resource::Folder(folder.clone())));
    }
    fn populate(&mut self, stdout: StdOut) {
        let mut current: Folder = self.to_owned();
        for line in stdout.0.iter() {
            match line {
                StdOutLine::Cmd(cmd) => match cmd {
                    Command::To(path) => {
                        let folder = current
                            .children
                            .into_iter()
                            .find_map(|x| match &*x {
                                Resource::File(_) => None,
                                Resource::Folder(folder) => {
                                    if folder.path.as_str() == path.as_str() {
                                        return Some(folder.borrow().clone());
                                    } else {
                                        return None;
                                    }
                                }
                            })
                            .unwrap();
                        current = folder.clone();
                    }
                    Command::Root => {
                        while let Some(parent) = current.parent {
                            current = (&*parent).borrow().clone();
                        }
                    }
                    Command::Back => current = (&*self.parent.clone().unwrap()).borrow().clone(),
                    Command::Ls => {}
                },
                StdOutLine::Output(_) => todo!(),
            }
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

impl Resource {
    fn is_dir(&self) -> bool {
        match self {
            Resource::File(_) => false,
            Resource::Folder(_) => true,
        }
    }
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
    use super::{Command, File, Folder, StdOut};

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
        let mut root = Folder::default();
        root.populate(stdout);
        assert!(false);
    }
}

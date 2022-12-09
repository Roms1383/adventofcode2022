#![allow(dead_code)]

use std::{borrow::Borrow, rc::Rc};

pub struct File {
    name: String,
    size: usize,
}

#[derive(Clone)]
pub struct Folder {
    children: Vec<Rc<Resource>>,
    parent: Option<Rc<Folder>>,
    path: String,
}

pub struct FileSystem {
    tree: Vec<Rc<Folder>>,
    current: Option<usize>,
}

impl FileSystem {
    fn find(&self, path: &str) -> usize {
        self.tree.iter().position(|x| x.path == path).unwrap()
    }
    fn retrieve(&self, idx: usize) -> &Folder {
        self.tree.get(idx).unwrap()
    }
    fn goto(&mut self, cmd: &Command) {
        match cmd {
            Command::Root => {
                self.current = Some(self.find("/"));
            }
            Command::Back => {
                if let Some(ref current) = self.current {
                    let current = self.retrieve(current.clone());
                    let parent = current.parent.clone();
                    if let Some(parent) = parent {
                        let idx = self.find(parent.path.as_str());
                        self.current = Some(idx);
                    }
                }
            }
            Command::To(path) => {
                let idx = self.find(path);
                self.current = Some(idx);
            }
        };
    }
}

pub enum Command {
    To(String),
    Root,
    Back,
}

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

#[cfg(test)]
mod tests {
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
    fn sample() {}
}

#![allow(dead_code)]

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Resource {
    File(File),
    Folder(Folder),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FullPath(String);

impl Display for FullPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for FullPath {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl FullPath {
    pub fn contains_str(&self, v: &str) -> bool {
        self.0.contains(v)
    }
    pub fn starts_with_str(&self, v: &str) -> bool {
        self.0.starts_with(v)
    }
}

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: usize,
    parent: FullPath,
}

#[derive(Debug, Clone)]
pub struct Folder {
    parent: Option<String>,
    name: String,
}

impl Default for Folder {
    fn default() -> Self {
        Self {
            parent: None,
            name: String::from("/"),
        }
    }
}

impl Folder {
    fn path(&self) -> FullPath {
        match (&self.parent, &self.name) {
            (Some(ref parent), name) if parent.len() == 1 => {
                FullPath(format!("{}{}", parent, name))
            }
            (Some(ref parent), name) if parent.len() > 1 => {
                FullPath(format!("{}/{}", parent, name))
            }
            (_, name) => FullPath(name.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    To(String),
    Root,
    Back,
    Ls,
    Dir(String),
    File(usize, String),
}

#[derive(Debug)]
pub struct StdOut(Vec<Command>);

#[derive(Debug)]
pub struct FileSystem {
    tree: Vec<Resource>,
    current: String,
}

impl Default for FileSystem {
    fn default() -> Self {
        let root = Folder::default();
        Self {
            current: root.name.clone(),
            tree: vec![Resource::Folder(root)],
        }
    }
}

impl From<&str> for StdOut {
    fn from(v: &str) -> Self {
        let mut fs = vec![];
        for line in v.lines() {
            fs.push(Command::from(line));
        }
        Self(fs)
    }
}

impl From<&str> for Command {
    fn from(v: &str) -> Self {
        if v.starts_with("$") {
            let inner = &v["$".len() + 1..];
            if inner.starts_with("ls") {
                return Command::Ls;
            }
            if inner.starts_with("cd") {
                if inner.ends_with('/') {
                    return Command::Root;
                }
                if inner.ends_with("..") {
                    return Command::Back;
                }
                return Command::To(inner["cd".len() + 1..].to_string());
            }
        }
        if v.starts_with("dir") {
            return Command::Dir(v["dir".len() + 1..].to_string());
        }
        let parts: Vec<&str> = v.split(' ').collect();
        assert!(parts.len() == 2);
        let size = parts
            .first()
            .expect("filesize")
            .parse::<usize>()
            .expect("should be a digit");
        let name = parts.get(1).expect("filename");
        return Command::File(size, name.to_string());
    }
}

impl From<StdOut> for FileSystem {
    fn from(v: StdOut) -> Self {
        let mut root = FileSystem::default();
        for line in v.0.iter() {
            match line {
                Command::To(dir) => {
                    root.current = if root.current.len() == 1 {
                        format!("{}{}", root.current, dir)
                    } else {
                        format!("{}/{}", root.current, dir)
                    };
                }
                Command::Root => {
                    root.current = "/".into();
                }
                Command::Back => {
                    let last = root.current.rfind('/').expect("find slash");
                    if last > 1 {
                        root.current = root.current[0..last - 1].into();
                    }
                }
                Command::Ls => {}
                Command::Dir(path) => {
                    root.tree.push(Resource::Folder(Folder {
                        name: path.clone(),
                        parent: Some(root.current.clone()),
                    }));
                }
                Command::File(size, name) => {
                    root.tree.push(Resource::File(File {
                        name: name.clone(),
                        size: *size,
                        parent: FullPath(root.current.clone()),
                    }));
                }
            };
        }
        root
    }
}

impl File {
    fn is_in(&self, path: &FullPath) -> bool {
        &self.parent == path
    }
    fn is_nested_in(&self, path: &FullPath) -> bool {
        let x = path.starts_with_str(self.parent.as_ref());
        println!("file at {} is nested in {} ({x})", path, self.parent);
        x
    }
}

pub trait Size {
    fn size(&self) -> usize;
}

impl Size for Vec<&File> {
    fn size(&self) -> usize {
        self.iter().map(|x| x.size).sum()
    }
}

impl Size for FileSystem {
    fn size(&self) -> usize {
        let files: Vec<&File> = self
            .tree
            .iter()
            .filter_map(|x| match x {
                Resource::File(file) => Some(file),
                Resource::Folder(_) => None,
            })
            .collect();
        files.iter().map(|x| x.size).sum()
    }
}

impl FileSystem {
    pub fn find_nested_files(&self, path: &FullPath) -> Vec<&File> {
        self.tree
            .iter()
            .filter_map(|x| match x {
                Resource::File(file) => {
                    // println!("[find_nested_files] search for nested file at {}", path);
                    if file.is_nested_in(path) {
                        Some(file)
                    } else {
                        None
                    }
                }
                Resource::Folder(_) => None,
            })
            .collect()
    }
    pub fn find_dirs(&self) -> Vec<&Folder> {
        self.tree
            .iter()
            .filter_map(|x| match x {
                Resource::File(_) => None,
                Resource::Folder(dir) => Some(dir),
            })
            .collect()
    }
    pub fn find_lightweight_dirs(&self, max: usize) -> Vec<&Folder> {
        self.find_dirs()
            .into_iter()
            .filter(|x| {
                // println!(
                //     "[find_lightweight_dirs] search for nested file at {}",
                //     x.path()
                // );
                self.find_nested_files(&x.path()).size() <= max
            })
            .collect()
    }
    pub fn dirs_size(&self, dirs: Vec<&Folder>) -> usize {
        dirs.iter()
            .map(|x| {
                // println!("[dirs_size] search for nested file at {}", x.path());
                self.find_nested_files(&x.path()).size()
            })
            .sum()
    }
    pub fn dir_size(&self, path: &FullPath) -> usize {
        self.find_nested_files(path).size()
    }
    pub fn sum_lightweight_dirs(&self, max: usize) -> usize {
        self.find_dirs()
            .iter()
            .filter_map(|x| {
                // println!(
                //     "[sum_lightweight_dirs] search for nested file at {}",
                //     x.path()
                // );
                let size = self.find_nested_files(&x.path()).size();
                if size <= max {
                    return Some(size);
                }
                None
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_7::{FileSystem, FullPath, StdOut};

    use super::Command;

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
        let folder = Command::from(line);
        assert_eq!(folder, Command::Dir("a".into()));

        let line = "8033020 d.log";
        let file = Command::from(line);
        assert_eq!(file, Command::File(8033020, "d.log".into()))
    }

    #[test]
    fn sizes() {
        let stdout = StdOut::from(INPUT);
        let fs = FileSystem::from(stdout);
        let size = fs.dir_size(&FullPath(String::from("/a/e")));
        assert_eq!(size, 584);
        let size = fs.dir_size(&FullPath(String::from("/a")));
        assert_eq!(size, 94_853);
        let size = fs.dir_size(&FullPath(String::from("/d")));
        assert_eq!(size, 24_933_642);
        let size = fs.dir_size(&FullPath(String::from("/")));
        assert_eq!(size, 48_381_165);
    }

    #[test]
    fn lightweight() {
        let stdout = StdOut::from(INPUT);
        let fs = FileSystem::from(stdout);
        println!("{fs:#?}");
        let lightweight = fs.find_lightweight_dirs(100_000);
        let names: Vec<&str> = lightweight.iter().map(|x| x.name.as_str()).collect();
        assert_eq!(names, vec!["a", "e"]);
        let total = fs.dirs_size(lightweight);
        assert_eq!(total, 95_437);
        let total = fs.sum_lightweight_dirs(100_000);
        assert_eq!(total, 95_437);
        // assert!(false);
    }
}

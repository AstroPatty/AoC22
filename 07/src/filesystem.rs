use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug)]
enum DirEntry {
    File((String, usize)),
    Directory(Directory),
    Invalid,
}

type DirListing = Vec<DirEntry>;

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub listing: RefCell<HashMap<String, DirEntry>>,
}

#[derive(Debug)]
pub enum LineEntry {
    Cd(String),
    Ls,
    LsEntry(DirEntry),
    End,
}

pub struct FileSystem {
    pub root: Directory,
}

pub struct Session {
    pub cwd: String,
    pub fs: FileSystem,
}

impl Session {
    pub fn new() -> Self {
        return Session {
            cwd: String::from(""),
            fs: FileSystem::new(),
        };
    }

    pub fn run_session(&mut self, data: String) {
        let lines: Vec<&str> = data.lines().collect();
        let line_iterator = lines.iter().enumerate();
        let mut skip = 0;
        for (i, line) in line_iterator {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            let cmd = parse_command(line);
            match cmd {
                LineEntry::Cd(dirname) => self.cd(&dirname),
                LineEntry::Ls => {
                    let slice = &lines[i + 1..];
                    let dir_entries = self.get_dir_entries(slice);
                    skip = dir_entries.len();
                    self.fs.insert(&self.cwd, dir_entries);
                }
                LineEntry::LsEntry(_) => panic!(),
                LineEntry::End => return,
            }
        }
    }

    fn get_dir_entries(&self, data: &[&str]) -> HashMap<String, DirEntry> {
        let mut entries: HashMap<String, DirEntry> = HashMap::new();
        for &entry_str in data {
            let entry = parse_dir_entry(entry_str);
            match entry {
                DirEntry::Invalid => break,
                DirEntry::Directory(ref dir) => {
                    let _ = entries.insert(dir.name.clone(), entry);
                }
                DirEntry::File((ref name, _)) => {
                    let _ = entries.insert(name.to_owned(), entry);
                }
            }
        }
        return entries;
    }

    fn cd(&mut self, new_directory: &str) {
        if new_directory == "/" {
            self.cwd = String::from("");
            return;
        } else if new_directory == ".." {
            let cwd_parts: Vec<&str> = self.cwd.split("/").collect();
            let new_cwd = cwd_parts[..cwd_parts.len() - 1].join("/");
            self.cwd = new_cwd;
        } else if self.cwd == "" {
            self.cwd = new_directory.to_owned();
        } else {
            let new_cwd = format!("{}/{}", self.cwd, new_directory);
            self.cwd = new_cwd;
        }
    }
}

impl Directory {
    fn new(name: &str) -> Self {
        return Directory {
            name: name.to_owned(),
            listing: RefCell::new(HashMap::new()),
        };
    }

    fn insert(&self, mut path: impl Iterator<Item = impl AsRef<str>>, name: &str, entry: DirEntry) {
        let next = path.next();
        if next.is_none() {
            self.listing.borrow_mut().insert(name.to_owned(), entry);
            return;
        }

        let mut self_entries = self.listing.borrow_mut();
        let self_entry = self_entries.get(next.unwrap().as_ref());
        if let Some(e) = self_entry {
            match e {
                DirEntry::Invalid => panic!(),
                DirEntry::File(_) => panic!(),
                DirEntry::Directory(dir) => dir.insert(path, name, entry),
            }
        } else {
            self_entries.insert(name.to_owned(), entry);
        }
    }

    pub fn get_sizes(&self, entries: &mut HashMap<String, usize>) {
        let mut self_size: usize = 0;
        for (_, entry) in self.listing.borrow().iter() {
            match entry {
                DirEntry::Invalid => panic!(),
                DirEntry::File((_, size)) => self_size += size,
                DirEntry::Directory(dir) => {
                    let mut sub_entries = HashMap::new();
                    dir.get_sizes(&mut sub_entries);
                    for (cname, csize) in sub_entries.drain() {
                        let new_cname = if self.name == "/" {
                            format!("/{}", cname)
                        } else {
                            format!("{}/{}", self.name, cname)
                        };
                        entries.insert(new_cname, csize);
                        if !cname.contains('/') {
                            self_size += csize;
                        }
                    }
                }
            }
        }
        entries.insert(self.name.clone(), self_size);
    }
}

impl FileSystem {
    fn new() -> Self {
        let root = Directory {
            name: String::from("/"),
            listing: RefCell::new(HashMap::new()),
        };
        FileSystem { root: root }
    }
    pub fn get_sizes(&self) -> HashMap<String, usize> {
        let mut sizes = HashMap::new();
        self.root.get_sizes(&mut sizes);
        return sizes;
    }
    fn insert(&mut self, path: &str, mut entries: HashMap<String, DirEntry>) {
        let path_parts = path.split("/");

        for (name, entry) in entries.drain() {
            self.root.insert(path_parts.clone(), &name, entry)
        }
    }
}

pub fn parse_line(data: &str) -> LineEntry {
    if data.starts_with("$") {
        let command = &data[2..];
        return parse_command(command);
    } else {
        let entry = parse_dir_entry(data);
        return LineEntry::LsEntry(entry);
    }
}

fn parse_command(data: &str) -> LineEntry {
    if data.starts_with("$ cd") {
        let dir_name = data[5..].to_owned();
        return LineEntry::Cd(dir_name);
    } else if data.starts_with("$ ls") {
        return LineEntry::Ls;
    } else {
        return LineEntry::End;
    }
}

fn parse_dir_entry(data: &str) -> DirEntry {
    let mut entries = data.split_whitespace();
    let first = entries.next().unwrap();
    let size = first.parse::<usize>();
    if let Ok(s) = size {
        return DirEntry::File((entries.next().unwrap().to_owned(), s));
    } else if first.starts_with("dir") {
        let dirname = entries.next().unwrap();
        return DirEntry::Directory(Directory::new(dirname));
    } else {
        return DirEntry::Invalid;
    }
}

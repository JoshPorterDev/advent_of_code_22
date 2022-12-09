use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Default)]
struct Directory {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Directory>>,
    children: RefCell<HashMap<String, Rc<Directory>>>,
}

impl Directory {
    fn new() -> Self {
        Self {
            _name: String::new(),
            size: RefCell::new(0),
            parent: None,
            children: RefCell::new(HashMap::new()),
        }
    }
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .children
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

fn main() {
    let file = fs::read_to_string("src/input").expect("Failed to read file");
    part_one(&file);
    part_two(&file);
}

fn part_one(file: &str) {
    let root: Rc<Directory> = Rc::new(Directory::new());
    let mut current_dir = Rc::clone(&root);

    for l in file.lines() {
        let words = l.split_whitespace().collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => current_dir = Rc::clone(&root),
                ".." => current_dir = Rc::clone(current_dir.parent.as_ref().unwrap()),
                new_dir => {
                    let new_dir = current_dir.children.borrow().get(new_dir).unwrap().clone();
                    current_dir = new_dir
                }
            },
            ("dir", dir) => {
                current_dir.children.borrow_mut().insert(
                    dir.to_string(),
                    Rc::new(Directory {
                        _name: dir.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&current_dir)),
                        children: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _file) => {
                *current_dir.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }

    let mut to_visit = vec![Rc::clone(&root)];
    let mut total = 0;
    while let Some(dir) = to_visit.pop() {
        for d in dir.children.borrow().values() {
            to_visit.push(Rc::clone(d));
        }

        let size = dir.get_size();
        if size <= 100000 {
            total += size;
        }
    }

    println!("Total size of directories under 1000000: {total}");
}

fn part_two(file: &str) {
    let root: Rc<Directory> = Rc::new(Directory::new());
    let mut current_dir = Rc::clone(&root);

    for l in file.lines() {
        let words = l.split_whitespace().collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => current_dir = Rc::clone(&root),
                ".." => current_dir = Rc::clone(current_dir.parent.as_ref().unwrap()),
                new_dir => {
                    let new_dir = current_dir.children.borrow().get(new_dir).unwrap().clone();
                    current_dir = new_dir
                }
            },
            ("dir", dir) => {
                current_dir.children.borrow_mut().insert(
                    dir.to_string(),
                    Rc::new(Directory {
                        _name: dir.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&current_dir)),
                        children: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _file) => {
                *current_dir.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }

    let mut to_visit = vec![Rc::clone(&root)];
    let root_total = &root.get_size();
    let mut target_dirs: Vec<usize> = Vec::new();
    let needed_space = 30000000 - (70000000 - root_total);
    while let Some(dir) = to_visit.pop() {
        for d in dir.children.borrow().values() {
            to_visit.push(Rc::clone(d));
        }

        let size = dir.get_size();
        if size >= needed_space {
            target_dirs.push(size)
        }
    }
    target_dirs.sort();
    println!(
        "Found deletion candidate: directory of size {}",
        target_dirs[0]
    );
    // Total:           70000000
    // Used:            43956976
    // Available:       26043024
    // Need to free:    3956976
}

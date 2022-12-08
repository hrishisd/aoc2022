use core::slice::Iter;
use scan_fmt::scan_fmt;
use std::{collections::HashMap, iter::Peekable};

#[derive(Debug)]
struct Dir {
    files: HashMap<String, usize>,
    subdirs: HashMap<String, Dir>,
}

fn main() {
    let log: Vec<&str> = include_str!("../../input").lines().collect();
    let mut sentinel = Dir::new();
    visit(&mut sentinel, &mut log.iter().peekable());
    let root = sentinel.subdirs.get("/").unwrap();
    let dir_sizes = compute_dir_sizes(root);
    println!("part 1: {}", part1(&dir_sizes));
    println!("part 2: {}", part2(&dir_sizes));
}

fn part1(dir_sizes: &[usize]) -> usize {
    dir_sizes
        .iter()
        .filter(|&size| *size <= 100000)
        .sum::<usize>()
}

fn part2(dir_sizes: &[usize]) -> usize {
    let curr_total_space = dir_sizes.iter().max().unwrap();
    let max_usable_space = 40000000usize;
    let need_to_delete = curr_total_space - max_usable_space;
    *dir_sizes
        .iter()
        .filter(|&size| *size >= need_to_delete)
        .min()
        .unwrap()
}

fn compute_dir_sizes(root: &Dir) -> Vec<usize> {
    let mut result = Vec::new();
    compute_dir_sizes_rec(root, &mut result);
    result
}

fn compute_dir_sizes_rec(curr_dir: &Dir, acc: &mut Vec<usize>) -> usize {
    let size = curr_dir
        .subdirs
        .values()
        .map(|child| compute_dir_sizes_rec(child, acc))
        .sum::<usize>()
        + curr_dir.files.values().sum::<usize>();
    acc.push(size);
    size
}

fn visit(curr_dir: &mut Dir, log: &mut Peekable<Iter<&str>>) {
    while let Some(&line) = log.next() {
        if line == "$ cd .." {
            return;
        } else if line.starts_with("$ cd ") {
            let dir_name = line.strip_prefix("$ cd ").unwrap();
            let child = curr_dir
                .subdirs
                .entry(dir_name.to_owned())
                .or_insert_with(Dir::new);
            visit(child, log);
        } else if line == "$ ls" {
            while log
                .peek()
                .map(|line| !line.starts_with('$'))
                .unwrap_or(false)
            {
                let line = log.next().expect("log can't be empty");
                if line.starts_with("dir") {
                    let dir_name = line.strip_prefix("dir ").unwrap();
                    curr_dir.subdirs.insert(dir_name.to_owned(), Dir::new());
                } else {
                    let (size, file_name) = scan_fmt!(line, "{} {}", usize, String).unwrap();
                    curr_dir.files.insert(file_name, size);
                }
            }
        }
    }
}

impl Dir {
    fn new() -> Self {
        Dir {
            files: HashMap::new(),
            subdirs: HashMap::new(),
        }
    }
}

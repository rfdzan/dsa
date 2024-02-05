
use std::collections::{VecDeque, HashMap};
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};
pub fn bfs_main() {
    let start = PathBuf::from("/home/user/Documents/resume/programmer");
    let _ = bfs(start);
}
fn bfs(start: PathBuf) {
    let mut visited_vertices = HashMap::with_capacity(100);
    let mut deque = VecDeque::with_capacity(100);
    visited_vertices.insert(start.clone(), true);
    deque.push_back(start);

    while deque.len() > 0 {
        let current_node = deque.pop_front();
        match current_node {
            None => (),
            Some(path) => {
                println!("{:?}", &path);
                visited_vertices.insert(path.clone(), true);
                if path.is_dir() {
                    let nodes = std::fs::read_dir(path).unwrap();
                    for node in nodes {
                        let node_pathbuf = node.unwrap().path();
                        if let Some(true) = visited_vertices.get(&node_pathbuf) {
                            continue
                        }
                        deque.push_back(node_pathbuf);
                    }
                }
            }
        }
    }
}
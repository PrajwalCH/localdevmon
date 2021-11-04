use std::io;
use std::path::{Path, PathBuf};

pub type DirNode = Node;

#[derive(Debug)]
pub struct Node {
    path: PathBuf,
    files: Option<Vec<String>>,
    dirs: Option<Vec<Node>>,
}

trait HiddenChecker {
    fn is_hidden(&self) -> bool;
}

impl HiddenChecker for PathBuf {
    fn is_hidden(&self) -> bool {
        self.file_name()
            .and_then(|base_name| base_name.to_str())
            .map_or_else(|| false, |base_name| base_name.starts_with('.'))
    }
}

impl Node {
    pub(super) fn new<T: AsRef<Path>>(path: T) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            files: None,
            dirs: None,
        }
    }

    pub(super) fn add_file(&mut self, path: PathBuf) {
        let filename = path.file_name();

        if let Some(filename) = filename {
            let filename = filename.to_string_lossy().into_owned();

            if let Some(files) = &mut self.files {
                files.push(filename);
            } else {
                self.files = Some(vec![filename]);
            }
        }
    }

    pub(super) fn add_dir(&mut self, child_node: Node) {
        if let Some(dirs) = &mut self.dirs {
            dirs.push(child_node);
        } else {
            self.dirs = Some(vec![child_node]);
        }
    }

    pub fn make_pathname_using_uri(&self, request_uri: &str) -> PathBuf {
        if request_uri == "/" {
            return self.path.clone();
        }
        let pathname = self.path.as_path();
        let pathname = pathname.join(&request_uri[1..]);

        pathname
    }
}

pub fn gen_dir_tree<T: AsRef<Path>>(path: T) -> io::Result<DirNode> {
    let path = path.as_ref();

    let mut root_node = Node::new(&path);

    path.read_dir().and_then(|entries| {
        for entry in entries {
            if let Ok(entry) = entry {
                let child_path = entry.path();

                if child_path.is_hidden() {
                    continue;
                }

                if child_path.is_file() {
                    root_node.add_file(child_path);
                    continue;
                }

                if child_path.is_dir() {
                    let child_node = gen_dir_tree(&child_path)?;

                    root_node.add_dir(child_node);
                }
            }
        }

        Ok(root_node)
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dir_tree_generator() {
        use super::*;

        let current_dir = "./";
        let dirnode = gen_dir_tree(current_dir).unwrap();

        assert_eq!(4, dirnode.files.unwrap().len());
    }
}

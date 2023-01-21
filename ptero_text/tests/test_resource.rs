use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use ptero_text::line_separator::DEFAULT_LINE_SEPARATOR;

pub struct ResourceLoader {
    root_path: PathBuf,
}

impl ResourceLoader {
    pub fn new(root: &Path) -> Self {
        let pwd = env::current_dir().unwrap();

        ResourceLoader {
            root_path: pwd.join(root),
        }
    }

    pub fn load_resource(self, path: &Path) -> String {
        let res_path = self.root_path.join(path);

        let string = read_to_string(res_path).unwrap();
        string.replace("\r\n", DEFAULT_LINE_SEPARATOR.into())
    }
}

pub fn split_path(path: &str) -> Vec<String> {
    let paths = path.split("/").map(|s| s.to_string()).collect();
    return paths;
}

pub fn read_file(path: &str) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

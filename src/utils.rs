pub fn split_path(path: &str) -> Vec<String> {
    let paths = path.split("/").map(|s| s.to_string()).collect();
    return paths;
}

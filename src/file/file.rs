use std::path::Path;

pub fn read_string(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
use std::fs;
pub fn get_fallback() -> String {
    if let Ok(file) = fs::read_to_string("./readme.md") {
        return file;
    }
    if let Ok(file) = fs::read_to_string("./readme.MD") {
        return file;
    }
    if let Ok(file) = fs::read_to_string("./README.MD") {
        return file;
    }
    if let Ok(file) = fs::read_to_string("./README.md") {
        return file;
    }

    "".to_string()
}

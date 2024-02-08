use std::{
    fs,
    process::Command,
    str,
};

const NAME_VARIATIONS: [&str; 4] = ["README.md", "readme.md", "readme.MD", "README.MD"];

pub fn get_readme_file() -> String {
    if let Ok(output) = Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()
    {
        if output.status.success() {
            if let Ok(root) = str::from_utf8(&output.stdout) {
                for name in NAME_VARIATIONS {
                    if let Ok(file) = fs::read_to_string(format!("{}/{}", root.trim(), name)) {
                        return file;
                    }
                }
            }
        }
    }

    // Fallback to local folder if no git
    for name in NAME_VARIATIONS {
        if let Ok(file) = fs::read_to_string(format!("./{}", name)) {
            return file;
        }
    }

    "".to_string()
}

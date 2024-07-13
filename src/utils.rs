use regex::Regex;
use std::{
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
};

pub struct Prompt {
    pub name: String,
    pub content: String,
    pub line_number: u32,
}

pub fn get_prompt_content(file_content: String) -> Vec<Prompt> {
    let re: Regex = Regex::new(
        r#"(?m)^\s*(\w*prompt\w*)\s*=\s*(?:[^\S\n]*[^\n]*\s*)*(?:'{3}|"{3})(.*?)(?:'{3}|"{3})"#,
    )
    .unwrap();
    let mut prompts: Vec<Prompt> = Vec::new();

    for cap in re.captures_iter(&file_content) {
        if let Some(s) = cap.get(2) {
            let prompt_name = cap.get(1).unwrap().as_str().to_string();
            let content = s.as_str().to_string();
            prompts.push(Prompt {
                name: prompt_name,
                content: content,
                line_number: 1,
            });
        }
    }

    prompts
}

pub fn get_file_content(file_path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_all_python_file_paths_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    if !dir.is_dir() {
        eprintln!("Path: {:?} is not a directory.", dir);
        return file_paths;
    }

    for file in fs::read_dir(&dir).unwrap() {
        let dir_entry: fs::DirEntry = file.unwrap();
        let file_type: Result<fs::FileType, std::io::Error> = dir_entry.file_type();

        match file_type {
            Ok(f) => {
                if f.is_dir() {
                    file_paths.extend(get_all_python_file_paths_in_dir(dir_entry.path().as_path()))
                }
            }
            Err(_e) => (),
        }

        match dir_entry.path().extension() {
            Some(f) => {
                if f == "py" {
                    file_paths.push(dir_entry.path())
                }
            }
            None => (),
        }
    }
    file_paths
}

pub fn is_url(s: &str) -> bool {
    if s.starts_with("http:") || s.starts_with("https://") {
        return true;
    }
    return false;
}

pub fn is_email(s: &str) -> bool {
    let email_regex: Regex =
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(s)
}

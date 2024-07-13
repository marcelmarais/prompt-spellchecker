mod display;
mod spellchecker;
mod utils;

use spellchecker::SpellChecker;
use std::env;
use std::{
    error::Error,
    path::{Path, PathBuf},
};
use utils::{get_all_python_file_paths_in_dir, get_file_content, get_prompt_content, Prompt};

fn spellcheck_file(file: PathBuf, spellchecker: &SpellChecker) {
    let contents: String = get_file_content(&file).expect("Error reading file");
    let prompts: Vec<Prompt> = get_prompt_content(contents);

    for prompt in prompts {
        let words: Vec<&str> = prompt.content.split_whitespace().collect();

        for (cnt, word) in words.iter().enumerate() {
            let result = spellchecker.check(word);

            if !result {
                let mut start_col: usize = words[..cnt].iter().map(|w| w.len() + 1).sum();
                let end_col: usize = start_col + word.chars().count() + 1;
                start_col += 1;
                println!("Word misspelt: {}", word);
                let snippet: display::LintSnippet = display::LintSnippet {
                    file_name: file
                        .file_name()
                        .expect("Error getting file name")
                        .to_str()
                        .expect("Error converting file name to string")
                        .to_owned(),
                    variable_name: prompt.name.clone(),
                    line_number: 10,
                    code: prompt.content.clone(),
                    start_col,
                    end_col,
                };
                display::print_error_with_underline("Misspelt word", &snippet)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut dir_path: &str = &String::from(".");

    if args.len() > 1 {
        dir_path = &args[1];
    }

    let python_file_paths: Vec<PathBuf> = get_all_python_file_paths_in_dir(Path::new(dir_path));
    let spellchecker = SpellChecker::new();

    for file in python_file_paths {
        spellcheck_file(file, &spellchecker)
    }
    Ok(())
}

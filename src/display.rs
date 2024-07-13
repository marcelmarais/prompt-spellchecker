use colored::Colorize;

pub struct LintSnippet {
    pub file_name: String,
    pub variable_name: String,
    pub line_number: usize,
    pub code: String,
    pub start_col: usize,
    pub end_col: usize,
}

pub fn print_error_with_underline(message: &str, snippet: &LintSnippet) {
    println!(
        "{}: {}::{}",
        "error".red().bold(),
        snippet.file_name.bold(),
        snippet.variable_name.dimmed()
    );
    println!("{:>4} | {}", snippet.line_number, snippet.code);

    let mut underline = " ".repeat(snippet.start_col + 6);
    underline.push_str(&"^".repeat(snippet.end_col - snippet.start_col));

    println!("{} {}", underline.red(), message.red());
}

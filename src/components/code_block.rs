pub fn code_block(language: impl std::fmt::Display, code: impl std::fmt::Display) {
    use colored::Colorize;

    let sep1 = "┌―"
        .to_string()
        .yellow()
        .bold();
    
    let sep2 = "└―"
        .to_string()
        .yellow()
        .bold();

    let language = language
        .to_string()
        .replace("\n", " ");

    let code_newline = "\n ┆ "
        .yellow()
        .bold()
        .to_string();
    
    let code = code
        .to_string()
        .replace("\n", code_newline.as_str());
    
    
    println!(" {} {}{}{}\n {}", sep1, language, code_newline, code, sep2);
}

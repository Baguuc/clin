pub fn header(text: impl std::fmt::Display) {
    use colored::Colorize;
    
    let text = text
        .to_string()
        .replace("\n", " ")
        .bold();

    println!(" {} {}", "―――".yellow().bold(), text);
}

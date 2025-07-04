pub fn error(reason: impl std::fmt::Display, details: impl std::fmt::Display) {
    use colored::Colorize;

    println!(" {} {}:\n{}", "error".red().bold(), reason.to_string().bold(), details);
}

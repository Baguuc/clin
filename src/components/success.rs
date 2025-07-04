pub fn success(text: impl std::fmt::Display) {
    use colored::Colorize;

    println!(" {} {}", "ok".green().bold(), text.to_string().replace("\n", "\n "));
}

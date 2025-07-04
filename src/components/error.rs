pub fn error(reason: impl std::fmt::Display, details: impl std::fmt::Display) {
    use colored::Colorize;
    
    let reason = reason
        .to_string()
        .bold()
        .replace("\n", "\n ");

    let details = details
        .to_string()
        .replace("\n", "\n ");
    
    println!(
        " {} {}:\n {}",
        "error".red().bold(),
        reason,
        details
    );
}

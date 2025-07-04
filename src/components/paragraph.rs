pub fn paragraph(text: impl std::fmt::Display, bold: bool, underline: bool) {
    use colored::Colorize;
    
    let mut text = text.to_string();
    
    if bold { text = text.bold().to_string(); };
    if underline { text = text.underline().to_string(); };

    println!(" {}", text.replace("\n", "\n "));
}

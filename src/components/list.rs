pub fn list(items: Vec<impl std::fmt::Display>) {
    use colored::Colorize;

    for item in items {
        let item = item
            .to_string()
            .replace("\n", " ");

        println!(" {} {}", "+".bold().yellow(), item);
    }
}

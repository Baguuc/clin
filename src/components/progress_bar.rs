pub fn progress_bar(max: usize, now: usize) {
    use std::io::{self, Write};
    use colored::Colorize;
    
    print!("\r");

    for _ in 0..max-1 {
        print!("{}", "―".red());
    }

    print!("\r"); 

    for _ in 0..now {
        print!("{}", "█".green());
    }

    io::stdout().flush();
}

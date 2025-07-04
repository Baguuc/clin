pub fn progress_bar(max: usize, now: usize) {
    use std::io::{self, Write};
    use colored::Colorize;

    let base = 60f32;
    
    let now = if now == 0 { 1 } else { now } as f32;
    let max = (max - 1) as f32;

    let curr_perc = now / max;
    let curr_steps = (base * curr_perc) as usize;
    
    let base = base as usize;
         
    print!("\r");
    
    for _ in 0..base {
        print!("{}", "―".red());
    }

    print!("\r"); 

    for _ in 0..curr_steps {
        print!("{}", "█".green());
    }

    let _ = io::stdout().flush();
}

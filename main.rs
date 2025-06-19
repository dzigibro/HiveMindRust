use std::io::{BufRead, BufReader, Result};
use std::process::{Child, Command, Stdio};
use rand::Rng;
use lazy_static::lazy_static;
mod spawn_journal;
use regex::Regex;
use systemd::journal::{Journal, JournalSeek, OpenOptions};

//REGEX setup once a runtime = fast! Nice! Efficent! -----Should add the fucking outside import to handle these in yaml or json instead 
lazy_static::lazy_static! {
    static ref PATTERNS: Vec<Regex> = vec![
        Regex::new(r"failed").unwrap(),
        Regex::new(r"sudo").unwrap(),
        Regex::new(r"login").unwrap(),
        Regex::new(r"root").unwrap(),
        Regex::new(r"error").unwrap(),
    ];
}








fn main() {




check_line_for_matches(line);

}








fn read_journal_loop() -> Result<(), systemd::Error> {
    // Build the reader
    let mut journal = OpenOptions::default()
        .runtime_only(false)   // include persistent boots
        .local_only(false)     // include remote hosts too
        .open()?;              // <-- this replaces your mythical `open_with()`

    // Start at the end and follow like `journalctl -f`
    journal.seek(JournalSeek::Tail)?;
    journal.watch_all_elements()?;   // arm inotify/Fd watch

    // Tail the log forever
    loop {
        if let Some(entry) = journal.next_entry()? {
            if let Some(message) = entry.get("MESSAGE") {
                check_line_for_matches(message);
            }
        }
    }
}




fn check_line_for_matches(line: &str) {
    


    for re in PATTERNS.iter() {
        
        if re.is_match(line) {
            println!("[ALERT]{}", {line});
            print_random_brackets(line);
            // break; // uncomment if you only want to alert once per line
        }
    }
}

fn print_random_brackets(text: &str) {
    let level = comfy_range(2, 15);
    for _ in 0..10 {
        println!("{}{}{}", "[".repeat(level), text, "]".repeat(level));
    }
    let open = "[".repeat(level);
    let close = "]".repeat(level);
    println!("{}{}{}", open, text, close)
}

fn comfy_range(min: usize, max: usize) -> usize {
    use rand::distributions::{Distribution, Uniform};
    let mut rng = rand::thread_rng();
    Uniform::from(min..max).sample(&mut rng)
}



    //*spawn_journal::spawnjournal();

use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use rand::Rng;

fn main() {
    // Start journalctl as a child process
    let mut child = Command::new("journalctl")
        .arg("-f")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn journalctl");

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    // Stream each line, checking for matches
    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            check_line_for_matches(&line);
        }
    }
}

fn check_line_for_matches(line: &str) {
    let patterns = vec![
        r"failed", r"sudo", r"login", r"cmd\.exe", r"powershell"
    ];

    for pat in patterns {
        let re = regex::Regex::new(pat).unwrap();
        if re.is_match(line) {
            println!("[ALERT] {line}");
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

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};



pub fn spawnjournal(){// Start journalctl as a child process
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
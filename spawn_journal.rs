use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

use crate::check_line_for_matches;


pub fn spawnjournal(){
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




/*  Start journalctl as a child process

fn spawnjournal2<F: Fn(&str)>(callback: F){

let mut child = Command::new("journalctl")

.arg("-f")

.stdout(Stdio::piped())

.spawn()

.expect("Cant Spawn journal bro check commands?");

let stdoout = child.stdout.take().unwrap();
let reader = BufReader::new(stdoout);

for line_result in reader.lines(){
    if let Ok(line)=line_result
{
    callback(&line);


}
}


}

*/
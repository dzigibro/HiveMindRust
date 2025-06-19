pub fn check_line_for_matches(line: &str) {

    let patterns: Vec<&'static str> = vec![
        r"failed", r"sudo", r"login", r"cmd\.exe", r"powershell"
    ];


    for pat in patterns {

        let re: regex::Regex = regex::Regex::new(pat).unwrap();
        if re.is_match(line){
            print_random_brackets(line);
        }
    }
}
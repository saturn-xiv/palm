use std::process::Command;

use regex::Regex;

#[test]

fn printer() {
    // printer XXX is idle.  enabled since Fri 23 Jan 2026 12:35:18 AM UTC
    // let re = Regex::new(r"^printer [[:word:]] is [[:word:]].  \w+$").unwrap();
    let re =
        Regex::new(r"printer (?<name>\w+) is (?<status>\w+).  (?<detail>[[:ascii:]]+)").unwrap();
    for line in String::from_utf8(
        Command::new("sh")
            .arg("-c")
            .arg("lpstat -p -l")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .lines()
    {
        println!("{}", line);
        let caps = re.captures(line).unwrap();
        assert_eq!(caps.len(), 4);
        println!("GET: {:?}", caps);
    }
}

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

pub fn cagrep(pathfile: &str, repeat: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(file);
    let mut stringvec: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not present");
        if !line.starts_with(">") {
            stringvec.push(line)
        }
    }

    let mut vecstring: Vec<String> = Vec::new();

    for i in stringvec.iter() {
        let sequence = i
            .as_bytes()
            .windows(repeat.parse::<usize>().unwrap())
            .map(|x| str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();

        for i in sequence.iter() {
            vecstring.push(i.to_string())
        }
    }

    let mut groups: HashMap<&str, Vec<&str>> = HashMap::new();

    for item in &vecstring {
        groups.entry(item).or_default().push(item);
    }

    let mut filewrite = File::create("path.txt").expect("file not found");
    for i in groups.iter() {
        writeln!(filewrite, "{}\t{:?}\t{}", i.0, i.1, i.1.len()).expect("line not found");
    }

    Ok("table has been prepared".to_string())
}

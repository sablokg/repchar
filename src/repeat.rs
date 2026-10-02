use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

/// uses a window approach for the detection of the genotyping
pub fn repchar(pathfile: &str, repeat: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(pathfile).expect("file not found");
    let fileread = BufReader::new(file);

    let mut hashmap_repeat: Vec<String> = Vec::new();

    let mut sequence: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not found");
        if !line.starts_with(">") {
            sequence.push(line);
        }
    }

    for i in sequence.iter() {
        let sequence = i
            .as_bytes()
            .windows(repeat.parse::<usize>().unwrap())
            .map(|x| str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        for i in sequence.iter() {
            hashmap_repeat.push(i.to_string());
        }
    }

    let mut counts = HashMap::new();
    for stringsearch in hashmap_repeat {
        *counts.entry(stringsearch).or_insert(0) += 1;
    }

    let finalcounts: Vec<(String, usize)> = counts.into_iter().collect();

    let mut countwrite = File::create("counts.txt").expect("file not found");
    for i in finalcounts.iter() {
        writeln!(countwrite, "{}\t{}\n", i.0, i.1).expect("file not found");
    }

    Ok("repchar has been finished".to_string())
}

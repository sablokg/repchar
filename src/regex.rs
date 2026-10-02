use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

pub fn analyze(pathfile: &str, stringtype: &str, count: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(pathfile).expect("file not found");
    let fileopen = BufReader::new(file);

    let mut genomestring: Vec<String> = Vec::new();
    let mut stringvec: Vec<(String, usize, usize)> = Vec::new();

    let pattern = format!("{}{{{}}}", regex::escape(stringtype), count);
    let re = Regex::new(&pattern).unwrap();

    for i in fileopen.lines() {
        let line = i.expect("line not present");
        if line.starts_with(">") {
            continue;
        }

        for m in re.find_iter(&line) {
            stringvec.push((m.as_str().to_string(), m.start(), m.end()));
        }

        genomestring.push(line);
    }

    let mut filewrite = File::create("pathrep.txt").expect("file not found");
    for i in stringvec.iter() {
        writeln!(filewrite, "{}\t{}\t{}\n", i.0, i.1, i.2).expect("line not found");
    }

    Ok("analyze has finished".to_string())
}

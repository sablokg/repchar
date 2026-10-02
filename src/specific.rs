use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/*
Gaurav Sablok
gsablok@proton.me
*/

/// uses a window approach for the detection of the genotyping
pub fn specific(pathfile: &str, specific: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(pathfile).expect("file not found");
    let fileread = BufReader::new(file);

    let mut sequence: Vec<String> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not found");
        if !line.starts_with(">") {
            sequence.push(line)
        }
    }

    let mut count: Vec<String> = Vec::new();

    for i in sequence.iter() {
        let stringsearch = i
            .as_bytes()
            .windows(specific.len())
            .map(|x| str::from_utf8(x).unwrap())
            .collect::<Vec<_>>();
        for var in stringsearch.iter() {
            count.push(var.to_string())
        }
    }

    let total_matches = count.iter().filter(|window| window.as_str() == specific).count();

    println!(
        "The count of the specific genotyping {} is {}",
        specific, total_matches
    );
    Ok("specific has been written".to_string())
}

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct CodonTable {
    pub codon_table: HashMap<&'static str, &'static str>,
}

impl CodonTable {
    pub fn new() -> CodonTable {
        let mut ct = CodonTable { codon_table: HashMap::new() };
        ct.init();
        ct
    }

    fn init(&mut self) {
        self.codon_table.insert("UUU", "F");
        self.codon_table.insert("CUU", "L");
        self.codon_table.insert("AUU", "I");
        self.codon_table.insert("GUU", "V");
        self.codon_table.insert("UUC", "F");
        self.codon_table.insert("CUC", "L");
        self.codon_table.insert("AUC", "I");
        self.codon_table.insert("GUC", "V");
        self.codon_table.insert("UUA", "L");
        self.codon_table.insert("CUA", "L");
        self.codon_table.insert("AUA", "I");
        self.codon_table.insert("GUA", "V");
        self.codon_table.insert("UUG", "L");
        self.codon_table.insert("CUG", "L");
        self.codon_table.insert("AUG", "M");
        self.codon_table.insert("GUG", "V");
        self.codon_table.insert("UCU", "S");
        self.codon_table.insert("CCU", "P");
        self.codon_table.insert("ACU", "T");
        self.codon_table.insert("GCU", "A");
        self.codon_table.insert("UCC", "S");
        self.codon_table.insert("CCC", "P");
        self.codon_table.insert("ACC", "T");
        self.codon_table.insert("GCC", "A");
        self.codon_table.insert("UCA", "S");
        self.codon_table.insert("CCA", "P");
        self.codon_table.insert("ACA", "T");
        self.codon_table.insert("GCA", "A");
        self.codon_table.insert("UCG", "S");
        self.codon_table.insert("CCG", "P");
        self.codon_table.insert("ACG", "T");
        self.codon_table.insert("GCG", "A");
        self.codon_table.insert("UAU", "Y");
        self.codon_table.insert("CAU", "H");
        self.codon_table.insert("AAU", "N");
        self.codon_table.insert("GAU", "D");
        self.codon_table.insert("UAC", "Y");
        self.codon_table.insert("CAC", "H");
        self.codon_table.insert("AAC", "N");
        self.codon_table.insert("GAC", "D");
        self.codon_table.insert("UAA", "Stop");
        self.codon_table.insert("CAA", "Q");
        self.codon_table.insert("AAA", "K");
        self.codon_table.insert("GAA", "E");
        self.codon_table.insert("UAG", "Stop");
        self.codon_table.insert("CAG", "Q");
        self.codon_table.insert("AAG", "K");
        self.codon_table.insert("GAG", "E");
        self.codon_table.insert("UGU", "C");
        self.codon_table.insert("CGU", "R");
        self.codon_table.insert("AGU", "S");
        self.codon_table.insert("GGU", "G");
        self.codon_table.insert("UGC", "C");
        self.codon_table.insert("CGC", "R");
        self.codon_table.insert("AGC", "S");
        self.codon_table.insert("GGC", "G");
        self.codon_table.insert("UGA", "Stop");
        self.codon_table.insert("CGA", "R");
        self.codon_table.insert("AGA", "R");
        self.codon_table.insert("GGA", "G");
        self.codon_table.insert("UGG", "W");
        self.codon_table.insert("CGG", "R");
        self.codon_table.insert("AGG", "R");
        self.codon_table.insert("GGG", "G");
    }
}

pub fn read_file_into_string(file_name: &'static str, file_content: &mut String) {
    let path = Path::new(file_name);
    let display = path.display();


    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {} : {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    match file.read_to_string(file_content) {
        Err(why) => panic!("Could not read {} : {}", display, Error::description(&why)),
        Ok(_) => {}//println!("File contains {}",file_content),
    };
}

pub fn read_fasta(file_name: &'static str) -> Vec<String> {
    let mut s = String::new();
    read_file_into_string(file_name, &mut s);
    let raw_lines: Vec<&str> = s.split("\r\n").collect();
    //  println!("lines {:?}", raw_lines);
    // lines.retain(|&x| !x.starts_with(">") && x.len() > 0 );
    let mut lines: Vec<String> = Vec::new();
    // println!("lines {:?}", lines);
    let mut accu_str: String = String::new();
    for temp in &raw_lines {
        if !(*temp).starts_with(">") {
            accu_str.push_str(*temp);
        } else {
            if !accu_str.is_empty() {
                lines.push(accu_str);
                accu_str = String::new();
            }
        }
    }

    if !accu_str.is_empty() {
        lines.push(accu_str);
    }

    lines
}

pub fn complement(dna_string: &str) -> String {
    dna_string.chars()
              .map(|base| {
                  match base {
                      'A' => 'T',
                      'T' => 'A',
                      'G' => 'C',
                      'C' => 'G',
                      _ => unreachable!(),
                  }
              })
              .collect::<String>()
}

pub fn reverse_complement(dna_string: &str) -> String {
    let reversed = dna_string.chars().rev().collect::<String>();
    complement(&reversed)
}

pub fn transcibe(dna_string: String) -> String {
    dna_string.replace("T", "U")
}

pub fn is_start_codon(rna_codon: &str) -> bool {
    match rna_codon {
        "AUG" => true,
        _ => false,
    }
}

pub fn is_stop_codon(rna_codon: &str) -> bool {
    match rna_codon {
        "UAG" | "UGA" | "UAA" => true,
        _ => false,
    }
}

mod test {
    #[test]
    fn test_start_codon() {
        assert_eq!(super::is_start_codon("AUG"), true);
        assert_eq!(super::is_start_codon("AGG"), false)
    }

    #[test]
    fn test_stop_codon() {
        assert_eq!(super::is_stop_codon("UAG"), true);
        assert_eq!(super::is_stop_codon("UGA"), true);
        assert_eq!(super::is_stop_codon("UAA"), true);
        assert_eq!(super::is_stop_codon("AUG"), false)
    }

    #[test]
    fn test_transcribe() {
        assert_eq!(super::transcibe("ATG".to_owned()), "AUG")
    }
}

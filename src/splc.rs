mod ProteinUtilities;

fn main() {
    let ct = ProteinUtilities::CodonTable::new();
    let mut file_contents = String::new();
    ProteinUtilities::read_file_into_string("splc.txt", &mut file_contents);

    let mut raw_lines: Vec<&str> = file_contents.split("\r\n").collect();
    let mut lines: Vec<String> = Vec::new();
    let mut accu_str: String = String::new();

    // read multiple line dna into single string
    for temp in raw_lines.iter() {
        if !(*temp).starts_with(">") && !(*temp).is_empty() {
            accu_str.push_str(*temp);
        } else {
            if accu_str.len() != 0 {
                lines.push(accu_str);
                accu_str = String::new();
            }
        }
    }

    // extraxt introns
    for index in 1..lines.len() {
        lines[0] = lines[0].replace(&lines[index], "");
    }

    // translate from dna to rna
    lines[0] = lines[0].replace("T", "U");

    // extract codons and look their aa counter parts from map
    let mut protein_string: String = String::new();
    for index in 0..(lines[0].len() / 3) {
        let codon: &str = &lines[0][index * 3..index * 3 + 3];
        if ct.codon_table.get(codon).unwrap() != &"Stop" {
            protein_string.push_str(ct.codon_table.get(codon).unwrap());
        }
    }

    println!("{}", protein_string);
}

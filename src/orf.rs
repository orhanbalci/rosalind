mod ProteinUtilities;

fn main() {
    let dnas = ProteinUtilities::read_fasta(".\\src\\orf.txt");
    let rna = ProteinUtilities::transcibe(dnas[0].clone());
    let ct = ProteinUtilities::CodonTable::new();
    let mut result_string = String::new();

    let mut result = find_open_reading_frames(rna.clone(), 0, &ct);
    print_vector(&result, &mut result_string);
    result = find_open_reading_frames(rna.clone(), 1, &ct);
    print_vector(&result, &mut result_string);
    result = find_open_reading_frames(rna.clone(), 2, &ct);
    print_vector(&result, &mut result_string);

    let complement_rna =
        ProteinUtilities::transcibe(ProteinUtilities::reverse_complement(&dnas[0].clone()));
    result = find_open_reading_frames(complement_rna.clone(), 0, &ct);
    print_vector(&result, &mut result_string);
    result = find_open_reading_frames(complement_rna.clone(), 1, &ct);
    print_vector(&result, &mut result_string);
    result = find_open_reading_frames(complement_rna.clone(), 2, &ct);
    print_vector(&result, &mut result_string);
    ProteinUtilities::write_string_to_file("results.txt", &result_string);
}

fn print_vector(result: &Vec<String>, result_string: &mut String) {
    for s in result {
        result_string.push_str("\r\n");
        result_string.push_str(s);
    }
}

fn find_open_reading_frames(rna: String,
                            skip_char_count: usize,
                            ct: &ProteinUtilities::CodonTable)
                            -> Vec<String> {

    let mut protein_sequence = String::new();
    let mut results: Vec<String> = Vec::new();
    // find start codon indexes
    let mut indexes = vec![];
    for chunk in rna.clone().into_bytes()[skip_char_count..].chunks(3).enumerate() {
        if ProteinUtilities::is_start_codon(String::from_utf8_lossy(chunk.1).as_ref()) {
            indexes.push(chunk.0);
        }
    }

    for index in indexes {
        for chunk in rna.clone().into_bytes()[skip_char_count..].chunks(3).skip(index) {
            if ProteinUtilities::is_stop_codon(String::from_utf8_lossy(chunk).as_ref()) {
                if !protein_sequence.is_empty() {
                    results.push(protein_sequence.clone());
                    // println!("{:?}", protein_sequence);
                    protein_sequence.clear();
                    break;
                }
            } else {
                if chunk.len() == 3 {
                    protein_sequence.push_str(ct.codon_table
                                                .get(String::from_utf8_lossy(chunk).as_ref())
                                                .unwrap());
                }
            }
        }
    }

    results
}

mod protein_utilities;

fn main() {
    let dna = protein_utilities::read_fasta("revp.txt");

    for j in 4..13 {
        for i in 0..dna[0].len() - j + 1 {
            if dna[0][i..i + j] == protein_utilities::reverse_complement(&dna[0][i..i + j]) {
                println!("{} {}", i + 1, j);
            }
        }
    }
}

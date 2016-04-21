mod ProteinUtilities;

fn main() {
    let dna = ProteinUtilities::read_fasta("revp.txt");

    for j in 4..13 {
        for i in 0..dna[0].len() - j + 1 {
            if dna[0][i..i + j] == ProteinUtilities::reverse_complement(&dna[0][i..i + j]) {
                println!("{} {}", i+1, j);
            }
        }
    }
}

// http://rosalind.info/problems/kmer/
mod protein_utilities;

fn main() {
    let dnas = protein_utilities::read_fasta(".\\src\\kmer.txt");
    let alphabet = ['A', 'C', 'G', 'T'];
    let mut kmers: Vec<String> = Vec::new();
    permute(&alphabet, 4, &mut kmers);
    kmers.sort();
    let kmer_freq = compute_kmer_freq(&kmers, &dnas[0]);
    let result = kmer_freq.iter()
                          .fold(String::new(), |mut acc, x| {
                              acc.push_str(&x.to_string());
                              acc.push_str(" ");
                              acc
                          });
    protein_utilities::write_string_to_file(".\\src\\result.txt", &result);
}

fn compute_kmer_freq(kmer: &Vec<String>, dna: &String) -> Vec<u32> {
    let mut result: Vec<u32> = vec![0; kmer.len()];
    for (index, i) in kmer.iter().enumerate() {
        for j in 0..dna.len() - 3 {
            if dna[j..j + 4] == i[..] {
                result[index] += 1;
            }
        }
    }
    result
}

fn permute(alphabet: &[char], max_level: u32, mut kmers: &mut Vec<String>) {
    permute_helper(alphabet, max_level, 0, "", &mut kmers)
}

fn permute_helper(alphabet: &[char],
                  max_level: u32,
                  level: u32,
                  prefix: &str,
                  mut kmers: &mut Vec<String>) {
    if level < max_level {
        for c in alphabet {
            let mut iner_prefix: String = String::from(prefix);
            iner_prefix.push(*c);
            permute_helper(alphabet, max_level, level + 1, &iner_prefix, &mut kmers)
        }
    } else if level == max_level {
        kmers.push(String::from(prefix));
        // println!("{}", prefix);
    }
}

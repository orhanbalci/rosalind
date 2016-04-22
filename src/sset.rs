// solution to http://rosalind.info/problems/sset

mod protein_utilities;

fn main() {
    let mut file_content = String::new();
    protein_utilities::read_file_into_string("./data/sset.txt", &mut file_content);
    let set_size = file_content.trim().parse::<u32>().unwrap();
    println!("{}", compute_subset_size(set_size, 1_000_000));
}

fn compute_subset_size(set_size: u32, modulo: u32) -> u32 {
    (1..set_size + 1).fold(1, |accum, x| 2 * accum % modulo)
}

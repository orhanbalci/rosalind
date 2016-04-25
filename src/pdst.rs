// http://rosalind.info/problems/pdst
mod protein_utilities;
use std::fmt;
fn main() {
    let dnas = protein_utilities::read_fasta("./data/pdst.txt");
    let mut matrix_1d = vec![0.0; dnas.len() * dnas.len()];
    let mut row_collection = matrix_1d.chunks_mut(dnas.len()).collect::<Vec<_>>();
    let mut matrix = row_collection.as_mut_slice();
    dnas.iter()
        .enumerate()
        .inspect(|index_first_dna| {
            dnas.iter()
                .skip(index_first_dna.0)
                .enumerate()
                .inspect(|index2_second_dna| {
                    let distance = distance(index_first_dna.1, index2_second_dna.1);
                    matrix[index_first_dna.0][index_first_dna.0 + index2_second_dna.0] = distance;
                    matrix[index_first_dna.0 + index2_second_dna.0][index_first_dna.0] = distance;
                })
                .collect::<Vec<_>>();
        })
        .collect::<Vec<_>>();

    pretty_print(matrix);
}

fn distance(first_string: &str, second_string: &str) -> f32 {
    let different_char_count = first_string.chars()
                                           .zip(second_string.chars())
                                           .filter(|chartuple| chartuple.0 != chartuple.1)
                                           .collect::<Vec<_>>()
                                           .len();

    different_char_count as f32 / first_string.len() as f32
}

fn pretty_print<T: fmt::Display>(matrix: &mut [&mut [T]]) {
    matrix.iter()
          .inspect(|row| {
              row.iter()
                 .inspect(|element| {
                     print!("{} ", element);
                 })
                 .collect::<Vec<_>>();
              println!("");
          })
          .collect::<Vec<_>>();
}

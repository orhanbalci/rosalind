// http://rosalind.info/problems/sseq

mod protein_utilities;
fn main() {
    let mut dnas = protein_utilities::read_fasta("./data/sseq.txt");
    let mut result = vec![];
    let mut result_index = 0;
    dnas[1]
        .chars()
        .inspect(|c| {
            let next_index = dnas[0].chars().skip(result_index).position(|k| k == *c).unwrap();
            result.push(result_index + next_index + 1);
            result_index += (next_index + 1);
        })
        .collect::<Vec<_>>();

    result.iter().inspect(|x| print!("{} ", x)).collect::<Vec<_>>();
}

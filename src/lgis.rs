mod protein_utilities;

fn main() {
    let mut file_content = String::new();
    protein_utilities::read_file_into_string("./data/lgis.txt", &mut file_content);
    let lines = file_content.split("\n").map(|s| s.trim()).collect::<Vec<_>>();

    let sequence = lines.iter()
                        .nth(1)
                        .unwrap()
                        .split(" ")
                        .map(|s| s.trim().parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
    longest_sequence(&sequence, |x, y| x > y).iter().inspect(|item| print!("{} ", item)).count();
    println!("");
    longest_sequence(&sequence, |x, y| x < y).iter().inspect(|item| print!("{} ", item)).count();
}


fn longest_sequence<F>(sequence: &Vec<u32>, predicate: F) -> Vec<u32>
    where F: Fn(u32, u32) -> bool
{
    let mut longest_sequence: Vec<Vec<u32>> = (0..sequence.len())
                                                  .map(|i| {
                                                      let mut a =
                                                          Vec::with_capacity(sequence.len());
                                                      a.push(sequence[i]);
                                                      a
                                                  })
                                                  .collect();
    let mut longest_sequence_counter = Vec::with_capacity(sequence.len());
    for i in 0..sequence.len() {
        longest_sequence_counter.push(1);
    }

    sequence.iter()
            .enumerate()
            .inspect(|&(index1, &elem)| {
                sequence.iter()
                        .skip(index1 + 1)
                        .enumerate()
                        .inspect(|&(index2, &elem2)| {
                            if predicate(elem2, elem) &&
                               (longest_sequence_counter[index1] + 1 >
                                longest_sequence_counter[index1 + 1 + index2]) {
                                longest_sequence_counter[index1 + 1 + index2] =
                                    longest_sequence_counter[index1] + 1;
                                longest_sequence[index1 + 1 + index2] = longest_sequence[index1]
                                                                            .clone();
                                longest_sequence[index1 + 1 + index2].push(elem2);
                            }
                        })
                        .count();
            })
            .count();

    let longest_sequence_index = longest_sequence_counter.iter()
                                                         .enumerate()
                                                         .max_by_key(|&(index, &elem)| elem)
                                                         .unwrap()
                                                         .0;
    longest_sequence[longest_sequence_index].clone()
}

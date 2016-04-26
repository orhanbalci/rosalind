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
    let mut longest_sequence = vec![];
    let mut last_longest = vec![];

    sequence.iter()
            .enumerate()
            .inspect(|&(index, &elem)| {
                for skipper in index + 1..sequence.len() {
                    if sequence.len() - skipper < last_longest.len() {
                        break;
                    }
                    let len = longest_sequence.len();
                    longest_sequence.insert(len, elem);
                    sequence.iter()
                            .skip(skipper)
                            .inspect(|&elem2| {
                                if predicate(*elem2, *longest_sequence.last().unwrap()) {
                                    let len = longest_sequence.len();
                                    longest_sequence.insert(len, *elem2);
                                }
                            })
                            .count();
                    if longest_sequence.len() > last_longest.len() {
                        last_longest = longest_sequence.clone();
                    }
                    longest_sequence.clear();
                }
            })
            .count();

    last_longest
}

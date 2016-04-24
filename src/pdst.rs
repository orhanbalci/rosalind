fn main() {

}

fn distance(first_string: &str, second_string: &str) -> f32 {
    let different_char_count = first_string.chars()
                                           .zip(second_string.chars())
                                           .filter(|chartuple| chartuple.0 != chartuple.1)
                                           .collect::<Vec<_>>()
                                           .len();

    different_char_count as f32 / first_string.len() as f32
}

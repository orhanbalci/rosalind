extern crate suffix;

use std::char;
use suffix::SuffixTable;

mod ProteinUtilities;

fn main() {
    let dnas = ProteinUtilities::read_fasta(".\\src\\lcsm.txt");
    let dna_count = dnas.len();
    let mut begin_terminal_character: u32 = 0x30;
    let all_strings: String = dnas.iter()
                                  .fold(String::new(), |mut acc, x| {
                                      if acc.len() > 0 {
                                          acc.push(char::from_u32(begin_terminal_character)
                                                       .unwrap());
                                          begin_terminal_character += 1;
                                      }
                                      acc.push_str(x);
                                      acc
                                  });
    // print!("{}", all_strings);
    let sa: SuffixTable = SuffixTable::new(all_strings.to_lowercase());
    for iter in 0..sa.len() - dna_count {
        let mut suffix_set: Vec<String> = vec![];
        for inner_iter in iter..iter + dna_count {
            suffix_set.push(sa.suffix(inner_iter).to_string());
        }
        if !is_from_unique_dnas(&dnas, &suffix_set) {
            continue;
        }
    }
    print!("{:?}", sa);
}

fn is_from_unique_dnas(dna_vector: &Vec<String>, suffix_set: &Vec<String>) -> bool {
    true
}

fn get_number_of_special_chars(dna_suffix: &str, special_char_begin: u32, dna_count: u32) -> u8 {
    let mut special_char_count = 0u8;
    dna_suffix.chars().inspect(|c| {        
        if (*c as u32) >= special_char_begin && (*c as u32) < special_char_begin + dna_count {
            special_char_count += 1;
        }
    }).collect::<String>();
    special_char_count
}

#[test]
fn test_get_number_of_special_chars() {
    assert_eq!(get_number_of_special_chars("0tagacca1ataca", 0x30u32, 3u32),
               2);
}

#[test]
fn test_char_to_u32() {
    assert_eq!('0' as u32, 0x30u32);
    assert_eq!('1' as u32, 0x31u32);
    assert_eq!('2' as u32, 0x32u32);
}

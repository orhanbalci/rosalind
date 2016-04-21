extern crate suffix;

use std::char;
use std::collections::HashSet;
use std::collections::HashMap;
use suffix::SuffixTable;

mod ProteinUtilities;

fn main() {
    let dnas = ProteinUtilities::read_fasta(".\\src\\lcsm.txt");
    let dna_count = dnas.len();
    let mut begin_terminal_character: u32 = 0x30;
    let mut suffix_special_char_count_table: HashMap<String, u32> = HashMap::new();
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
    let mut longest_substring: String = String::new();
    // print!("{:?}", sa);
    for iter in 0..sa.len() - dna_count {
        let mut suffix_set: Vec<String> = vec![];
        for inner_iter in iter..iter + dna_count {
            suffix_set.push(sa.suffix(inner_iter).to_string());
        }
        if !is_from_unique_dnas(&suffix_set,
                                0x30u32,
                                dna_count as u32,
                                &mut suffix_special_char_count_table) {
            continue;
        } else {
            let mut temp_longest_string = String::new();
            find_longest_prefix(&suffix_set, &mut temp_longest_string);
            // println!("{:?}", longest_substring);
            if temp_longest_string.len() > longest_substring.len() {
                longest_substring = temp_longest_string.clone();
                println!("{}:{}", "New long found", longest_substring);
            }
        }
    }

    println!("{} {}",
             "Longest common substring is",
             longest_substring.to_uppercase());
}

fn is_from_unique_dnas(suffix_set: &Vec<String>,
                       special_char_begin: u32,
                       dna_count: u32,
                       mut memo: &mut HashMap<String, u32>)
                       -> bool {
    let mut number_of_special_chars: HashSet<u32> = HashSet::new();
    let mut result = true;
    suffix_set.iter()
              .inspect(|suffix| {
                  let num_of_chars = get_number_of_special_chars(suffix,
                                                                 special_char_begin,
                                                                 dna_count,
                                                                 &mut memo);
                  if number_of_special_chars.contains(&num_of_chars) {
                      result = false;
                  } else {
                      number_of_special_chars.insert(num_of_chars);
                  }
              })
              .count();
    result
}

fn get_number_of_special_chars(dna_suffix: &str,
                               special_char_begin: u32,
                               dna_count: u32,
                               memo: &mut HashMap<String, u32>)
                               -> u32 {
    let mut special_char_count = 0u32;
    if memo.contains_key(dna_suffix) {
        special_char_count = *memo.get(dna_suffix).unwrap();
    } else {

        dna_suffix.chars()
                  .inspect(|c| {
                      if (*c as u32) >= special_char_begin &&
                         (*c as u32) < special_char_begin + dna_count {
                          special_char_count += 1;
                      }
                  })
                  .collect::<String>();
        memo.insert(String::from(dna_suffix), special_char_count);
    }
    special_char_count
}

fn find_longest_prefix(suffix_set: &Vec<String>, longest_substring: &mut String) {
    let mut longest_string_length = 0;

    for item in suffix_set.iter() {
        if item.len() > longest_string_length {
            longest_string_length = item.len();
        }
    }

    'outer: for i in 0..longest_string_length {
        for j in 0..suffix_set.len() - 1 {
            if suffix_set.iter().nth(j).unwrap().chars().nth(i) !=
               suffix_set.iter().nth(j + 1).unwrap().chars().nth(i) {
                break 'outer;
            }
        }
        longest_substring.push(suffix_set.iter().nth(0).unwrap().chars().nth(i).unwrap());
    }
}

#[test]
fn test_get_number_of_special_chars() {
    let mut test_map: HashMap<String, u32> = HashMap::new();
    assert_eq!(get_number_of_special_chars("0tagacca1ataca", 0x30u32, 3u32, &mut test_map),
               2);
}

#[test]
fn test_char_to_u32() {
    assert_eq!('0' as u32, 0x30u32);
    assert_eq!('1' as u32, 0x31u32);
    assert_eq!('2' as u32, 0x32u32);
}

#[test]
fn test_find_longest_prefix() {
    let mut input: Vec<String> = vec![];
    input.push(String::from("orhan"));
    input.push(String::from("orh"));
    input.push(String::from("or"));
    let mut output = String::new();
    find_longest_prefix(&input, &mut output);
    assert_eq!(output, "or");
}

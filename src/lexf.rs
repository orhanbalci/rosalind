mod ProteinUtilities;

fn main() {
    let mut file_content = String::new();
    ProteinUtilities::read_file_into_string("lexf.txt", &mut file_content);
    let parameters: Vec<&str> = file_content.split("\r\n").collect();
    let alpha: Vec<char> = parameters[0].split(" ").map(|x| x.chars().next().unwrap()).collect();
    permute(&alpha[..], parameters[1].parse::<u32>().unwrap());
}


fn permute(alphabet: &[char], max_level: u32) {
    permute_helper(alphabet, max_level, 0, "")
}

fn permute_helper(alphabet: &[char], max_level: u32, level: u32, prefix: &str) {
    if level < max_level {
        for c in alphabet {
            let mut iner_prefix: String = String::from(prefix);
            iner_prefix.push(*c);
            permute_helper(alphabet, max_level, level + 1, &iner_prefix)
        }
    } else if level == max_level {
        println!("{}", prefix);
    }
}

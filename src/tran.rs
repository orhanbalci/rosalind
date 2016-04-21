mod ProteinUtilities;

enum base_type {
    purine,
    pyrimidine,
}

enum point_mutation_type {
    transition,
    transversion,
}

fn get_base_type(base: char) -> Option<base_type> {
    match base {
        'A' | 'G' => Option::Some(base_type::purine),
        'C' | 'T' => Option::Some(base_type::pyrimidine),
        _ => Option::None,
    }
}

fn get_mutation_type(base_pair: (base_type, base_type)) -> point_mutation_type {
    match base_pair {
        (base_type::purine, base_type::purine) |
        (base_type::pyrimidine, base_type::pyrimidine) => point_mutation_type::transition,
        (base_type::purine, base_type::pyrimidine) |
        (base_type::pyrimidine, base_type::purine) => point_mutation_type::transversion,
    }
}


fn main() {
    let mut file_contents = String::new();
    ProteinUtilities::read_file_into_string("tran.txt", &mut file_contents);
    let mut raw_lines: Vec<&str> = file_contents.split("\r\n").collect();
    let mut lines: Vec<String> = Vec::new();
    let mut accu_str: String = String::new();

    // read multiple line dna into single string
    for temp in raw_lines.iter() {
        if !(*temp).starts_with(">") && !(*temp).is_empty() {
            accu_str.push_str(*temp);
        } else {
            if accu_str.len() != 0 {
                lines.push(accu_str);
                accu_str = String::new();
            }
        }
    }

    let mut mutation_count: (f32, f32) = (0.0, 0.0);

    for i in 0..lines[0].len() {
        if lines[0].chars().nth(i).unwrap() != lines[1].chars().nth(i).unwrap() {
            match get_mutation_type((get_base_type(lines[0].chars().nth(i).unwrap()).unwrap(),
                                     get_base_type(lines[1].chars().nth(i).unwrap()).unwrap())) {
                point_mutation_type::transition => mutation_count.0 += 1.0,
                point_mutation_type::transversion => mutation_count.1 += 1.0,
            }
        }
    }
    println!("Result : {:?}", mutation_count);
    println!("Result : {}", mutation_count.0 / mutation_count.1)
}

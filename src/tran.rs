mod protein_utilities;

enum BaseType {
    Purine,
    Pyrimidine,
}

enum PointMutationType {
    Transition,
    Transversion,
}

fn get_base_type(base: char) -> Option<BaseType> {
    match base {
        'A' | 'G' => Option::Some(BaseType::Purine),
        'C' | 'T' => Option::Some(BaseType::Pyrimidine),
        _ => Option::None,
    }
}

fn get_mutation_type(base_pair: (BaseType, BaseType)) -> PointMutationType {
    match base_pair {
        (BaseType::Purine, BaseType::Purine) |
        (BaseType::Pyrimidine, BaseType::Pyrimidine) => PointMutationType::Transition,
        (BaseType::Purine, BaseType::Pyrimidine) |
        (BaseType::Pyrimidine, BaseType::Purine) => PointMutationType::Transversion,
    }
}


fn main() {
    let mut file_contents = String::new();
    protein_utilities::read_file_into_string("tran.txt", &mut file_contents);
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
                PointMutationType::Transition => mutation_count.0 += 1.0,
                PointMutationType::Transversion => mutation_count.1 += 1.0,
            }
        }
    }
    println!("Result : {:?}", mutation_count);
    println!("Result : {}", mutation_count.0 / mutation_count.1)
}

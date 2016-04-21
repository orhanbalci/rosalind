mod ProteinUtilities;

fn main() {
    let mut file_content : String = String::new();
    ProteinUtilities::read_file_into_string("grph.txt",&mut file_content);
    let raw_lines : Vec<&str> = file_content.split("\r\n").collect();
    let mut lines : Vec<String> = Vec::new();
    let mut accu_str : String = String::new();

    for temp in raw_lines.iter() {
        if !(*temp).starts_with(">"){
            accu_str.push_str(*temp);
        }else{
            if accu_str.len() != 0{
                lines.push(accu_str);
                accu_str = String::new();
            }

            lines.push(String::from(*temp));
        }
    }

    if accu_str.len() != 0 {
        lines.push(accu_str);
    }

    let (keys,values) : (Vec<_>,Vec<_>) = lines.iter().
                                        map(|x| x.trim()).
                                        filter(|&x| x.len() > 0 ).
                                        partition(|&x| x.chars().next().unwrap() == '>');

    for (i, el1) in values.iter().enumerate() {
        for (i2,el2) in values.iter().skip(i+1).enumerate() {

            if el1.chars().skip(el1.len()-3).take(3).collect::<String>() == el2.chars().take(3).collect::<String>()
            {
                let first_node = keys.iter().skip(i).next().unwrap();
                let second_node = keys.iter().skip(i2+i+1).next().unwrap();
                //println!("({} {}, {} {})", i,el1, i2+i+1,el2);
                println!("{} {}", first_node.chars().skip(1).collect::<String>(), second_node.chars().skip(1).collect::<String>());
            }

            if el2.chars().skip(el2.len()-3).take(3).collect::<String>() == el1.chars().take(3).collect::<String>() {
                let second_node = keys.iter().skip(i).next().unwrap();
                let first_node = keys.iter().skip(i2+i+1).next().unwrap();
                //println!("({} {}, {} {})", i,el1, i2+i+1,el2);
                println!("{} {}", first_node.chars().skip(1).collect::<String>(), second_node.chars().skip(1).collect::<String>());
            }

        }
    }
}

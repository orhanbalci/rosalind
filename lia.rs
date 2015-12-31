//http://rosalind.info/problems/lia/
mod math_helper;
mod ProteinUtilities;

fn main() {
    let mut file_content = String::new();
    ProteinUtilities::read_file_into_string(".\\src\\lia.txt", &mut file_content);
    let k_n: Vec<&str> = file_content.trim().split(" ").collect();
    println!("{:?}",k_n);
    let k = k_n[0].parse::<u32>().unwrap();
    let n = k_n[1].parse::<u32>().unwrap();
    let population_count = 2_u32.pow(k);
    println!("{}",population_count);
    println!("{}", compute_at_least(population_count, n));
}

fn compute_at_least(population_count: u32, at_least: u32) -> f32 {
    let mut accu: f32 = 0_f32;
    for i in at_least..population_count + 1 {
        accu += compute_exactly(population_count, i);
    }
    accu
}

fn compute_exactly(population_count: u32, exactly: u32) -> f32 {
    math_helper::combinations(population_count, exactly) *
    (0.25_f32.powf(exactly as f32) * 0.75_f32.powf(population_count as f32 - exactly as f32))
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("prob.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           Error::description(&why)),
                           Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           Error::description(&why)),
                           Ok(_) => print!("{} contains:\n{}", display, s),
    }

    let vec: Vec<&str> = s.split("\n").collect();

    let mut freq_at : i32 =0;
    let mut freq_gc : i32 =0;
    for c in vec[0].chars(){
        match c {
            'A'|'T' => freq_at += 1,
            'G'|'C' => freq_gc += 1,
            _ => {}
        }
    }

    let mut vec2 : Vec<f64> = vec[1].split(" ").map(|s| s.trim()).map(|s| s.parse().unwrap()).collect();

    vec2 = vec2.into_iter().map(|gc_content| compute_log(freq_gc,freq_at,gc_content)).collect();
    for element in &vec2 {
        print!("{} ",element);
    }

}

fn compute_log(gc_freq : i32, at_freq : i32, gc_content : f64) ->f64{
    let result : f64;
    let prob_at : f64;
    let prob_gc : f64;

    prob_gc = gc_content / 2.0;
    prob_at = (1.0 - gc_content)/2.0;
    result = prob_at.powi(at_freq) * prob_gc.powi(gc_freq);
    result.log10()
}

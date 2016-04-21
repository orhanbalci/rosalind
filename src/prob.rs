
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

    let mut freqAT : i32 =0;
    let mut freqGC : i32 =0;
    for c in vec[0].chars(){
        match c {
            'A'|'T' => freqAT += 1,
            'G'|'C' => freqGC += 1,
             _ => {}
        }
    }

    let mut vec2 : Vec<f64> = vec[1].split(" ").map(|s| s.trim()).map(|s| s.parse().unwrap()).collect();

    vec2 = vec2.into_iter().map(|gcContent| computeLog(freqGC,freqAT,gcContent)).collect();
    for element in &vec2 {
        print!("{} ",element);
    }

}

fn computeLog(gcFreq : i32, atFreq : i32, gcContent : f64) ->f64{
        let mut result : f64 = 0.0;
        let mut probAT : f64 = 0.0;
        let mut probGC : f64 = 0.0;

        probGC = gcContent / 2.0;
        probAT = ((1.0 - gcContent)/2.0);
        result = probAT.powi(atFreq) * probGC.powi(gcFreq);
        result.log10()
}

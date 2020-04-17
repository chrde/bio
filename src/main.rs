use lib::{clumps, frequency_table, Kmer, Nucl};
use std::{
    fs,
    time::{Duration, Instant},
};

fn main() {
    let str = fs::read_to_string("oric_vibrio_cholerae").unwrap();
    let genome: Vec<_> = str
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| Nucl::from(c))
        .collect();

    let now = Instant::now();
    clumps(&genome, 500, 3, 9);
    let dur = now.elapsed();
    println!("{:?}", dur);
    // let freqs = frequency_table(&nucls, 9);
    // for (k, v) in freqs {
    //     if v >= 3 {
    //         print!("{},", Kmer::from_id(k, 9));
    //     }
    // }
    // println!("hello world")
}

use crate::prelude::*;

pub struct Clump {}

pub fn clumps(genome: &[Nucl], region_length: usize, threshold: usize, k: usize) -> Vec<Clump> {
    let mut result = vec![];
    let mut res = HashSet::new();
    let kmer: Kmer = "ATGATCAAG".parse().unwrap();
    let mut freqs = crate::frequency_table(&genome[0..region_length], k);
    let regions = genome.windows(region_length);
    for region in regions.skip(1) {
        let first_kmer = Kmer::new(&region[0..k]).to_id();
        let last_kmer_start = region_length - k;
        let last_kmer = Kmer::new(&region[last_kmer_start..region_length]);
        match freqs.entry(first_kmer) {
            Entry::Occupied(mut e) => {
                let freq = e.get_mut();
                if *freq == 1 {
                    e.remove();
                } else {
                    *freq -= 1;
                }
            }
            _ => {}
        };
        freqs
            .entry(last_kmer.to_id())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        let mut count = 0;
        for (k, v) in &freqs {
            if *v >= threshold {
                res.insert(Kmer::from_id(*k, 9));
            }
        }
    }
    println!("found {}", res.len());
    for r in res {
        println!("{}", r);
    }
    result
}

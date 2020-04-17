use prelude::*;

mod clump;
mod kmer;
mod nucl;
mod prelude;

pub use clump::clumps;
pub use kmer::Kmer;
pub use nucl::Nucl;

pub fn frequency_table(text: &[Nucl], k: usize) -> BTreeMap<usize, usize> {
    let mut table = BTreeMap::new();
    for w in text.windows(k) {
        let kmer = Kmer::new(w);
        table
            .entry(kmer.to_id())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    table
}

#[cfg(test)]
fn pretty_table(table: &BTreeMap<usize, usize>, k: usize) -> String {
    use prettytable::*;

    let mut result = Table::new();
    let mut titles = Row::empty();
    let mut freqs = Row::empty();
    for (t, f) in table.iter() {
        let cell = Cell::new(&Kmer::from_id(*t, k).to_string());
        titles.add_cell(cell);
        freqs.add_cell(Cell::new(&f.to_string()));
    }

    result.add_row(titles);
    result.add_row(freqs);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seq(str: &str) -> Vec<Nucl> {
        str.chars().map(|c| c.into()).collect()
    }

    #[test]
    fn test_frequency_table() {
        let text = seq("ACGTTTCACGTTTTACGG");
        let freqs = frequency_table(&text, 3);
        println!("{}", pretty_table(&freqs, 3));
        panic!();
        // assert_eq!(0, count);
    }
}

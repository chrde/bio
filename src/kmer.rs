use crate::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Ord, PartialOrd, Eq)]
pub struct Kmer {
    inner: Vec<Nucl>,
}

impl Kmer {
    pub fn new(nucls: &[Nucl]) -> Self {
        Self {
            inner: nucls.to_vec(),
        }
    }

    pub fn reverse_complement(&self) -> Self {
        let inner = self.inner.iter().rev().map(|k| k.complement()).collect();
        Self { inner }
    }

    pub fn from_id(mut id: usize, length: usize) -> Self {
        let mut inner = vec![];
        while id > 0 {
            let rem = id % 4;
            inner.push(rem.into());
            id = id / 4;
        }
        while inner.len() < length {
            inner.push(A);
        }
        inner.reverse();
        Self { inner }
    }

    pub fn to_id(&self) -> usize {
        let mut res = 0;
        for s in self.inner.iter().cloned() {
            res = 4 * res + (s as usize);
        }
        res
    }
}

impl Hash for Kmer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let self_id = self.to_id();
        let reverse_id = self.reverse_complement().to_id();
        let min = std::cmp::min(self_id, reverse_id);
        min.hash(state);
    }
}

impl PartialEq for Kmer {
    fn eq(&self, other: &Kmer) -> bool {
        let self_id = self.to_id();
        self_id == other.to_id() || self_id == other.reverse_complement().to_id()
    }
}

impl str::FromStr for Kmer {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s.chars().map(|c| Nucl::from(c)).collect();
        Ok(Self { inner })
    }
}

impl fmt::Display for Kmer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in &self.inner {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn parse_kmer(seqs: &str) -> Kmer {
        let inner = seqs.chars().map(|c| c.into()).collect();
        Kmer { inner }
    }

    #[test_case(0, 2, "AA")]
    #[test_case(4, 2, "CA")]
    #[test_case(11, 2, "GT")]
    #[test_case(15, 2, "TT")]
    #[test_case(1, 3, "AAC")]
    #[test_case(61166, 9, "AAC")]
    fn from_id(id: usize, length: usize, expected: &str) {
        let kmer = Kmer::from_id(id, length);
        assert_eq!(expected, kmer.to_string())
    }

    #[test_case(0, "AA")]
    #[test_case(1, "C")]
    #[test_case(1, "AC")]
    #[test_case(4, "CA")]
    #[test_case(11, "GT")]
    #[test_case(15, "TT")]
    #[test_case(1, "AAC")]
    fn to_id(expected: usize, seqs: &str) {
        let kmer = parse_kmer(seqs);
        assert_eq!(expected, kmer.to_id())
    }
}

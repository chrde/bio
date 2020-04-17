use crate::prelude::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Nucl {
    A,
    T,
    C,
    G,
}

impl Nucl {
    pub fn complement(&self) -> Self {
        use Nucl::*;
        match self {
            A => T,
            T => A,
            C => G,
            G => C,
        }
    }
}

impl fmt::Display for Nucl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<char> for Nucl {
    fn from(c: char) -> Self {
        use Nucl::*;
        match c {
            'A' => A,
            'T' => T,
            'C' => C,
            'G' => G,
            o => panic!("invalid nucleobase {}", o),
        }
    }
}

impl From<Nucl> for char {
    fn from(c: Nucl) -> Self {
        match c {
            A => 'A',
            T => 'T',
            C => 'C',
            G => 'G',
        }
    }
}

impl From<usize> for Nucl {
    fn from(c: usize) -> Self {
        match c {
            0 => A,
            1 => T,
            2 => C,
            3 => G,
            o => panic!("invalid nucleobase {}", o),
        }
    }
}

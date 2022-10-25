use bitvec::prelude::*;
use nom::{character::complete::one_of, combinator::*, error::context, multi::*, IResult};

/// # Nucleotide
/// A nucleotide of a gene.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Nucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
}

impl Nucleotide {
    /// Convert a nucleotide to a bitvector.
    pub fn to_bits(&self) -> BitVec {
        match &self {
            Nucleotide::Adenine => bitvec![0, 0],
            Nucleotide::Cytosine => bitvec![0, 1],
            Nucleotide::Guanine => bitvec![1, 0],
            Nucleotide::Thymine => bitvec![1, 1],
        }
    }

    /// Convert a nucleotide to a character.
    pub fn to_letter(&self) -> char {
        match &self {
            Nucleotide::Adenine => 'A',
            Nucleotide::Cytosine => 'C',
            Nucleotide::Guanine => 'G',
            Nucleotide::Thymine => 'T',
        }
    }

    // Convert a bitvector to a nucleotide.
    // Returns a `None` if the bitvector does not match.
    pub fn from_bits(bv: BitVec) -> Option<Nucleotide> {
        if bv == bits![0, 0] {
            Some(Nucleotide::Adenine)
        } else if bv == bits![0, 1] {
            Some(Nucleotide::Cytosine)
        } else if bv == bits![1, 0] {
            Some(Nucleotide::Guanine)
        } else if bv == bits![1, 1] {
            Some(Nucleotide::Thymine)
        } else {
            None
        }
    }

    // Convert a character to a nucleotide.
    // Returns a `None` if the character does not match.
    pub fn from_letter(c: char) -> Option<Nucleotide> {
        match c {
            'A' | 'a' => Some(Nucleotide::Adenine),
            'C' | 'c' => Some(Nucleotide::Cytosine),
            'G' | 'g' => Some(Nucleotide::Guanine),
            'T' | 't' => Some(Nucleotide::Thymine),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Codon {
    pub first: Nucleotide,
    pub second: Nucleotide,
    pub third: Nucleotide,
}

impl Codon {
    pub fn from_nucleotide_vec(nv: Vec<Nucleotide>) -> Codon {
        Self {
            first: nv[0],
            second: nv[1],
            third: nv[2],
        }
    }

    pub fn to_nucleotide_vec(&self) -> Vec<Nucleotide> {
        vec![self.first, self.second, self.third]
    }

    pub fn to_string(&self) -> String {
        self.to_nucleotide_vec()
            .iter()
            .map(Nucleotide::to_letter)
            .collect()
    }
}

pub struct Gene {
    pub codons: Vec<Codon>,
}

impl Gene {
    pub fn from_codon_vec(cv: Vec<Codon>) -> Gene {
        Self { codons: cv }
    }

    pub fn linear_contains(&self, key: Codon) -> bool {
        println!(
            "Performing linear search for presence of `{}` in gene",
            key.to_string()
        );

        for &codon in self.codons.iter() {
            if codon == key {
                return true;
            }
        }
        false
    }

    pub fn binary_contains(&self, key: Codon) -> bool {
        println!(
            "Performing binary search for presence of `{}` in gene",
            key.to_string()
        );

        let mut sorted_codons = self.codons.clone();

        sorted_codons.sort();

        let mut low = 0;
        let mut high = sorted_codons.len() - 1;

        while low <= high {
            let middle = (low + high) / 2;

            if let Some(codon) = sorted_codons.get(middle) {
                use std::cmp::Ordering::*;

                match codon.cmp(&key) {
                    Less => {
                        low = middle + 1;
                    }
                    Greater => {
                        high = middle - 1;
                    }
                    Equal => {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}

/// Parse a nucleotide letter, which is A, C, G, or T, including lowercase variants.
pub fn parse_nucleotide_letter<'a>(input: &'a str) -> IResult<&'a str, char> {
    context("nucleotide letter", one_of("AaCcGgTt"))(input)
}

/// Parse a nucleotide, combining `parse_nucleotide_letter` and `Nucleotide::from_letter`.
pub fn parse_nucleotide<'a>(input: &'a str) -> IResult<&'a str, Nucleotide> {
    map_opt(parse_nucleotide_letter, |token| {
        Nucleotide::from_letter(token)
    })(input)
}

/// Parse a codon,
pub fn parse_codon<'a>(input: &'a str) -> IResult<&'a str, Codon> {
    map(count(parse_nucleotide, 3), Codon::from_nucleotide_vec)(input)
}

/// Parse a gene, as one or more nucleotides. This is the entry parser.
pub fn parse_gene<'a>(input: &'a str) -> IResult<&'a str, Gene> {
    map(many1(parse_codon), Gene::from_codon_vec)(input)
}

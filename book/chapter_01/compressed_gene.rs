/// # Compressed Genes, featuring Nom!
/// 
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/CompressedGene.java
/// 

use bitvec::prelude::*;

use nom::{
    IResult,
    combinator::*,
    multi::*,
    error::VerboseError,
    character::complete::one_of,
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Nucleotide {
    Adenine,
    Cytosine,
    Guanine,
    Thymine,
}

impl Nucleotide {
    fn to_gene(nucleotides: Vec<Nucleotide>) -> Gene {
        Gene(nucleotides)
    }

    fn to_bits(&self) -> &BitSlice<u8> {
        match &self {
            Nucleotide::Adenine => 0b00.view_bits::<Lsb0>(),
            Nucleotide::Cytosine => 0b01.view_bits::<Lsb0>(),
            Nucleotide::Guanine => 0b10.view_bits::<Lsb0>(),
            Nucleotide::Thymine => 0b11.view_bits::<Lsb0>(),
        }
    }
}

#[derive(Clone, Debug)]
struct Gene(Vec<Nucleotide>);

impl Gene {

}

fn parse_nucleotide<'a>(input: &'a str) -> IResult<&'a str, Nucleotide, VerboseError<&'a str>> {
    let (input, token) = one_of("AaCcGgTt")(input)?;

    Ok((
        input,
        match token {
            'A' | 'a' => Nucleotide::Adenine,
            'C' | 'c' => Nucleotide::Cytosine,
            'G' | 'g' => Nucleotide::Guanine,
            'T' | 't' => Nucleotide::Thymine,
            _ => unreachable!(),
        }
    ))
}

fn parse_gene<'a>(input: &'a str) -> IResult<&'a str, Gene, VerboseError<&'a str>> {
    map(
        many1(parse_nucleotide),
        Nucleotide::to_gene
    )(input)
}

fn main() {
    if let Ok(result) = parse_gene("AGCTTGCAACGTCAGCA") {
        let unconsumed = result.0;
        let gene: Gene = result.1;

        println!("Unconsumed input: `{:?}`", unconsumed);
        println!("Result: `{:?}`", gene);

        let nucleotides: Vec<Nucleotide> = gene.0;

        for n in nucleotides {
            println!("{:?}", n.to_bits());
        }
    }
}

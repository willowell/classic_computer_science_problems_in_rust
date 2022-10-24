/// # Compressed Genes, featuring Nom!
///
/// In this example, we compress a gene (meaning a string of arbitrary length including A, C, G, and T)
/// into a vector of bits, where each nucleotide corresponds to a 2-bit number.
///
/// Unlike the Java and Swift implementations, this Rust implementation uses a `Nucleotide` enum to internally represent
/// the gene and uses Nom to parse the string into a vector of nucleotides.
///
/// This way, we can parse the string right into a `Vec<Nucleotide>` and prevent unexpected characters.
/// For instance,
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/CompressedGene.java
///
use std::convert::identity;

use classic_computer_science_problems::gene::*;

use bitvec::prelude::*;

fn main() {
    let test_gene =
        "TAGGGATTAACCGTTATATATATATAGCCATGGATCGATTATATAGGGATTAACCGTTATATATATATAGCCATGGATCGATTATA";

    let result = parse_gene(test_gene);

    match result {
        Ok(result) => {
            let unconsumed = result.0;
            let gene = result.1;

            println!(
                "Unconsumed input: `{:?}`",
                if unconsumed.is_empty() {
                    "<empty>"
                } else {
                    unconsumed
                }
            );
            println!("Result: `{:?}`", gene.codons);

            let original_gene_as_nucleotide_vec: Vec<Nucleotide> =
                gene.codons
                    .clone()
                    .iter()
                    .flat_map(Codon::to_nucleotide_vec)
                    .collect();

            let compressed_gene: Vec<BitVec> =
                original_gene_as_nucleotide_vec
                    .iter()
                    .map(Nucleotide::to_bits)
                    .collect();

            let decompressed_gene: Vec<Nucleotide> = compressed_gene
                .clone()
                .iter()
                .map(|bv| Nucleotide::from_bits(bv.clone()))
                .filter_map(identity)
                .collect();

            let short_gene: String =
                original_gene_as_nucleotide_vec
                    .iter()
                    .map(Nucleotide::to_letter)
                    .collect();

            let short_decompressed_gene: String = decompressed_gene
                .clone()
                .iter()
                .map(Nucleotide::to_letter)
                .collect();

            println!("Original gene:     `{}`", test_gene);
            println!("Parsed gene:       `{}`", short_gene);
            // Uncomment this to print out the vector of bitvecs representing the compressed gene.
            // println!("Compressed gene: `{:?}`", compressed_gene);
            println!("Decompressed gene: `{}`", short_decompressed_gene);
            println!(
                "Did the compression maintain the same data? {}",
                if original_gene_as_nucleotide_vec == decompressed_gene {
                    "yes"
                } else {
                    "no"
                }
            );
        }
        Err(err) => {
            println!("There was an error parsing the test gene.");
            println!("{}", err);
        }
    }
}

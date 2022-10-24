/// # DNA Search
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter2/Gene.java

use classic_computer_science_problems::gene::*;

fn main() {
    let test_gene = "ACGTGGCTCTCTAACGTACGTACGTACGGGGTTTATATATACCCTAGGACTCCCTTT";

    println!(
        "There are {} nucleotides in the gene, which {} divisible by 3.{}",
        test_gene.len(),
        if test_gene.len() % 3 == 0 {
            "is"
        } else {
            "is not"
        },
        if test_gene.len() % 3 == 0 {
            format!(" ({} times)", test_gene.len() / 3)
        } else {
            String::new()
        },
    );

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

            let short_gene: String = gene
                .codons
                .clone()
                .iter()
                .map(Codon::to_nucleotide_vec)
                .flatten()
                .collect::<Vec<Nucleotide>>()
                .iter()
                .map(Nucleotide::to_letter)
                .collect();

            println!("Original gene:     `{}`", test_gene);
            println!("Parsed gene:       `{}`", short_gene);

            let acg = parse_codon("ACG");
            let gat = parse_codon("GAT");

            if let Ok((_, acg)) = acg {
                let does_gene_contain_acg = gene.linear_contains(acg);

                println!("Does the gene contain `ACG`? {}", does_gene_contain_acg);

                let does_gene_contain_acg = gene.binary_contains(acg);

                println!("Does the gene contain `ACG`? {}", does_gene_contain_acg);
            }

            if let Ok((_, gat)) = gat {
                let does_gene_contain_gat = gene.linear_contains(gat);

                println!("How about `GAT`? {}", does_gene_contain_gat);

                let does_gene_contain_gat = gene.binary_contains(gat);

                println!("How about `GAT`? {}", does_gene_contain_gat);
            }
        }
        Err(err) => {
            println!("There was an error parsing the test gene.");
            println!("{}", err);
        }
    }
}

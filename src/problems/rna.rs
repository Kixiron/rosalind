use crate::dna::{Dna, Rna};

pub fn run(dna: &[Dna]) {
    let rna = Rna::from_dna_slice(dna);
    println!("RNA: {}", Rna::to_string(&rna));
}

#[test]
fn rna() {
    let input = Dna::from_str("GATGGAACTTGACTACGTAAATT");
    let expected = "GAUGGAACUUGACUACGUAAAUU";
    assert_eq!(Rna::to_string(&Rna::from_dna_slice(&input)), expected);
}

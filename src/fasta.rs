use crate::dna::Dna;
use std::collections::HashMap;

pub fn parse_fasta(input: &str) -> HashMap<String, Vec<Dna>> {
    let mut data = HashMap::new();

    for dataset in input.split('>') {
        if dataset.is_empty() {
            continue;
        }

        let (name, dna) = dataset.split_once('\n').unwrap();
        let name = name.trim().to_string();

        let mut dna = dna.trim().to_string();
        dna.retain(|c| !c.is_whitespace());
        let dna = Dna::from_string(dna).unwrap();

        data.insert(name, dna);
    }

    data
}

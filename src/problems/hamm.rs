use crate::dna::Dna;

pub fn run(first: &[Dna], second: &[Dna]) {
    let distance = hamming_distance(first, second);
    println!("HAMM: {distance}");
}

fn hamming_distance(first: &[Dna], second: &[Dna]) -> usize {
    assert_eq!(first.len(), second.len());
    first.iter().zip(second).filter(|(a, b)| a != b).count()
}

#[test]
fn hamm() {
    let first = Dna::from_str("GAGCCTACTAACGGGAT");
    let second = Dna::from_str("CATCGTAATGACGGCCT");
    assert_eq!(hamming_distance(&first, &second), 7);
}

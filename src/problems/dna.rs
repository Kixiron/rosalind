use crate::dna::Dna;

pub fn run(dna: &[Dna]) {
    #[allow(non_snake_case)]
    let [A, C, G, T] = count_nucleotides(dna);
    println!("DNA: {A} {C} {G} {T}");
}

fn count_nucleotides(dna: &[Dna]) -> [usize; 4] {
    let mut counts = [0; 4];
    for &molecule in dna {
        counts[molecule as usize] += 1;
    }

    counts
}

#[test]
fn dna() {
    let input =
        Dna::from_str("AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC");
    let expected = [20, 12, 17, 21];
    assert_eq!(count_nucleotides(&input), expected);
}

use crate::dna::Dna;

pub fn run(dna: Vec<Dna>) {
    let compliment = reverse_compliment(dna);
    println!("REVC: {}", Dna::to_string(&compliment));
}

fn reverse_compliment(mut dna: Vec<Dna>) -> Vec<Dna> {
    dna.reverse();
    for molecule in &mut dna {
        *molecule = molecule.reverse_compliment();
    }

    dna
}

#[test]
fn revc() {
    let input = Dna::from_str("AAAACCCGGT");
    let expected = Dna::from_str("ACCGGGTTTT");
    assert_eq!(reverse_compliment(input), expected);
}

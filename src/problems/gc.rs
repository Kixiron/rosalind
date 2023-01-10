use crate::dna::Dna;
use std::collections::HashMap;

pub fn run(data: &HashMap<String, Vec<Dna>>) {
    let (dataset_name, gc) = gc_count(data);
    println!("{dataset_name}\n{gc:.06}");
}

fn gc_count(data: &HashMap<String, Vec<Dna>>) -> (&str, f64) {
    let mut highest = None;

    for (dataset, data) in data {
        let gc = data
            .iter()
            .filter(|dna| matches!(dna, Dna::G | Dna::C))
            .count();
        let total = data.len();

        let gc = (gc as f64 / total as f64) * 100.0;

        match highest {
            Some((_, count)) => {
                if gc > count {
                    highest = Some((&**dataset, gc));
                }
            }

            None => highest = Some((&**dataset, gc)),
        }
    }

    highest.unwrap()
}

#[test]
fn gc() {
    let input = crate::fasta::parse_fasta(
        ">Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT",
    );
    let expected = ("Rosalind_0808", "60.919540");

    let (dataset, gc) = gc_count(&input);
    assert_eq!((dataset, &*format!("{gc:.06}")), expected);
}

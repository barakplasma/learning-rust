use std::thread;
use std::sync::mpsc::channel;

fn main() {
    println!("Hello, world!");
    dna_strand("AAAA");
}

fn dna_strand(dna: &str) -> String {
    let mut complements: Vec<(u32, &str)> = vec![];
    let dna_len = dna.len();
    let mut order: u32 = 0;
    let (tx, rx) = channel();
    for c in dna.chars() {
        let tx = tx.clone();
        thread::spawn(move|| {
            let cc = match c {
                'A' => "T",
                'T' => "A",
                'G' => "C",
                'C' => "G",
                _ => "X"
            };
            let tuple = (order, cc);
            tx.send(tuple).unwrap();
        });
        order+=1;
    }
    for _ in 0..dna_len {
        let tup = rx.recv().unwrap();
        complements.push(tup);
    }
    complements.sort_by(|a, b| a.0.cmp(&b.0));
    complements.into_iter().map(|c| c.1).collect::<Vec<&str>>().join("")
}

#[cfg(test)]
mod tests {
    use super::dna_strand;

    #[test]
    fn returns_expected() {
      assert_eq!(dna_strand("AAAA"),"TTTT");
      assert_eq!(dna_strand("ATTGC"),"TAACG");
      assert_eq!(dna_strand("GTAT"),"CATA");
    }
}
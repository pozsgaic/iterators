//  Demonstrate how to use iterators with IntoIter
//  This example only works on type of u32, not a reference.
//  The counts object cannot be reused

#[derive(Debug, Clone)]
struct Counts(Vec<u32>);

impl Counts {
    fn new() -> Counts {
        Counts(Vec::new())
    }

    fn add(&mut self, elem: u32) {
        self.0.push(elem);
    }
 
    fn total(&mut self) -> u32 {
        let mut count = 0;
        for elem in &self.0 {
            count += elem;
        }
        count
    }
}

impl IntoIterator for Counts {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn main() {
    println!("Hello, into_iter");
    let mut counts = Counts::new();
    counts.add(40);
    counts.add(437);
    counts.add(199);

    let mut counts2 = counts.clone();
    println!("total = {}", counts.total());
    for c in counts.into_iter() {
        println!("count = {}", c);
    }

    for c in counts2.into_iter() {
        println!("count = {}", c);
    }
}

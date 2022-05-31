use std::fmt;

//  Chain two iterators together and iterate over every
//  other element in the chain.

//  Create a counter class to handle out counting.
struct Counter(u32);

impl Counter {
    fn new() -> Counter {
        Counter(0)
    }

    fn increment(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
  
    fn increment_by(&mut self, by: u32) -> u32 {
        self.0 += by;
        self.0
    }

    fn reset(&mut self) -> u32 {
        self.0 = 0;
        self.0
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("Hello, world of iterators");
    let a1 = vec![11, 12, 13];
    let a2 = vec![14, 15, 16];

    let mut iter_chain = a1.iter().chain(a2.iter());

    //	Iterate over every other element in the chain.
    count.reset();
    for elem in iter_chain.step_by(2) {
        println!("Item {}:{}", count, elem);
        count.increment_by(2);
    }
}

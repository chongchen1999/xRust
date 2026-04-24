struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Only `next` is required; all adapter methods come for free.
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn run() {
    // Yields 1, 2, 3, 4, 5
    let values: Vec<u32> = Counter::new().collect();
    assert_eq!(values, vec![1, 2, 3, 4, 5]);

    // Standard adapters work immediately after implementing Iterator
    let sum: u32 = Counter::new().sum();
    assert_eq!(sum, 15);

    // Pair each count with its successor using zip
    let pairs: Vec<(u32, u32)> = Counter::new().zip(Counter::new().skip(1)).collect();
    assert_eq!(pairs, vec![(1, 2), (2, 3), (3, 4), (4, 5)]);

    println!("custom_iterator: ok");
}

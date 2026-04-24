pub fn run() {
    let v = vec![1, 2, 3, 4, 5, 6];

    // map: transform each element
    let squares: Vec<i32> = v.iter().map(|x| x * x).collect();
    assert_eq!(squares, vec![1, 4, 9, 16, 25, 36]);

    // filter: keep elements matching the predicate
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    assert_eq!(evens, vec![&2, &4, &6]);

    // enumerate: attach a 0-based index to each element
    let indexed: Vec<(usize, &i32)> = v.iter().enumerate().collect();
    assert_eq!(indexed[0], (0, &1));
    assert_eq!(indexed[2], (2, &3));

    // zip: pair elements from two iterators
    let a = vec![1, 2, 3];
    let b = vec!["one", "two", "three"];
    let zipped: Vec<(i32, &&str)> = a.into_iter().zip(b.iter()).collect();
    assert_eq!(zipped[1], (2, &"two"));

    // chain: concatenate two iterators into one
    let first = vec![1, 2];
    let second = vec![3, 4];
    let chained: Vec<i32> = first.into_iter().chain(second.into_iter()).collect();
    assert_eq!(chained, vec![1, 2, 3, 4]);

    // flat_map: map then flatten one level
    let words = vec!["hello world", "foo bar"];
    let tokens: Vec<&str> = words.iter().flat_map(|s| s.split_whitespace()).collect();
    assert_eq!(tokens, vec!["hello", "world", "foo", "bar"]);

    // take / skip: slice a sequence by count
    let naturals: Vec<i32> = (1..=10).skip(2).take(4).collect();
    assert_eq!(naturals, vec![3, 4, 5, 6]);

    println!("common_adapters: ok");
}

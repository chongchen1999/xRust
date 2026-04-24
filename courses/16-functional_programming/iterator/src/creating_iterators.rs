pub fn run() {
    let v = vec![1, 2, 3];

    // iter() borrows: yields &T
    let sum: i32 = v.iter().map(|x| x).sum();
    assert_eq!(sum, 6);

    // iter_mut() borrows mutably: yields &mut T
    let mut v2 = vec![1, 2, 3];
    v2.iter_mut().for_each(|x| *x *= 2);
    assert_eq!(v2, vec![2, 4, 6]);

    // into_iter() consumes the collection: yields T
    let v3 = vec![1, 2, 3];
    let doubled: Vec<i32> = v3.into_iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6]);

    println!("creating_iterators: ok");
}

pub fn run() {
    let v = vec![1, 2, 3, 4, 5];

    // collect: gather elements into a collection
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    assert_eq!(doubled, vec![2, 4, 6, 8, 10]);

    // sum / product
    let total: i32 = v.iter().sum();
    assert_eq!(total, 15);

    // count
    let n = v.iter().filter(|&&x| x > 2).count();
    assert_eq!(n, 3);

    // fold: general left reduction
    let product = v.iter().fold(1, |acc, x| acc * x);
    assert_eq!(product, 120);

    // any / all: short-circuit predicates
    assert!(v.iter().any(|&x| x > 4));
    assert!(v.iter().all(|&x| x > 0));

    // find: first matching element
    let first_even = v.iter().find(|&&x| x % 2 == 0);
    assert_eq!(first_even, Some(&2));

    // position: index of first match
    let pos = v.iter().position(|&x| x == 3);
    assert_eq!(pos, Some(2));

    println!("consuming_adapters: ok");
}

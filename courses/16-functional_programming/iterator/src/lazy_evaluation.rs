pub fn run() {
    let v = vec![1, 2, 3];

    // Nothing prints here — the chain is not yet consumed.
    let _lazy = v.iter().map(|x| x * 10);

    // Now we consume it — only here does the closure fire.
    let result: Vec<i32> = v.iter().map(|x| x * 10).collect();
    assert_eq!(result, vec![10, 20, 30]);

    println!("lazy_evaluation: ok");
}

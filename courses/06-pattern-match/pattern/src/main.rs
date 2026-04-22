fn main() {
    let some_option: Option<i32> = Some(5);

    match some_option {
      inner @ Some(0) => println!("got Some(0): {inner:?}"),
      Some(n @ 1..=9) => println!("single digit: {n}"),
      _ => println!("other"),
    }
}

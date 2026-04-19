fn main() {
    match some_option {
      inner @ Some(0) => println!("got Some(0): {inner:?}"),
      Some(n @ 1..=9) => println!("single digit: {n}"),
      _ => println!("other"),
    }
}

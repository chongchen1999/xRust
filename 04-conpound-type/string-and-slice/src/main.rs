fn main() {
    let mut s = String::from("hello world");
    let _s = "Hello";

    let word = first_word(&s);
    println!("the first word is: {}", word);
    s.clear(); // error!
}
fn first_word(s: &String) -> &str {
    &s[..1]
}
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;

fn main() {
    // === 1. format! — returns String, no I/O ===
    let s = format!("x = {}, y = {}", 10, 20);
    println!("format!: {s}");

    // === 2. print! / println! — stdout, no newline / with newline ===
    print!("no newline ");
    println!("=> with newline");

    // === 3. eprint! / eprintln! — stderr ===
    eprintln!("This goes to stderr");

    // === 4. write! / writeln! — write to any impl Write ===

    // 4a. fmt::Write — write to String
    let mut buf = String::new();
    writeln!(buf, "line 1: {}", 42).unwrap();
    writeln!(buf, "line 2: {:.2}", 3.14).unwrap();
    println!("fmt::Write into String:\n{buf}");

    // 4b. io::Write — write to Vec<u8> (byte buffer)
    let mut bytes: Vec<u8> = Vec::new();
    writeln!(bytes, "hello bytes: {}", 100).unwrap();
    println!("io::Write into Vec<u8>: {}", String::from_utf8(bytes).unwrap());

    // 4c. io::Write — write to stderr handle
    let mut stderr = std::io::stderr();
    writeln!(stderr, "write! to stderr handle: {}", "direct").unwrap();

    // === 5. format_args! — zero-allocation formatting ===
    let args = format_args!("no alloc: {} + {} = {}", 1, 2, 3);
    println!("{}", args);

    // === 6. Practical: building a table ===
    println!("\n{:-^40}", " Table ");
    println!("{:<10} {:>10} {:>10}", "Name", "Score", "Grade");
    println!("{:-<10} {:->10} {:->10}", "", "", "");
    let data = [("Alice", 95, "A"), ("Bob", 82, "B"), ("Carol", 78, "C+")];
    for (name, score, grade) in &data {
        println!("{:<10} {:>10} {:>10}", name, score, grade);
    }
}

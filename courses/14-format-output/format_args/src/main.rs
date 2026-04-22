fn main() {
    // === 1. Positional arguments ===
    println!("{0} loves {1}, and {1} loves {0}", "Alice", "Bob");

    // === 2. Named arguments ===
    println!("{name} is {age} years old", name = "Tom", age = 30);

    // Since Rust 1.58, capture variables directly
    let name = "Jerry";
    let age = 25;
    println!("{name} is {age} years old");

    // === 3. Padding & Alignment ===
    // Right-align (default for numbers)
    println!("[{:>10}]", "right");
    // Left-align
    println!("[{:<10}]", "left");
    // Center
    println!("[{:^10}]", "center");
    // Custom fill char + alignment
    println!("[{:*>10}]", "right");
    println!("[{:*<10}]", "left");
    println!("[{:*^10}]", "center");

    // === 4. Width from argument ===
    let width = 15;
    println!("[{:>width$}]", "dynamic");
    println!("[{:>1$}]", "dynamic", width);

    // === 5. Number formatting ===
    let x = 42;
    println!("Decimal:  {}", x);
    println!("Binary:   {:b}", x);
    println!("Octal:    {:o}", x);
    println!("Hex low:  {:x}", x);
    println!("Hex up:   {:X}", x);
    // With prefix: 0b, 0o, 0x
    println!("Binary:   {:#b}", x);
    println!("Octal:    {:#o}", x);
    println!("Hex:      {:#x}", x);
    // Zero padding
    println!("Zero pad: {:08}", x);
    println!("Zero hex: {:08x}", x);

    // === 6. Sign display ===
    let pos = 42;
    let neg = -42;
    println!("Default:  {} / {}", pos, neg);
    println!("Always:   {:+} / {:+}", pos, neg);

    // === 7. Floating point precision ===
    let pi = std::f64::consts::PI;
    println!("Default:  {}", pi);
    println!("2 digits: {:.2}", pi);
    println!("8 digits: {:.8}", pi);
    // Precision from argument
    let prec = 4;
    println!("{prec} digits: {:.prec$}", pi);

    // === 8. String truncation via precision ===
    println!("Truncate: {:.5}", "Hello, World!");

    // === 9. Scientific notation ===
    let big = 1234567.89;
    println!("Lower e:  {:e}", big);
    println!("Upper E:  {:E}", big);

    // === 10. Debug vs Display ===
    let v = vec![1, 2, 3];
    println!("Debug:    {:?}", v);
    println!("Pretty:   {:#?}", v);

    // === 11. Pointer address ===
    let val = 42;
    let ptr = &val;
    println!("Pointer:  {:p}", ptr);

    // === 12. Escaping braces ===
    println!("Literal braces: {{}} => {}", 42);

    // === 13. Combining specifiers ===
    // Fill '*', center, width 20, 3 decimal places
    println!("[{:*^20.3}]", pi);
    // Fill '0', right, width 12, hex with prefix
    println!("[{:#012x}]", 255);
}

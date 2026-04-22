use std::fmt;

// --- Display trait: user-facing output with {} ---

struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Display for a wrapper around Vec (newtype pattern)
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", i, v)?;
        }
        write!(f, "]")
    }
}

// Display automatically provides ToString
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

// Implement multiple format traits for one type
struct Matrix([[f64; 2]; 2]);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {:.2}  {:.2} ]\n[ {:.2}  {:.2} ]",
            self.0[0][0], self.0[0][1],
            self.0[1][0], self.0[1][1])
    }
}

impl fmt::Octal for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix(octal: {:o}, {:o}, {:o}, {:o})",
            self.0[0][0] as i64, self.0[0][1] as i64,
            self.0[1][0] as i64, self.0[1][1] as i64)
    }
}

fn main() {
    // --- Basic Display ---
    let p = Point { x: 3.14, y: 2.72 };
    println!("Display: {}", p);
    // Display provides to_string() for free
    println!("to_string(): {}", p.to_string());

    // --- Display for collections (newtype pattern) ---
    let list = List(vec![10, 20, 30]);
    println!("{}", list);

    // --- Color hex formatting ---
    let colors = [
        Color { r: 255, g: 0, b: 0 },
        Color { r: 0, g: 128, b: 255 },
        Color { r: 0, g: 0, b: 0 },
    ];
    for c in &colors {
        println!("{}", c);
    }

    // --- Multiple traits for one type ---
    let m = Matrix([[1.0, 2.5], [3.7, 4.0]]);
    println!("Display:\n{}", m);
    println!("Octal: {:o}", m);
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    calculation: T,
    value: Option<U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Clone,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, x: U) -> U {
        if let Some(v) = self.value.as_ref() {
            println!("Cache hit!");
            return v.clone();
        }
        println!("Cache miss!");
        let v = (self.calculation)(x);
        self.value = Some(v.clone());
        v
    }
}

fn main() {
    let mut cache_u32 = Cacher::new(|x: String| x.repeat(2));
    println!("{}", cache_u32.value(String::from("Wow")));
    println!("{}", cache_u32.value(String::from("Wow")));
}

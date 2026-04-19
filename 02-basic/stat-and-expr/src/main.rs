fn main() {
   let v = {let _x = 3;};

   assert!(v == ());
}
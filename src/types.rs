pub fn run() {
  
  let _x = 1; // Default is "i32"
  let _y = 2.5; // Default is "f64"
  let _z: i64 = 112233445566; // Explicitly typed
  
  println!("Max i32: {}", std::i32::MAX); // Find max size of i32
  println!("Max i64: {}", std::i64::MAX); // Find max size of i64
  
  let is_active: bool = true; // Boolean
  let is_greater: bool = 10 > 5; // Conditional expression
  
  let a1 = 'a'; // char
  let face = '\u{1F600}'; // Unicode is also a char
  
  println!("{:?}", (_x, _y, _z, is_active, is_greater, a1, face));
  
}
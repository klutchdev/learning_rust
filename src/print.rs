pub fn run() {
  // Print to console
  println!("Hello from print.rs!");
  
  // Simple args
  println!("{} is from {}", "Klutch", "Holyoke");
  
  // Positional args
  println!("{0} and {1} are my favorites, followed by {2}", "JS","TS","Rust");
  
  // Named args
  println!("{me} loves to write code in {currentLang}", me = "Klutch", currentLang = "Rust");
  
  // Placeholder traits
  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10 , 10, 10 );
  
  // Debug placeholder traits
  println!("{:?}", (12, true, "Wadddap!"));
  
  // Basic maath
  println!("10 + 10 = {}", 10 + 10);
  
  
  
  
}
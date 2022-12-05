pub fn run() {
  println!("Basic printing");

  println!("In this print, I can import a {} and not only {} but many.", "variable", 1);

  println!("I can show multiple {0} by just putting {1}, and even I can repeat {0}", "variables", "{}");

  println!("{name} is a defined variable in this print function, it's called {vartype}", name = "RustVar", vartype = "Name Arguments");

  println!("Binary: {:b} Hex: {:x} Octal: {:o}", 12, 12, 12);

  println!("Some basic math operation: 2 + 2 = {}", 2 + 2);
}

fn main() {
    // shadowing allows the type and value of a variable to be mutated in a scope
    let spaces = "   ";
    println!("spaces: {spaces}");
    let spaces = spaces.len();
    println!("spaces: {spaces}");
}

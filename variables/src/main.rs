fn main() {
    // Immutable variable
    let x = 5;
    // x = 6; this line would cause a compile-time (error) because x is immutable
    println!("The value of x: {x}");
    // Mutable variable
    let mut y = 5;
    println!("The value of y: {y}");
    y = 6; // change the value of y
    println!("The value of y: {y}");
    // Constant variable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
    // Shadowing: redeclaring a variable with the same name
    let z = 5;
    let z = z + 1; // z is now 6
    // inner scope shadowing
    {
        let z = z * 2; // z is now 12 in this inner scope
        println!("The value of z in the inner scope: {z}");
    }
    println!("The value of z in the outer scope: {z}");
    // Shadowing to change type
    let spaces = "   "; // spaces is of type &str
    let spaces = spaces.len(); // spaces is now of type usize
    println!("The number of spaces: {spaces}");
}

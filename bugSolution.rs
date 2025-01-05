fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x; // y is a mutable reference to x
        *y += 1;
    }
    { // Create a second scope
        let z = &mut x; // z is a mutable reference to x
        *z += 1;
    }
    println!("x: {}", x); // Output: x: 7
} 
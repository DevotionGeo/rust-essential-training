fn main() {
    let mut x: u8 = 255;
    x = x + 1;
    println!("x is {}", x);
}

// let x: u8 = -10 fails because unsigned integers
// can't hold negative values.

// let x: u8 = 1000 failes because of the overflow

// program panics when adding 1 to the maximum value of u8
// i.e., adding 1 to 255.

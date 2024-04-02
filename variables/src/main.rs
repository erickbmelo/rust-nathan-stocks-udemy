fn main() {
    // LET //

    let variable = 2; // auto type int
    let typedvar: i32 = 4; // signed 32bit integer
    let (bunnies, carrots) = (8, 50); // multiple variables at once
    
    // variable = 3; // error, variable is immutable by default
    let mut mutablevariable = 32;
    mutablevariable = 2;


    // CONST // (Even immutabler, and fast)

    const WARP_FACTOR: f64 = 9.9; // (data type is required) 
}

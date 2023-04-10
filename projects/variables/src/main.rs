fn main() {
    // Variable Mutability
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Variable Shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces is {spaces} spaces long.");

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    /*
      We can "Destructure" the tuple placing them into three separate variables
      let (x, y, z) = tup;
      println!("{tup}") Does not work because 'tup' is considered a single compound element
      println!("{x} {y} {z}");
    */
    /*
      With the format of tup.0 we can access a tuple element at an index.
      let five_hundred = tup.0;
      println!("{five_hundred}");
    */

    /* A Tuple with NO VALUES is a UNIT ( The value and type for this is written as () ) */

    // Arrays - Useful when you want data on the STACK and not the HEAP.
    // Arrays in Rust can not grow, or shrink. They're FIXED.
    // If you wanted something that can grow, or shrink use a VECTOR
    let arr = [ 1, 2, 3, 4, 5 ];
    let a: [i32; 5] = [ 1, 2, 3, 4, 5 ];
    let b: [3; 5];
}

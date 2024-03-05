/*  Example 3: Memory Management

    Compile ->  rustc ex_3.rs
    Run ->      ./ex_3
*/

fn main() {
    
    {
        let string_1 = String::from("Stay Froggy!");  // In scope
    }

    // Out of scope and AUTOMAGICALLY deleted

}